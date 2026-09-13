#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

mode = sys.argv[1]
if mode == "request":
    root = Path(os.environ["QWEN_ROOT"])
    phase = Path(os.environ["QWEN_PHASE"]).read_text(encoding="utf-8")
    plan = Path(os.environ["QWEN_PLAN"]).read_text(encoding="utf-8")
    contract = (root / "AGENT_CONTRACT.md").read_text(encoding="utf-8")
    model = os.environ.get("QWEN_MODEL", "hermes-qwen-code:latest")
    content = (
        f"PROJECT_ROOT={root}\nPHASE_FILE:\n{phase}\n"
        f"MASTER_PLAN:\n{plan}\nCONTRACT:\n{contract}\n"
        "Return only a patch between PATCH_BEGIN and PATCH_END, or NO_PATCH with a concise reason."
    )
    print(json.dumps({"model": model, "stream": False,
        "options": {"temperature": 0.1, "num_ctx": 8192},
        "messages": [
            {"role": "system", "content": "You are a fail-closed local coding agent. Work on exactly one phase. Do not invent semantics. Return a unified git patch only between PATCH_BEGIN and PATCH_END, or NO_PATCH if blocked. Never include shell commands, secrets, or edits outside allowed project scope."},
            {"role": "user", "content": content}
        ]}, ensure_ascii=False))
elif mode == "prompt":
    root = Path(os.environ["QWEN_ROOT"])
    phase = Path(os.environ["QWEN_PHASE"]).read_text(encoding="utf-8")
    plan = Path(os.environ["QWEN_PLAN"]).read_text(encoding="utf-8")
    contract = (root / "AGENT_CONTRACT.md").read_text(encoding="utf-8")
    print("You are a fail-closed local coding agent. Work on exactly one phase. Return only a unified git patch between PATCH_BEGIN and PATCH_END, or NO_PATCH with a concise reason. Do not invent semantics.\n")
    print(f"PROJECT_ROOT={root}\nPHASE_FILE:\n{phase}\nMASTER_PLAN:\n{plan}\nCONTRACT:\n{contract}\n")
    print("Return only the patch or NO_PATCH.")
elif mode == "content":
    data = json.load(sys.stdin)
    print(data.get("message", {}).get("content", ""))
else:
    raise SystemExit("unknown mode")
