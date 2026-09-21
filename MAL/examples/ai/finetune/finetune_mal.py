#!/usr/bin/env python3
"""MAL LLM Fine-tuning with LoRA (Qwen2.5-0.5B or similar)."""
import argparse, json
from pathlib import Path
def main():
    p = argparse.ArgumentParser()
    p.add_argument("--base-model", default="Qwen/Qwen2.5-0.5B-Instruct")
    p.add_argument("--train-data", default="dataset_train.jsonl")
    p.add_argument("--eval-data", default="dataset_eval.jsonl")
    p.add_argument("--output-dir", default="./mal-lora")
    p.add_argument("--epochs", type=int, default=3)
    p.add_argument("--batch-size", type=int, default=2)
    p.add_argument("--lr", type=float, default=2e-4)
    p.add_argument("--lora-r", type=int, default=8)
    args = p.parse_args()
    try:
        from transformers import AutoModelForCausalLM, AutoTokenizer, TrainingArguments, Trainer, DataCollatorForLanguageModeling
        from peft import LoraConfig, get_peft_model, prepare_model_for_kbit_training
        from datasets import Dataset
        import torch
    except ImportError as e:
        print(f"❌ Missing: {e}\npip install transformers peft accelerate datasets torch"); return
    print(f"🚀 Loading {args.base_model}")
    tok = AutoTokenizer.from_pretrained(args.base_model, trust_remote_code=True)
    if tok.pad_token is None: tok.pad_token = tok.eos_token
    model = AutoModelForCausalLM.from_pretrained(args.base_model,
        torch_dtype=torch.float16, device_map="auto" if torch.cuda.is_available() else None)
    model = prepare_model_for_kbit_training(model)
    model = get_peft_model(model, LoraConfig(r=args.lora_r, lora_alpha=32,
        target_modules=["q_proj","v_proj"], lora_dropout=0.05, bias="none", task_type="CAUSAL_LM"))
    model.print_trainable_parameters()
    def load(path):
        with open(path, encoding="utf-8") as f: return [json.loads(l) for l in f]
    def fmt(ex):
        txt = f"### Instruction:\n{ex['prompt']}\n\n### Response:\n{ex['completion']}</s>"
        return tok(txt, truncation=True, max_length=512)
    train_ds = Dataset.from_list([fmt(e) for e in load(args.train_data)])
    eval_ds = Dataset.from_list([fmt(e) for e in load(args.eval_data)]) if Path(args.eval_data).exists() else None
    trainer = Trainer(model=model,
        args=TrainingArguments(output_dir=args.output_dir, num_train_epochs=args.epochs,
            per_device_train_batch_size=args.batch_size, learning_rate=args.lr,
            logging_steps=10, save_steps=100,
            eval_strategy="epoch" if eval_ds else "no",
            fp16=torch.cuda.is_available(), report_to="none"),
        train_dataset=train_ds, eval_dataset=eval_ds,
        data_collator=DataCollatorForLanguageModeling(tok, mlm=False))
    print("🏋️ Training..."); trainer.train()
    model.save_pretrained(args.output_dir); tok.save_pretrained(args.output_dir)
    print(f"✅ Saved to {args.output_dir}")
if __name__ == "__main__": main()
