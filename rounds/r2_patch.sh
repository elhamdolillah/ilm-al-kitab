set -euo pipefail
cd /root/ilm-al-kitab
TS="$(date -u +%Y%m%d_%H%M%S)"
cp MAL/src/parser/src/lib.rs "MAL/src/parser/src/lib.rs.bak_$TS"
python3 - << 'PY'
from pathlib import Path
p = Path("MAL/src/parser/src/lib.rs")
t = p.read_text(encoding="utf-8")
old = """            TokenKind::LParen => {
                let expr = self.parse_expr(arena)?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }"""
new = """            TokenKind::LParen => {
                let mut expr = self.parse_expr(arena)?;
                self.expect(TokenKind::RParen)?;
                while self.peek().map_or(false, |t| t.kind == TokenKind::LParen) {
                    self.bump();
                    let mut args = Vec::new();
                    while !self.peek().map_or(false, |t| t.kind == TokenKind::RParen) {
                        args.push(self.parse_expr(arena)?);
                        if self.peek().map_or(false, |t| t.kind == TokenKind::Comma) {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    let args_node = if args.is_empty() {
                        NodeID::INVALID
                    } else if args.len() == 1 {
                        args[0]
                    } else {
                        let mut list = arena.allocate(ASTNode::List { head: args[args.len() - 1], tail: NodeID::INVALID })?;
                        for i in (0..args.len() - 1).rev() {
                            list = arena.allocate(ASTNode::List { head: args[i], tail: list })?;
                        }
                        list
                    };
                    expr = arena.allocate(ASTNode::Call { func: expr, args: args_node })?;
                }
                Ok(expr)
            }"""
n = t.count(old)
print(f"anchor_count={n}")
if n != 1:
    raise SystemExit("ABORT: anchor count != 1")
p.write_text(t.replace(old, new), encoding="utf-8")
print("PATCH_OK")
PY
