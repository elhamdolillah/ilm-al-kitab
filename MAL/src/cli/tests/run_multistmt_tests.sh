#!/usr/bin/env bash
# Regression test suite for multi-statement programs
set -uo pipefail
MALC="${1:-/root/ilm-al-kitab/MAL/tools/malc-native/target/release/malc-native}"
FIXTURES="/root/ilm-al-kitab/MAL/src/cli/tests/fixtures/multistmt"
run_test() {
    local name="$1"
    local file="$2"
    local expected="$3"
    if [ ! -f "$file" ]; then
        echo "❌ $name — fixture missing"
        return 1
    fi
    "$MALC" "$file" -o /tmp/test.bin 2>/dev/null
    if [ ! -f /tmp/test.bin ]; then
        echo "❌ $name — compile failed"
        return 1
    fi
    chmod +x /tmp/test.bin 2>/dev/null
    /tmp/test.bin 2>/dev/null
    local R=$?
    if [ "$R" = "$expected" ]; then
        echo "✅ $name = $R"
        return 0
    elif [ "$((expected % 256))" = "$R" ]; then
        echo "⚠️  $name = $R (masked, expected $expected)"
        return 0
    else
        echo "❌ $name = $R (expected $expected)"
        return 1
    fi
}
PASS=0
FAIL=0
echo "══════════════════════════════════════"
echo "  MAL Multi-Statement Regression Suite"
echo "══════════════════════════════════════"
echo ""
if run_test "simple_add (5+3)" "$FIXTURES/simple_add.mal" "8"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi
if run_test "two_vars (5+10)" "$FIXTURES/two_vars.mal" "15"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi
if run_test "three_vars (5*10)" "$FIXTURES/three_vars.mal" "50"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi
if run_test "five_lines (6*10)" "$FIXTURES/five_lines.mal" "60"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi
if run_test "builtin_then_expr" "$FIXTURES/builtin_then_expr.mal" "12"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi
if run_test "nested_builtins" "$FIXTURES/nested_builtins.mal" "12"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi
if run_test "arabic_builtins (جذر)" "$FIXTURES/arabic_builtins.mal" "12"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi
if run_test "mixed_operations (100-50)" "$FIXTURES/mixed_operations.mal" "50"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); fi
echo ""
echo "══════════════════════════════════════"
echo "  Results: Pass=$PASS, Fail=$FAIL"
echo "══════════════════════════════════════"
[ "$FAIL" -eq 0 ]
