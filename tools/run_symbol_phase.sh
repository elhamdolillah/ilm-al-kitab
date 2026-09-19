#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="/root/ilm-al-kitab"
LOG_DIR="$ROOT/evidence/symbol-phases"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
LOG="$LOG_DIR/$STAMP.log"

mkdir -p "$LOG_DIR"
cd "$ROOT"
source /root/.cargo/env 2>/dev/null || true

exec > >(tee -a "$LOG") 2>&1

echo "===== SYMBOL PHASE START ====="
echo "timestamp_utc=$STAMP"

test -f "$ROOT/TASKS/ACTIVE/SYMBOLS_NEXT.md"

echo "===== BASELINE ====="
"$ROOT/.qwen_autotest/run_all_tests.sh"

echo "===== ACTIVE TASK ====="
cat "$ROOT/TASKS/ACTIVE/SYMBOLS_NEXT.md"

echo "===== READY ====="
echo "Baseline passed. Execute exactly the active task."
