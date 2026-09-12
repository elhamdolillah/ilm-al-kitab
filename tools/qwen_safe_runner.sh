#!/usr/bin/env bash
set -euo pipefail
PROJECT_DIR="${PROJECT_DIR:-/root/ilm-al-kitab}"
COMMAND_FILE="${1:?usage: qwen_safe_runner.sh COMMAND_FILE}"
RUN_ID="$(date -u +%Y%m%d_%H%M%S)"
RUN_DIR="$PROJECT_DIR/.qwen_runner"
RESULT_FILE="$RUN_DIR/result_${RUN_ID}.log"
NORMALIZED_FILE="$RUN_DIR/normalized_${RUN_ID}.sh"
case "$COMMAND_FILE" in
  "$PROJECT_DIR"/*) ;;
  *) echo "BLOCKED: command file must be inside $PROJECT_DIR" >&2; exit 90 ;;
esac
mkdir -p "$RUN_DIR"
python3 - "$COMMAND_FILE" "$NORMALIZED_FILE" << 'PY'
from pathlib import Path
import sys
src = Path(sys.argv[1])
dst = Path(sys.argv[2])
data = src.read_bytes()
if b"\x00" in data:
    raise SystemExit("BLOCKED: NUL byte")
text = data.decode("utf-8")
text = text.replace("\r\n", "\n").replace("\r", "\n")
if not text.endswith("\n"):
    text += "\n"
if not text.strip():
    raise SystemExit("BLOCKED: empty command file")
dst.write_text(text, encoding="utf-8", newline="\n")
PY
if grep -nE '(^|[[:space:]])(rm[[:space:]]+-r|git[[:space:]]+reset[[:space:]]+--hard|git[[:space:]]+clean[[:space:]]+-f|git[[:space:]]+push[[:space:]]+(-f|--force)|mkfs|dd[[:space:]]+if=|shutdown|reboot|curl[[:space:]].*\|[[:space:]]*(ba)?sh|wget[[:space:]].*\|[[:space:]]*(ba)?sh)' "$NORMALIZED_FILE"; then
  echo "BLOCKED: destructive or remote-publishing command detected" >&2
  exit 91
fi
if grep -nE 'sed[[:space:]]+-i[^;|]*\.rs' "$NORMALIZED_FILE"; then
  echo "BLOCKED: sed -i on .rs forbidden (DEC-005); use python patch with match count" >&2
  exit 92
fi
bash -n "$NORMALIZED_FILE"
if [ -d /root/.cargo/bin ]; then
  export PATH="/root/.cargo/bin:$PATH"
fi
{
  echo "========== QWEN SAFE RUN START =========="
  echo "project_dir: $PROJECT_DIR"
  echo "command_file: $COMMAND_FILE"
  echo "command_sha256: $(sha256sum "$NORMALIZED_FILE" | cut -d' ' -f1)"
  echo "---------- COMMANDS ----------"
  cat "$NORMALIZED_FILE"
  echo "---------- OUTPUT ----------"
  cd "$PROJECT_DIR"
  set +e
  timeout 900 bash "$NORMALIZED_FILE"
  code=$?
  set -e
  echo "---------- EXIT CODE ----------"
  echo "$code"
  echo "=========== QWEN SAFE RUN END ==========="
  echo "$code" > "$RUN_DIR/exit_${RUN_ID}.code"
} > "$RESULT_FILE" 2>&1
code="$(cat "$RUN_DIR/exit_${RUN_ID}.code")"
echo "RESULT_FILE: $RESULT_FILE"
echo "RESULT_SHA256: $(sha256sum "$RESULT_FILE" | cut -d' ' -f1)"
exit "$code"
