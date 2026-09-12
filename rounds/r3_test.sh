set -euo pipefail
cd /root/ilm-al-kitab/MAL/src/parser
set +e
cargo check --release > /root/ilm-al-kitab/evidence/MAL-PARSER-002-r2-check.stdout 2>&1
check_code=$?
set -e
if [ "$check_code" -ne 0 ]; then
  echo "COMPILE_GATE_FAILED exit=$check_code"
  tail -20 /root/ilm-al-kitab/evidence/MAL-PARSER-002-r2-check.stdout
  exit "$check_code"
fi
set +e
cargo test --release > /root/ilm-al-kitab/evidence/MAL-PARSER-002-r2.stdout 2>&1
code=$?
set -e
cd /root/ilm-al-kitab
sha256sum evidence/MAL-PARSER-002-r2.stdout > evidence/MAL-PARSER-002-r2.sha256
echo "CARGO_TEST_EXIT=$code"
grep -E '^(running|test result)' evidence/MAL-PARSER-002-r2.stdout
grep -E '^test tests::' evidence/MAL-PARSER-002-r2.stdout
cat evidence/MAL-PARSER-002-r2.sha256
exit "$code"
