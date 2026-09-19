#!/usr/bin/env bash
set -u
set -o pipefail
MODEL="${QWEN_MODEL:-qwen2.5-coder:1.5b-instruct}"
OUT="${1:-/tmp/qwen-incremental}"
mkdir -p "$OUT"
run_case() {
  local name="$1" prompt="$2"
  printf '%s\n' "$prompt" >"$OUT/$name.prompt"
  python3 - "$MODEL" "$prompt" >"$OUT/$name.json" <<'PY'
import json,sys
print(json.dumps({"model":sys.argv[1],"stream":False,"keep_alive":"5m","options":{"temperature":0.1,"num_ctx":2048,"num_predict":128,"num_thread":2,"top_p":0.9},"messages":[{"role":"user","content":sys.argv[2]}]},ensure_ascii=False))
PY
  timeout 90 curl -sS --max-time 85 http://127.0.0.1:11434/api/chat -H 'Content-Type: application/json' -d @"$OUT/$name.json" >"$OUT/$name.response" 2>"$OUT/$name.stderr"
  local c=$?
  python3 - "$OUT/$name.response" "$name" "$c" <<'PY'
import json,sys
p,name,c=sys.argv[1:]
try:
 d=json.load(open(p,encoding='utf-8')); text=d.get('message',{}).get('content','')
 print(f'{name}: HTTP_OK={c=="0"} CONTENT={text!r}')
 print(f'{name}: READY_OK={"READY" in text} EXPLANATION_MARKERS={"EXPLANATION_BEGIN" in text and "EXPLANATION_END" in text} PATCH_MARKERS={"PATCH_BEGIN" in text and "PATCH_END" in text}')
 open(p+'.text','w',encoding='utf-8').write(text)
except Exception as e: print(f'{name}: PARSE_FAIL={e}')
PY
}
run_case t1_ready 'Reply with exactly READY and nothing else.'
run_case t2_protocol 'Return exactly: EXPLANATION_BEGIN\nshort\nEXPLANATION_END\nPATCH_BEGIN\nNO_PATCH\nPATCH_END'
run_case t3_patch 'Return EXPLANATION_BEGIN\nadd harmless file\nEXPLANATION_END\nPATCH_BEGIN\n*** Begin Patch\n*** Add File: /tmp/forbidden\n+x\n*** End Patch\nPATCH_END'
printf '=== DONE ===\n'
