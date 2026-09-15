#!/usr/bin/env bash
set -u
ROOT="/root/ilm-al-kitab"
OUT="$ROOT/evidence/selective-merge-tests-20260915"
mkdir -p "$OUT"
PASS=0; FAIL=0
run_gate() { local name="$1"; shift; echo "=== $name ===" | tee -a "$OUT/report.txt"; "$@" >"$OUT/$name.stdout" 2>&1; local c=$?; cat "$OUT/$name.stdout" | tee -a "$OUT/report.txt"; echo "EXIT=$c" | tee -a "$OUT/report.txt"; if [ "$c" -eq 0 ]; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi; }
: > "$OUT/report.txt"
for f in "$ROOT/CONTRACTS/OLD_MATH_NUMERIC_CONTRACT.md" "$ROOT/CONTRACTS/LINEAR_OWNERSHIP_CONTRACT.md" "$ROOT/CONTRACTS/INTEROP-C2-CONTRACT.md"; do test -s "$f" || { echo "MISSING $f"; exit 2; }; done
run_gate contract_syntax bash -lc "grep -q 'الحالة' '$ROOT/CONTRACTS/OLD_MATH_NUMERIC_CONTRACT.md' && grep -q 'الحالة' '$ROOT/CONTRACTS/LINEAR_OWNERSHIP_CONTRACT.md' && grep -q 'النطاق' '$ROOT/CONTRACTS/INTEROP-C2-CONTRACT.md'"
run_gate interop_c2 python3 "$ROOT/RESEARCH/INTEROP/equivalence_model.py"
run_gate set_theory python3 "$ROOT/RESEARCH/SET_THEORY/set_model.py"
run_gate ownership_contract python3 "$ROOT/RESEARCH/INTEROP/test_linear_ownership_contract.py"
# Legacy suite is diagnostic, never treated as a new-language gate.
set +e
python3 /root/arabic_math_lang/test_all_phases.py >"$OUT/legacy_math_suite.stdout" 2>&1
LEGACY_CODE=$?
set -e
printf 'LEGACY_EXIT=%s\n' "$LEGACY_CODE" | tee -a "$OUT/report.txt"
printf 'GATES_PASS=%s\nGATES_FAIL=%s\n' "$PASS" "$FAIL" | tee -a "$OUT/report.txt"
sha256sum "$OUT"/*.stdout > "$OUT/stdout.sha256"
if [ "$FAIL" -ne 0 ]; then echo 'FINAL_STATUS: FAIL' | tee -a "$OUT/report.txt"; exit 1; fi
echo 'FINAL_STATUS: PASS' | tee -a "$OUT/report.txt"
