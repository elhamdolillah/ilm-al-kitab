#!/usr/bin/env bash
set -u
BASE="/root/ilm-al-kitab/.qwen_autotest"
LOGS="$BASE/logs"
REPORTS="$BASE/reports"
RUN_ID="$(date -u +%Y%m%d_%H%M%S)"
RUN_DIR="$LOGS/$RUN_ID"
REPORT="$REPORTS/latest_report.txt"
mkdir -p "$RUN_DIR"
run_one() {
  NAME="$1"
  shift
  LOG="$RUN_DIR/$NAME.log"
  echo "===== RUNNING: $NAME =====" | tee -a "$REPORT"
  "$@" > "$LOG" 2>&1
  CODE=$?
  cat "$LOG" | tee -a "$REPORT"
  echo "===== EXIT_CODE: $CODE =====" | tee -a "$REPORT"
  echo | tee -a "$REPORT"
  if [ "$CODE" -ne 0 ]; then
    echo "FINAL_STATUS: FAIL" | tee -a "$REPORT"
    echo "FAILED_TEST: $NAME" | tee -a "$REPORT"
    exit "$CODE"
  fi
}
: > "$REPORT"
run_one env bash -lc 'source /root/.cargo/env 2>/dev/null; cargo --version; rustc --version; test -d /root/ilm-al-kitab'
run_one git_status git status --short
run_one diff_check git diff --check
run_one parser_lambda_apply bash -lc 'cd /root/ilm-al-kitab/MAL/src/parser && source /root/.cargo/env 2>/dev/null && cargo test test_parse_lambda_apply --release -- --nocapture'
run_one parser_all bash -lc 'cd /root/ilm-al-kitab/MAL/src/parser && source /root/.cargo/env 2>/dev/null && cargo test --release -- --nocapture'
run_one workspace_release bash -lc 'cd /root/ilm-al-kitab/MAL/src/parser && source /root/.cargo/env 2>/dev/null && cargo test --release -- --nocapture'
echo "FINAL_STATUS: PASS" | tee -a "$REPORT"
echo "ALL_WHITELISTED_TESTS_PASSED" | tee -a "$REPORT"
