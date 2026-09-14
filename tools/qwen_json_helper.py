#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

mode = sys.argv[1]

def inputs():
    root = Path(os.environ["QWEN_ROOT"])
    phase = Path(os.environ["QWEN_PHASE"]).read_text(encoding="utf-8")[:5000]
    plan = Path(os.environ["QWEN_PLAN"]).read_text(encoding="utf-8")[:3500]
    contract = (root / "AGENT_CONTRACT.md").read_text(encoding="utf-8")[:2500]
    return root, phase, plan, contract

if mode == "request":
    root, phase, plan, contract = inputs()
    model = os.environ.get("QWEN_MODEL", "qwen2.5-coder:1.5b-instruct")
    content = (
        f"PROJECT_ROOT={root}\nPHASE_FILE:\n{phase}\n"
        f"MASTER_PLAN:\n{plan}\nCONTRACT:\n{contract}\n"
        "Return EXPLANATION_BEGIN/EXPLANATION_END, then PATCH_BEGIN/PATCH_END, or NO_PATCH with a concise reason."
    )
    print(json.dumps({"model": model, "stream": False,
        "options": {"temperature": 0.1, "num_ctx": 2048, "num_predict": 512, "num_thread": 2, "top_p": 0.9},
        "messages": [
            {"role": "system", "content": "You are a fail-closed local coding agent. Work on exactly one phase. Do not invent semantics. Return a short explanation between EXPLANATION_BEGIN and EXPLANATION_END, then a unified git patch between PATCH_BEGIN and PATCH_END, or NO_PATCH if blocked. Never include shell commands, secrets, or edits outside allowed project scope."},
            {"role": "user", "content": content}
        ]}, ensure_ascii=False))
elif mode == "prompt":
    root, phase, plan, contract = inputs()
    print("You are a fail-closed local coding agent. Work on exactly one phase. Return EXPLANATION_BEGIN/EXPLANATION_END, then a unified git patch between PATCH_BEGIN and PATCH_END, or NO_PATCH with a concise reason. Do not invent semantics.\n")
    print(f"PROJECT_ROOT={root}\nPHASE_FILE:\n{phase}\nMASTER_PLAN:\n{plan}\nCONTRACT:\n{contract}\n")
    print("Return only the patch or NO_PATCH.")
elif mode == "content":
    data = json.load(sys.stdin)
    print(data.get("message", {}).get("content", ""))
else:
    raise SystemExit("unknown mode")
