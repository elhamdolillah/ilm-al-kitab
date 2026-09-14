#!/usr/bin/env bash
set -euo pipefail

ROOT="/root/ilm-al-kitab"
MODEL="${QWEN_MODEL:-qwen2.5-coder:1.5b-instruct}"
PHASE_FILE="${1:-$ROOT/TASKS/ACTIVE/SYMBOLS_NEXT.md}"
PLAN_FILE="${QWEN_PLAN_FILE:-$ROOT/QWEN_REMAINING_PHASES.md}"
RUN_ID="$(date -u +%Y%m%d_%H%M%S)"
RUN_DIR="$ROOT/evidence/qwen-cycles/$RUN_ID"
mkdir -p "$RUN_DIR"
cd "$ROOT"

fail() { echo "STATUS: BLOCKED" | tee "$RUN_DIR/status.txt"; echo "REASON: $*" | tee -a "$RUN_DIR/status.txt"; exit 2; }
[ -f "$PHASE_FILE" ] || fail "missing phase file: $PHASE_FILE"
[ -f "$PLAN_FILE" ] || fail "missing plan file: $PLAN_FILE"
[ -x "$ROOT/tools/qwen_json_helper.py" ] || fail "missing qwen_json_helper.py"

./.qwen_autotest/run_all_tests.sh >"$RUN_DIR/baseline.stdout" 2>&1 || fail "baseline failed; see $RUN_DIR/baseline.stdout"
sha256sum "$RUN_DIR/baseline.stdout" >"$RUN_DIR/baseline.sha256"
git status --short >"$RUN_DIR/status-before.txt"
git diff --check >"$RUN_DIR/diff-before.txt"

QWEN_ROOT="$ROOT" QWEN_PHASE="$PHASE_FILE" QWEN_PLAN="$PLAN_FILE" QWEN_MODEL="$MODEL" \
  python3 "$ROOT/tools/qwen_json_helper.py" request >"$RUN_DIR/request.json"
if curl --fail --silent --show-error --max-time 60 http://127.0.0.1:11434/api/chat -H 'Content-Type: application/json' -d @"$RUN_DIR/request.json" >"$RUN_DIR/qwen.json"; then
  python3 "$ROOT/tools/qwen_json_helper.py" content <"$RUN_DIR/qwen.json" >"$RUN_DIR/qwen.txt"
else
  echo "CHAT_API_FAILED: using ollama CLI fallback" >"$RUN_DIR/qwen-api-fallback.txt"
  QWEN_ROOT="$ROOT" QWEN_PHASE="$PHASE_FILE" QWEN_PLAN="$PLAN_FILE" \
    python3 "$ROOT/tools/qwen_json_helper.py" prompt >"$RUN_DIR/prompt.txt"
  timeout 240 ollama run "$MODEL" "$(cat "$RUN_DIR/prompt.txt")" >"$RUN_DIR/qwen.txt" 2>"$RUN_DIR/ollama.stderr" || fail "both chat API and ollama CLI failed"
fi
if grep -q '^NO_PATCH' "$RUN_DIR/qwen.txt"; then
  cp "$RUN_DIR/qwen.txt" "$ROOT/TASKS/ACTIVE/PHASE-BLOCKED.md"
  echo "STATUS: BLOCKED" | tee "$RUN_DIR/status.txt"
  exit 0
fi
awk '/PATCH_BEGIN/{flag=1;next}/PATCH_END/{flag=0}flag' "$RUN_DIR/qwen.txt" >"$RUN_DIR/change.patch"
awk '/EXPLANATION_BEGIN/{flag=1;next}/EXPLANATION_END/{flag=0}flag' "$RUN_DIR/qwen.txt" >"$RUN_DIR/explanation.txt"
[ -s "$RUN_DIR/change.patch" ] || fail "Qwen returned no valid patch"
if grep -E '(^|/)(\.git|\.github|\.ssh|AGENT_CONTRACT\.md|\.qwen_autotest/run_all_tests\.sh)(/|$)' "$RUN_DIR/change.patch" >/dev/null; then fail "forbidden path in patch"; fi
git apply --check "$RUN_DIR/change.patch" || fail "patch does not apply cleanly"
git apply "$RUN_DIR/change.patch"
git diff --check || { git apply -R "$RUN_DIR/change.patch"; fail "whitespace error; reverted Qwen patch"; }
./.qwen_autotest/run_all_tests.sh >"$RUN_DIR/tests.stdout" 2>&1 || { git diff >"$RUN_DIR/failed.diff"; git apply -R "$RUN_DIR/change.patch"; sha256sum "$RUN_DIR/tests.stdout" >"$RUN_DIR/tests.sha256"; fail "tests failed; reverted Qwen patch"; }
sha256sum "$RUN_DIR/tests.stdout" >"$RUN_DIR/tests.sha256"
git diff --stat >"$RUN_DIR/diff.stat"
git status --short >"$RUN_DIR/status-after.txt"
echo "STATUS: PASS" | tee "$RUN_DIR/status.txt"
echo "PHASE: $PHASE_FILE" | tee -a "$RUN_DIR/status.txt"
echo "EVIDENCE: $RUN_DIR" | tee -a "$RUN_DIR/status.txt"
