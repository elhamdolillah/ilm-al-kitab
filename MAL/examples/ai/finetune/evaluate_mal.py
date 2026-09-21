#!/usr/bin/env python3
"""Evaluate fine-tuned MAL model."""
import argparse, json, subprocess, tempfile
from pathlib import Path
def main():
    p = argparse.ArgumentParser()
    p.add_argument("--model-dir", default="./mal-lora")
    p.add_argument("--base-model", default="Qwen/Qwen2.5-0.5B-Instruct")
    p.add_argument("--eval-data", default="dataset_eval.jsonl")
    p.add_argument("--malc", default="../../../tools/malc-native/target/release/malc-native")
    args = p.parse_args()
    try:
        from transformers import AutoModelForCausalLM, AutoTokenizer, pipeline
        from peft import PeftModel
        import torch
    except ImportError as e:
        print(f"❌ Missing: {e}"); return
    tok = AutoTokenizer.from_pretrained(args.base_model)
    base = AutoModelForCausalLM.from_pretrained(args.base_model,
        torch_dtype=torch.float16, device_map="auto" if torch.cuda.is_available() else None)
    model = PeftModel.from_pretrained(base, args.model_dir)
    pipe = pipeline("text-generation", model=model, tokenizer=tok)
    with open(args.eval_data, encoding="utf-8") as f:
        eval_data = [json.loads(l) for l in f]
    correct, total = 0, len(eval_data)
    for i, ex in enumerate(eval_data, 1):
        try:
            out = pipe(ex["prompt"], max_new_tokens=64, do_sample=False)[0]["generated_text"]
            gen = out[len(ex["prompt"]):].split("</s>")[0].strip()
            with tempfile.NamedTemporaryFile("w", suffix=".mal", delete=False) as f:
                f.write(gen); mal_p = f.name
            with tempfile.NamedTemporaryFile(suffix=".bin", delete=False) as f:
                bin_p = f.name
            cr = subprocess.run([args.malc, mal_p, "-o", bin_p], capture_output=True, timeout=10)
            if cr.returncode == 0 and Path(bin_p).exists():
                rr = subprocess.run([bin_p], capture_output=True, timeout=5)
                ok = rr.returncode == ex["expected_exit"]
                correct += ok
                print(f"[{i}/{total}] {'✅' if ok else '❌'} exit={rr.returncode} exp={ex['expected_exit']} | {gen[:40]}")
        except Exception as e:
            print(f"[{i}/{total}] ❌ {e}")
    print(f"\n📊 Accuracy: {correct}/{total} ({100*correct/total:.1f}%)")
if __name__ == "__main__": main()
