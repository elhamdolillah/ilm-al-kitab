#!/usr/bin/env bash
set -euo pipefail

SUITE="${1:-/root/arabic_math_lang/test_all_phases.py}"
OUT="${2:-/root/ilm-al-kitab/evidence/MAL-MATH-SUITE-RUN.stdout}"

case "$SUITE" in
  /root/arabic_math_lang/*) ;;
  *) echo "BLOCKED: suite must be inside /root/arabic_math_lang" >&2; exit 90 ;;
esac
case "$OUT" in
  /root/ilm-al-kitab/evidence/*) ;;
  *) echo "BLOCKED: output must be inside ilm-al-kitab/evidence" >&2; exit 91 ;;
esac

mkdir -p "$(dirname "$OUT")"
set +e
python3 "$SUITE" >"$OUT" 2>&1
code=$?
set -e

if grep -qE 'RESULTS: [0-9]+/[0-9]+ passed' "$OUT"; then
  summary=$(grep -E 'RESULTS: [0-9]+/[0-9]+ passed' "$OUT" | tail -1)
  printf '%s\n' "$summary"
else
  echo "FAIL: summary line missing" >&2
  exit 92
fi

if grep -qE 'ALL PHASES PASSED!' "$OUT" && [ "$code" -eq 0 ]; then
  sha256sum "$OUT"
  exit 0
fi

echo "FAIL_CLOSED: suite failed or reported failures" >&2
sha256sum "$OUT" >&2
exit 1
