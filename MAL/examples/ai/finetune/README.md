# MAL LLM Fine-tuning Pipeline
## Overview
Fine-tune LLMs to generate Mathematical Arabic Language (MAL) code from natural language descriptions.
## Dataset v2.0 Statistics
- basic_arithmetic: 20
- variables: 15
- functions: 15
- equality: 10
- match: 10
- mixed: 30
- bool_ops: 10
- unary_advanced: 10
- advanced_builtins: 8
- nested_calls: 10
- complex_arithmetic: 15
- comparisons_advanced: 10
- match_wildcards_advanced: 10
- forall_exists: 5
- boolean_logic: 10
- mixed_advanced: 15
- **Total: 203 (100% verified)**
## Quick Start
### Installation
    pip install transformers peft accelerate datasets torch
### Training (LoRA)
    python3 finetune_mal.py \
      --base-model Qwen/Qwen2.5-0.5B-Instruct \
      --epochs 3 \
      --batch-size 2 \
      --lr 2e-4 \
      --lora-r 8
### Evaluation
    python3 evaluate_mal.py \
      --model-dir ./mal-lora \
      --eval-data dataset_eval.jsonl
## Supported Features
### Operators
- Arithmetic: +, -, ·, /, %, ^
- Comparisons: ==, <, ≤, >, ≥, ≠
- Boolean: ∧, ∨, ¬, true, false
### Functions
- Math: sqrt, abs, floor, ceil, round, min, max
- Power: power(base, exp), exp(x)
- Lists: list_head, list_tail, list_append
### Control Flow
- Variables: ≔ (assign/reassign)
- Match: match expr with | pattern => body | _ => default
- Quantifiers: ∀ x ∈ set : body, ∃ x ∈ set : body
## Dataset Structure
    MAL/examples/ai/
    ├── dataset/
    │   ├── dataset.jsonl          # All 203 examples
    │   ├── metadata.json          # Statistics
    │   └── <category>/*.mal       # Individual files
    └── finetune/
        ├── dataset_train.jsonl    # 182 examples (90%)
        ├── dataset_eval.jsonl     # 21 examples (10%)
        ├── finetune_mal.py        # LoRA training
        ├── evaluate_mal.py        # Compile-and-run evaluation
        └── README.md              # This file
## Evaluation Methodology
Each generated MAL snippet is:
1. Written to a temporary .mal file
2. Compiled with malc-native to ELF binary
3. Executed and exit code compared to expected
4. Accuracy = (correct exit codes) / (total examples)
﴿وقل رب زدني علماً﴾
