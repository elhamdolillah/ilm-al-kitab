set -euo pipefail
cd /root/ilm-al-kitab
echo "== GIT STATUS =="
git status --short
git log --oneline -8
echo ""
echo "== CARGO ENV =="
command -v cargo || echo "cargo missing from PATH"
cargo --version || echo "cargo version failed"
rustc --version || echo "rustc version failed"
echo ""
echo "== PARSE_EXPR =="
sed -n '/^    fn parse_expr/,/^    }$/p' MAL/src/parser/src/lib.rs | head -40
echo ""
echo "== PARSE_ADDITIVE =="
sed -n '/^    fn parse_additive/,/^    }$/p' MAL/src/parser/src/lib.rs | head -40
echo ""
echo "== PARSE_MULTIPLICATIVE =="
sed -n '/^    fn parse_multiplicative/,/^    }$/p' MAL/src/parser/src/lib.rs | head -40
echo ""
echo "== PARSE_PRIMARY =="
sed -n '/^    fn parse_primary/,/^    }$/p' MAL/src/parser/src/lib.rs | head -80
echo ""
echo "== PARSE_CALL CHECK =="
grep -n 'fn parse_call' MAL/src/parser/src/lib.rs || echo "parse_call: NOT FOUND"
echo ""
echo "== TEST_PARSE_LAMBDA_APPLY =="
sed -n '/^    fn test_parse_lambda_apply/,/^    }$/p' MAL/src/parser/src/lib.rs
echo ""
echo "== ASTNODE ENUM =="
sed -n '/^pub enum ASTNode/,/^}$/p' MAL/src/arena/src/lib.rs | head -60
echo ""
echo "== ASTNODE CALL VARIANT =="
grep -n -B 1 -A 6 'Call {' MAL/src/arena/src/lib.rs
echo ""
echo "== EVIDENCE CHECKS =="
sha256sum -c evidence/MAL-PARSER-002.sha256 || echo "PARSER-002 evidence stale"
sha256sum -c evidence/MAL-ARENA-001-v2.sha256 || echo "ARENA-v2 evidence stale"
sha256sum -c evidence/MAL-LEXER-001.sha256 || echo "LEXER evidence stale"
echo ""
echo "== LAST VERBATIM TEST RESULT =="
grep -E '^(running|test result|test tests::test_parse_lambda)' evidence/MAL-PARSER-002.stdout || echo "no prior PARSER-002 stdout"
echo ""
echo "== BACKUP FILES =="
ls -la MAL/src/parser/src/lib.rs.bak* || echo "no backups present"
