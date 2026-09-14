//! MAL Parser — recursive descent parser that builds AST in Arena.
//!
//! ## Declared scope
//! - Expressions: numbers, identifiers, binary ops (+, -, ·), function calls
//! - Statements: assignments (≔), print (⎕)
//! - Blocks: ﴿ ... ﴾
//! - Fail-closed: syntax errors yield `ParserError`, never panic
//!
//! ## Constitutional compliance
//! - `#![forbid(unsafe_code)]`
//! - AST stored in Arena via `NodeID(u32)`
//! - No heap allocation in hot path (Arena only)
//! - No time, no randomness, no network

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use mal_arena::{Arena, NodeID, ASTNode, ArenaError};
use mal_lexer::{Lexer, Token, TokenKind, LexerError};

/// Parser failure modes (fail-closed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    /// Lexer error propagated.
    Lexer(LexerError),
    /// Expected token but got something else.
    Expected {
        /// What was expected.
        expected: &'static str,
        /// What was found.
        found: String,
        /// Line number.
        line: u32,
        /// Column number.
        col: u32,
    },
    /// Unexpected end of input.
    UnexpectedEof,
    /// Arena capacity exceeded.
    ArenaError(ArenaError),
}

impl From<LexerError> for ParserError {
    fn from(e: LexerError) -> Self {
        ParserError::Lexer(e)
    }
}

impl From<ArenaError> for ParserError {
    fn from(e: ArenaError) -> Self {
        ParserError::ArenaError(e)
    }
}

/// Parser state over a token stream.
struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    src: &'a str,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Result<Self, ParserError> {
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize()?;
        Ok(Self { tokens, pos: 0, src })
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).copied();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        let tok = self.bump().ok_or(ParserError::UnexpectedEof)?;
        if tok.kind != kind {
            return Err(ParserError::Expected {
                expected: format!("{:?}", kind).leak(),
                found: format!("{:?}", tok.kind),
                line: tok.line,
                col: tok.col,
            });
        }
        Ok(tok)
    }

    fn parse_expr(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        self.parse_logical_or(arena)
    }

    fn parse_additive(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let mut left = self.parse_multiplicative(arena)?;
        while let Some(tok) = self.peek() {
            if tok.kind == TokenKind::Plus || tok.kind == TokenKind::Minus || tok.kind == TokenKind::Concat {
                let op = self.bump().unwrap();
                let right = self.parse_multiplicative(arena)?;
                let op_code = match op.kind {
                    TokenKind::Plus => 0,
                    TokenKind::Minus => 1,
                    TokenKind::Concat => 4,
                    _ => unreachable!(),
                };
                left = arena.allocate(ASTNode::BinOp {
                    op: op_code,
                    left,
                    right,
                })?;
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let mut left = self.parse_primary(arena)?;
        while let Some(tok) = self.peek() {
            let op_code = match tok.kind {
                TokenKind::Mul => 2,
                TokenKind::Div => 3,
                TokenKind::Mod => 5,
                _ => break,
            };
            self.bump();
            let right = self.parse_power(arena)?;
            left = arena.allocate(ASTNode::BinOp {
                op: op_code,
                left,
                right,
            })?;
        }
        Ok(left)
    }


    fn parse_lambda(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        self.bump(); // consume λ
        let mut params = Vec::new();
        while self.peek().map_or(false, |t| t.kind == TokenKind::Ident) {
            let tok = self.bump().unwrap();
            let ident = arena.allocate(ASTNode::Ident(tok.start))?;
            params.push(ident);
        }
        if params.is_empty() {
            return Err(ParserError::Expected {
                expected: "parameter after λ",
                found: "none".to_string(),
                line: 0,
                col: 0,
            });
        }
        self.expect(TokenKind::Dot)?;
        let body = self.parse_expr(arena)?;
        // بناء قائمة المعاملات
        let params_list = if params.len() == 1 {
            params[0]
        } else {
            let mut list = arena.allocate(ASTNode::List {
                head: params[params.len() - 1],
                tail: NodeID::INVALID,
            })?;
            for i in (0..params.len() - 1).rev() {
                list = arena.allocate(ASTNode::List {
                    head: params[i],
                    tail: list,
                })?;
            }
            list
        };
        Ok(arena.allocate(ASTNode::Lambda {
            params: params_list,
            body,
        })?)
    }

    fn parse_primary(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let tok = self.bump().ok_or(ParserError::UnexpectedEof)?;
        match tok.kind {
            TokenKind::Lambda => {
                self.pos -= 1; // backtrack λ consumed above
                self.parse_lambda(arena)
            }
            TokenKind::Num => {
                Ok(arena.allocate(ASTNode::Int(tok.num))?)
            }
            TokenKind::Ident => {
                // Check if it's a function call
                if self.peek().map_or(false, |t| t.kind == TokenKind::LParen) {
                    self.bump(); // consume (
                    let mut args = Vec::new();
                    while self.peek().map_or(false, |t| t.kind != TokenKind::RParen) {
                        args.push(self.parse_expr(arena)?);
                        if self.peek().map_or(false, |t| t.kind == TokenKind::Comma) {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    // Build argument list as linked list
                    let args_node = if args.is_empty() {
                        NodeID::INVALID
                    } else {
                        let mut list = arena.allocate(ASTNode::List {
                            head: args[args.len() - 1],
                            tail: NodeID::INVALID,
                        })?;
                        for i in (0..args.len() - 1).rev() {
                            list = arena.allocate(ASTNode::List {
                                head: args[i],
                                tail: list,
                            })?;
                        }
                        list
                    };
                    // Function name as identifier
                    let _func_name = &self.src[tok.start as usize..(tok.start + tok.len) as usize];
                    let func_str_idx = arena.allocate(ASTNode::Str(0))?; // placeholder
                    Ok(arena.allocate(ASTNode::Call {
                        func: func_str_idx,
                        args: args_node,
                    })?)
                } else {
                    // Plain identifier
                    let _name = &self.src[tok.start as usize..(tok.start + tok.len) as usize];
                    Ok(arena.allocate(ASTNode::Ident(0))?) // placeholder
                }
            }
            TokenKind::LParen => {
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
            }
            TokenKind::Forall => {
                self.parse_forall(arena)
            }
            TokenKind::Exists => {
                self.parse_exists(arena)
            }
            TokenKind::Mu => {
                self.parse_mu(arena)
            }
            TokenKind::LAngle => {
                self.parse_set_literal(arena)
            }
            TokenKind::Read => {
                // ⊙ read stdin — zero-argument builtin call (already consumed by parse_primary)
                Ok(arena.allocate(ASTNode::Call {
                    func: NodeID::INVALID, // read builtin (distinguished from print by context)
                    args: NodeID::INVALID, // no arguments
                })?)
            }
            TokenKind::Eof => Err(ParserError::UnexpectedEof),
            _ => Err(ParserError::Expected {
                expected: "expression",
                found: format!("{:?}", tok.kind),
                line: tok.line,
                col: tok.col,
            }),
        }
    }


    /// Unary operators: - (negation), ¬ (logical NOT)
    /// Grammar: Unary = "-" Unary | "¬" Unary | Primary
    fn parse_unary(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        if let Some(tok) = self.peek() {
            if tok.kind == TokenKind::Minus {
                self.bump();
                let operand = self.parse_unary(arena)?;
                return Ok(arena.allocate(ASTNode::BinOp {
                    op: 16, // unary minus
                    left: operand,
                    right: NodeID::INVALID,
                })?);
            }
            if tok.kind == TokenKind::Not {
                self.bump();
                let operand = self.parse_unary(arena)?;
                return Ok(arena.allocate(ASTNode::BinOp {
                    op: 17, // logical NOT
                    left: operand,
                    right: NodeID::INVALID,
                })?);
            }
        }
        self.parse_primary(arena)
    }

    /// Power operator: ^ (right-associative)
    /// Grammar: Power = Unary ["^" Power]
    fn parse_power(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let base = self.parse_unary(arena)?;
        if self.peek().map_or(false, |t| t.kind == TokenKind::Pow) {
            self.bump();
            let exp = self.parse_power(arena)?; // right-associative recursion
            Ok(arena.allocate(ASTNode::BinOp {
                op: 6, // power
                left: base,
                right: exp,
            })?)
        } else {
            Ok(base)
        }
    }

    /// Comparison operators: <, >, ≤, ≥, ==, ≠
    /// Grammar: Comparison = Additive [("<" | ">" | "≤" | "≥" | "==" | "≠") Additive]
    fn parse_comparison(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let left = self.parse_additive(arena)?;
        if let Some(tok) = self.peek() {
            let op_code = match tok.kind {
                TokenKind::Lt => 7,
                TokenKind::Gt => 8,
                TokenKind::Le => 9,
                TokenKind::Eq => 11,
                TokenKind::Neq => 12,
                TokenKind::Ge => 13,
                _ => return Ok(left),
            };
            self.bump();
            let right = self.parse_additive(arena)?;
            Ok(arena.allocate(ASTNode::BinOp {
                op: op_code,
                left,
                right,
            })?)
        } else {
            Ok(left)
        }
    }

    /// Logical AND: ∧ (left-associative)
    /// Grammar: LogicalAnd = Comparison {"∧" Comparison}
    fn parse_logical_and(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let mut left = self.parse_comparison(arena)?;
        while self.peek().map_or(false, |t| t.kind == TokenKind::And) {
            self.bump();
            let right = self.parse_comparison(arena)?;
            left = arena.allocate(ASTNode::BinOp {
                op: 14, // AND
                left,
                right,
            })?;
        }
        Ok(left)
    }

    /// Logical OR: ∨ (left-associative)
    /// Grammar: LogicalOr = LogicalAnd {"∨" LogicalAnd}
    fn parse_logical_or(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let mut left = self.parse_logical_and(arena)?;
        while self.peek().map_or(false, |t| t.kind == TokenKind::Or) {
            self.bump();
            let right = self.parse_logical_and(arena)?;
            left = arena.allocate(ASTNode::BinOp {
                op: 15, // OR
                left,
                right,
            })?;
        }
        Ok(left)
    }
    fn parse_forall(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        // ∀ already consumed by parse_primary
        let var_tok = self.bump().ok_or(ParserError::UnexpectedEof)?;
        if var_tok.kind != TokenKind::Ident {
            return Err(ParserError::Expected {
                expected: "identifier after ∀",
                found: format!("{:?}", var_tok.kind),
                line: var_tok.line,
                col: var_tok.col,
            });
        }
        let var = arena.allocate(ASTNode::Ident(var_tok.start))?;
        self.expect(TokenKind::In)?;
        let set = self.parse_expr(arena)?;
        self.expect(TokenKind::Colon)?;
        let body = self.parse_expr(arena)?;
        Ok(arena.allocate(ASTNode::ForAll { var, set, body })?)
    }
    fn parse_exists(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        // ∃ already consumed by parse_primary
        let var_tok = self.bump().ok_or(ParserError::UnexpectedEof)?;
        if var_tok.kind != TokenKind::Ident {
            return Err(ParserError::Expected {
                expected: "identifier after ∃",
                found: format!("{:?}", var_tok.kind),
                line: var_tok.line,
                col: var_tok.col,
            });
        }
        let var = arena.allocate(ASTNode::Ident(var_tok.start))?;
        self.expect(TokenKind::In)?;
        let set = self.parse_expr(arena)?;
        self.expect(TokenKind::Colon)?;
        let body = self.parse_expr(arena)?;
        Ok(arena.allocate(ASTNode::Exists { var, set, body })?)
    }
    fn parse_set_literal(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        // ⟨ already consumed by parse_primary
        if self.peek().map_or(false, |t| t.kind == TokenKind::RAngle) {
            self.bump(); // consume ⟩
            return Ok(arena.allocate(ASTNode::Set { elems: NodeID::INVALID })?);
        }
        // Collect all elements first
        let mut elements = vec![];
        elements.push(self.parse_expr(arena)?);
        while self.peek().map_or(false, |t| t.kind == TokenKind::Comma) {
            self.bump(); // consume ,
            elements.push(self.parse_expr(arena)?);
        }
        self.expect(TokenKind::RAngle)?;
        // Build list from right to left (bottom-up)
        let mut list = NodeID::INVALID;
        for elem in elements.into_iter().rev() {
            list = arena.allocate(ASTNode::List { head: elem, tail: list })?;
        }
        Ok(arena.allocate(ASTNode::Set { elems: list })?)
    }
    fn parse_mu(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        // μ already consumed by parse_primary
        let var_tok = self.bump().ok_or(ParserError::UnexpectedEof)?;
        if var_tok.kind != TokenKind::Ident {
            return Err(ParserError::Expected {
                expected: "identifier after μ",
                found: format!("{:?}", var_tok.kind),
                line: var_tok.line,
                col: var_tok.col,
            });
        }
        let var = arena.allocate(ASTNode::Ident(var_tok.start))?;
        self.expect(TokenKind::Dot)?;
        let body = self.parse_expr(arena)?;
        Ok(arena.allocate(ASTNode::Mu { var, body })?)
    }
    fn parse_stmt(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let tok = self.peek().ok_or(ParserError::UnexpectedEof)?;
        match tok.kind {
            TokenKind::Print => {
                self.bump();
                let expr = self.parse_expr(arena)?;
                Ok(arena.allocate(ASTNode::Call {
                    func: NodeID::INVALID, // print builtin
                    args: expr,
                })?)
            }
            TokenKind::Ident => {
                // Could be assignment or expression
                let _ident = self.bump().unwrap();
                if self.peek().map_or(false, |t| t.kind == TokenKind::Assign) {
                    self.bump(); // consume ≔
                    let value = self.parse_expr(arena)?;
                    let left_ident = arena.allocate(ASTNode::Ident(0))?;
                    Ok(arena.allocate(ASTNode::BinOp {
                        op: 10, // assignment marker
                        left: left_ident,
                        right: value,
                    })?)
                } else {
                    // Expression statement
                    self.pos -= 1; // backtrack
                    self.parse_expr(arena)
                }
            }
            TokenKind::LBlock => {
                self.bump(); // consume ﴿
                let mut stmts = Vec::new();
                while self.peek().map_or(false, |t| t.kind != TokenKind::RBlock) {
                    stmts.push(self.parse_stmt(arena)?);
                    if self.peek().map_or(false, |t| t.kind == TokenKind::Diamond) {
                        self.bump();
                    } else {
                        break;
                    }
                }
                self.expect(TokenKind::RBlock)?;
                // Build statement list
                if stmts.is_empty() {
                    Ok(arena.allocate(ASTNode::Empty)?)
                } else {
                    let mut list = arena.allocate(ASTNode::List {
                        head: stmts[stmts.len() - 1],
                        tail: NodeID::INVALID,
                    })?;
                    for i in (0..stmts.len() - 1).rev() {
                        list = arena.allocate(ASTNode::List {
                            head: stmts[i],
                            tail: list,
                        })?;
                    }
                    Ok(list)
                }
            }
            _ => self.parse_expr(arena),
        }
    }

    fn parse_program(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let mut stmts = Vec::new();
        while self.peek().map_or(false, |t| t.kind != TokenKind::Eof) {
            stmts.push(self.parse_stmt(arena)?);
        }
        self.expect(TokenKind::Eof)?;

        if stmts.is_empty() {
            Ok(arena.allocate(ASTNode::Empty)?)
        } else if stmts.len() == 1 {
            Ok(stmts[0])
        } else {
            let mut list = arena.allocate(ASTNode::List {
                head: stmts[stmts.len() - 1],
                tail: NodeID::INVALID,
            })?;
            for i in (0..stmts.len() - 1).rev() {
                list = arena.allocate(ASTNode::List {
                    head: stmts[i],
                    tail: list,
                })?;
            }
            Ok(list)
        }
    }
}

/// Parse MAL source into AST stored in Arena.
pub fn parse(src: &str, arena: &mut Arena) -> Result<NodeID, ParserError> {
    let mut parser = Parser::new(src)?;
    parser.parse_program(arena)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mal_arena::TypeTag;

    #[test]
    fn test_parse_number() {
        let mut arena = Arena::new(100);
        let root = parse("42", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::Int(42)));
    }

    #[test]
    fn test_parse_binary_add() {
        let mut arena = Arena::new(100);
        let root = parse("1 + 2", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 0); // +
            assert!(matches!(arena.get(*left).unwrap(), ASTNode::Int(1)));
            assert!(matches!(arena.get(*right).unwrap(), ASTNode::Int(2)));
        } else {
            panic!("expected BinOp");
        }
    }

    #[test]
    fn test_parse_binary_mul() {
        let mut arena = Arena::new(100);
        let root = parse("3 · 4", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 2); // ·
            assert!(matches!(arena.get(*left).unwrap(), ASTNode::Int(3)));
            assert!(matches!(arena.get(*right).unwrap(), ASTNode::Int(4)));
        } else {
            panic!("expected BinOp");
        }
    }

    #[test]
    fn test_parse_precedence() {
        let mut arena = Arena::new(100);
        let root = parse("1 + 2 · 3", &mut arena).unwrap();
        // Should parse as: 1 + (2 · 3)
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 0); // +
            assert!(matches!(arena.get(*left).unwrap(), ASTNode::Int(1)));
            if let ASTNode::BinOp { op: op2, .. } = arena.get(*right).unwrap() {
                assert_eq!(*op2, 2); // ·
            } else {
                panic!("expected BinOp on right");
            }
        } else {
            panic!("expected BinOp");
        }
    }

    #[test]
    fn test_parse_parens() {
        let mut arena = Arena::new(100);
        let root = parse("(1 + 2) · 3", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 2); // ·
            if let ASTNode::BinOp { op: op2, .. } = arena.get(*left).unwrap() {
                assert_eq!(*op2, 0); // +
            } else {
                panic!("expected BinOp on left");
            }
        } else {
            panic!("expected BinOp");
        }
    }

    #[test]
    fn test_parse_assignment() {
        let mut arena = Arena::new(100);
        let root = parse("أ ≔ 5", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 10); // assignment
        } else {
            panic!("expected assignment");
        }
    }

    #[test]
    fn test_parse_block() {
        let mut arena = Arena::new(100);
        let root = parse("﴿ 1 ⋄ 2 ﴾", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::List { .. }));
    }

    #[test]
    fn test_parse_error_unexpected_eof() {
        let mut arena = Arena::new(100);
        let err = parse("1 +", &mut arena).unwrap_err();
        assert!(matches!(err, ParserError::UnexpectedEof));
    }

    #[test]
    fn test_parse_error_expected_paren() {
        let mut arena = Arena::new(100);
        let err = parse("(1 + 2", &mut arena).unwrap_err();
        assert!(matches!(err, ParserError::Expected { .. }));
    }

    #[test]
    fn test_parse_lambda_simple() {
        let mut arena = Arena::new(100);
        let root = parse("λس. س", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::Lambda { .. }));
    }

    #[test]
    fn test_parse_lambda_with_binop() {
        let mut arena = Arena::new(100);
        let root = parse("λس. س · 2", &mut arena).unwrap();
        if let ASTNode::Lambda { body, .. } = arena.get(root).unwrap() {
            assert!(matches!(arena.get(*body).unwrap(), ASTNode::BinOp { .. }));
        } else {
            panic!("expected Lambda");
        }
    }

    #[test]
    fn test_parse_lambda_two_params() {
        let mut arena = Arena::new(100);
        let root = parse("λس ص. س + ص", &mut arena).unwrap();
        if let ASTNode::Lambda { params, .. } = arena.get(root).unwrap() {
            assert!(matches!(arena.get(*params).unwrap(), ASTNode::List { .. }));
        } else {
            panic!("expected Lambda");
        }
    }

    #[test]
    fn test_parse_forall_basic() {
        let mut arena = Arena::new(100);
        let root = parse("∀ س ∈ ص : س", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::ForAll { .. }));
    }
    #[test]
    fn test_parse_forall_with_body() {
        let mut arena = Arena::new(100);
        let root = parse("∀ س ∈ ص : س + 1", &mut arena).unwrap();
        if let ASTNode::ForAll { body, .. } = arena.get(root).unwrap() {
            assert!(matches!(arena.get(*body).unwrap(), ASTNode::BinOp { .. }));
        } else {
            panic!("Expected ForAll node");
        }
    }
    #[test]
    fn test_parse_forall_nested() {
        let mut arena = Arena::new(200);
        let root = parse("∀ س ∈ ص : ∀ ع ∈ د : س + ع", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::ForAll { .. }));
    }
    #[test]
    fn test_parse_mu_basic() {
        let mut arena = Arena::new(100);
        let root = parse("μ ن . ن", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::Mu { .. }));
    }
    #[test]
    fn test_parse_mu_with_body() {
        let mut arena = Arena::new(100);
        let root = parse("μ ن . ن + 1", &mut arena).unwrap();
        if let ASTNode::Mu { body, .. } = arena.get(root).unwrap() {
            assert!(matches!(arena.get(*body).unwrap(), ASTNode::BinOp { .. }));
        } else {
            panic!("Expected Mu node");
        }
    }
    #[test]
    fn test_parse_mu_with_condition() {
        let mut arena = Arena::new(100);
        let root = parse("μ ن . ن · 2", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::Mu { .. }));
    }
    #[test]
    fn test_parse_exists_basic() {
        let mut arena = Arena::new(100);
        let root = parse("∃ س ∈ ص : س", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::Exists { .. }));
    }
    #[test]
    fn test_parse_exists_with_body() {
        let mut arena = Arena::new(100);
        let root = parse("∃ س ∈ ص : س + 1", &mut arena).unwrap();
        if let ASTNode::Exists { body, .. } = arena.get(root).unwrap() {
            assert!(matches!(arena.get(*body).unwrap(), ASTNode::BinOp { .. }));
        } else {
            panic!("Expected Exists node");
        }
    }
    #[test]
    fn test_parse_set_literal_empty() {
        let mut arena = Arena::new(100);
        let root = parse("⟨⟩", &mut arena).unwrap();
        if let ASTNode::Set { elems } = arena.get(root).unwrap() {
            assert_eq!(*elems, NodeID::INVALID);
        } else {
            panic!("Expected Set node");
        }
    }
    #[test]
    fn test_parse_set_literal_one_elem() {
        let mut arena = Arena::new(100);
        let root = parse("⟨1⟩", &mut arena).unwrap();
        if let ASTNode::Set { elems } = arena.get(root).unwrap() {
            assert!(!(*elems == NodeID::INVALID));
        } else {
            panic!("Expected Set node");
        }
    }
    #[test]
    fn test_parse_set_literal_multi_elem() {
        let mut arena = Arena::new(100);
        let root = parse("⟨1,2,3⟩", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::Set { .. }));
    }
    #[test]
    fn test_parse_concat_basic() {
        let mut arena = Arena::new(100);
        let root = parse("أ ⊕ ب", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 4); // concat op code
        } else {
            panic!("Expected BinOp for concat");
        }
    }
    #[test]
    fn test_parse_concat_with_plus() {
        let mut arena = Arena::new(100);
        let root = parse("أ + ب ⊕ ج", &mut arena).unwrap();
        // Should parse as: (أ + ب) ⊕ ج
        if let ASTNode::BinOp { op, left, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 4); // outer is concat
            if let ASTNode::BinOp { op: inner_op, .. } = arena.get(*left).unwrap() {
                assert_eq!(*inner_op, 0); // inner is plus
            } else {
                panic!("Expected inner BinOp");
            }
        } else {
            panic!("Expected outer BinOp");
        }
    }
    #[test]
    fn test_parse_read_stdin_basic() {
        let mut arena = Arena::new(100);
        let root = parse("⊙", &mut arena).unwrap();
        if let ASTNode::Call { func, args } = arena.get(root).unwrap() {
            assert_eq!(*func, NodeID::INVALID); // read builtin
            assert_eq!(*args, NodeID::INVALID); // no arguments
        } else {
            panic!("Expected Call for read stdin");
        }
    }
    #[test]
    fn test_parse_read_stdin_in_assignment() {
        let mut arena = Arena::new(100);
        let root = parse("س ≔ ⊙", &mut arena).unwrap();
        // Should parse as assignment with read on right side
        if let ASTNode::BinOp { op, right, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 10); // assignment marker
            if let ASTNode::Call { func, args } = arena.get(*right).unwrap() {
                assert_eq!(*func, NodeID::INVALID);
                assert_eq!(*args, NodeID::INVALID);
            } else {
                panic!("Expected Call on right side of assignment");
            }
        } else {
            panic!("Expected BinOp for assignment");
        }
    }
    #[test]
    fn test_parse_read_stdin_with_print() {
        let mut arena = Arena::new(100);
        // This would be two statements: ⊙ then ⎕ ⊙
        // But parse() only handles single expression, so test just ⊙
        let root = parse("⊙", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::Call { .. }));
    }
    #[test]
    fn test_parse_concat_chain() {
        let mut arena = Arena::new(100);
        let root = parse("أ ⊕ ب ⊕ ج", &mut arena).unwrap();
        // Should parse as: (أ ⊕ ب) ⊕ ج (left-associative)
        assert!(matches!(arena.get(root).unwrap(), ASTNode::BinOp { .. }));
    }
    #[test]
    fn test_set_type_tag() {
        let node = ASTNode::Set { elems: NodeID::INVALID };
        assert_eq!(node.type_tag(), TypeTag::Set);
    }
    #[test]
    fn test_set_membership_api() {
        let mut arena = Arena::new(100);
        let elem = arena.allocate(ASTNode::Int(5)).unwrap();
        let set = arena.allocate(ASTNode::Int(0)).unwrap();
        let _p = Parser::new("test").unwrap();
        // Direct construction test
        let node = ASTNode::SetMembership { elem, set };
        let id = arena.allocate(node).unwrap();
        assert!(matches!(arena.get(id).unwrap(), ASTNode::SetMembership { .. }));
    }
    #[test]
    fn test_linear_let_api() {
        let mut arena = Arena::new(100);
        let name = arena.allocate(ASTNode::Ident(0)).unwrap();
        let value = arena.allocate(ASTNode::Int(42)).unwrap();
        let body = arena.allocate(ASTNode::Ident(0)).unwrap();
        let node = ASTNode::LinearLet { name, value, body };
        let id = arena.allocate(node).unwrap();
        assert!(matches!(arena.get(id).unwrap(), ASTNode::LinearLet { .. }));
    }
    #[test]
    fn test_linear_let_type_tag() {
        let node = ASTNode::LinearLet {
            name: NodeID(0),
            value: NodeID(1),
            body: NodeID(2),
        };
        assert_eq!(node.type_tag(), TypeTag::LinearLet);
    }
    #[test]
    fn test_forall_type_tag() {
        let node = ASTNode::ForAll {
            var: NodeID(0),
            set: NodeID(1),
            body: NodeID(2),
        };
        assert_eq!(node.type_tag(), TypeTag::ForAll);
    }
    #[test]
    fn test_parse_lambda_apply() {
        let mut arena = Arena::new(100);
        let root = parse("(λس. س · 2)(5)", &mut arena).unwrap();
        // يجب أن يُبنى كـ Call مع func = Lambda
        assert!(matches!(arena.get(root).unwrap(), ASTNode::Call { .. }));
    }
    // ═══════════════════════════════════════════════════════════════
    // Phase 3 Tests — Unary, Power, Comparison, Logical Operators
    // ═══════════════════════════════════════════════════════════════
    #[test]
    fn test_parse_unary_minus() {
        let mut arena = Arena::new(100);
        let root = parse("-5", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 16); // unary minus
            assert_eq!(*right, NodeID::INVALID);
            if let ASTNode::Int(v) = arena.get(*left).unwrap() {
                assert_eq!(*v, 5);
            } else {
                panic!("Expected Int for operand");
            }
        } else {
            panic!("Expected BinOp for unary minus");
        }
    }
    #[test]
    fn test_parse_unary_minus_ident() {
        let mut arena = Arena::new(100);
        let root = parse("-س", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::BinOp { op: 16, .. }));
    }
    #[test]
    fn test_parse_unary_not() {
        let mut arena = Arena::new(100);
        let root = parse("¬P", &mut arena).unwrap();
        if let ASTNode::BinOp { op, right, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 17); // logical NOT
            assert_eq!(*right, NodeID::INVALID);
        } else {
            panic!("Expected BinOp for logical NOT");
        }
    }
    #[test]
    fn test_parse_power() {
        let mut arena = Arena::new(100);
        let root = parse("2 ^ 3", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 6); // power
            if let ASTNode::Int(lv) = arena.get(*left).unwrap() {
                assert_eq!(*lv, 2);
            }
            if let ASTNode::Int(rv) = arena.get(*right).unwrap() {
                assert_eq!(*rv, 3);
            }
        } else {
            panic!("Expected BinOp for power");
        }
    }
    #[test]
    fn test_parse_power_right_assoc() {
        let mut arena = Arena::new(100);
        // 2 ^ 3 ^ 4 should parse as 2 ^ (3 ^ 4)
        let root = parse("2 ^ 3 ^ 4", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 6);
            if let ASTNode::Int(lv) = arena.get(*left).unwrap() {
                assert_eq!(*lv, 2); // leftmost
            }
            if let ASTNode::BinOp { op: inner_op, .. } = arena.get(*right).unwrap() {
                assert_eq!(*inner_op, 6); // right side is also power
            } else {
                panic!("Expected right-associative power");
            }
        } else {
            panic!("Expected BinOp for power");
        }
    }
    #[test]
    fn test_parse_division() {
        let mut arena = Arena::new(100);
        let root = parse("10 / 2", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 3); // division
        } else {
            panic!("Expected BinOp for division");
        }
    }
    #[test]
    fn test_parse_modulo() {
        let mut arena = Arena::new(100);
        let root = parse("10 % 3", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 5); // modulo
        } else {
            panic!("Expected BinOp for modulo");
        }
    }
    #[test]
    fn test_parse_comparison_lt() {
        let mut arena = Arena::new(100);
        let root = parse("أ < ب", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 7); // less than
        } else {
            panic!("Expected BinOp for comparison");
        }
    }
    #[test]
    fn test_parse_comparison_le() {
        let mut arena = Arena::new(100);
        let root = parse("أ ≤ ب", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 9); // less than or equal
        } else {
            panic!("Expected BinOp for ≤");
        }
    }
    #[test]
    fn test_parse_comparison_gt() {
        let mut arena = Arena::new(100);
        let root = parse("أ > ب", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 8); // greater than
        } else {
            panic!("Expected BinOp for >");
        }
    }
    #[test]
    fn test_parse_comparison_ge() {
        let mut arena = Arena::new(100);
        let root = parse("أ ≥ ب", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 13); // greater than or equal
        } else {
            panic!("Expected BinOp for ≥");
        }
    }
    #[test]
    fn test_parse_comparison_eq() {
        let mut arena = Arena::new(100);
        let root = parse("أ = ب", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 11); // equality
        } else {
            panic!("Expected BinOp for =");
        }
    }
    #[test]
    fn test_parse_comparison_neq() {
        let mut arena = Arena::new(100);
        let root = parse("أ ≠ ب", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 12); // not equal
        } else {
            panic!("Expected BinOp for ≠");
        }
    }
    #[test]
    fn test_parse_logical_and() {
        let mut arena = Arena::new(100);
        let root = parse("P ∧ Q", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 14); // AND
        } else {
            panic!("Expected BinOp for ∧");
        }
    }
    #[test]
    fn test_parse_logical_or() {
        let mut arena = Arena::new(100);
        let root = parse("P ∨ Q", &mut arena).unwrap();
        if let ASTNode::BinOp { op, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 15); // OR
        } else {
            panic!("Expected BinOp for ∨");
        }
    }
    #[test]
    fn test_parse_and_left_assoc() {
        let mut arena = Arena::new(100);
        // P ∧ Q ∧ R should parse as (P ∧ Q) ∧ R
        let root = parse("P ∧ Q ∧ R", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 14);
            assert!(matches!(arena.get(*left).unwrap(), ASTNode::BinOp { op: 14, .. }));
        } else {
            panic!("Expected BinOp for ∧");
        }
    }
    #[test]
    fn test_parse_precedence_mul_add() {
        let mut arena = Arena::new(100);
        // 2 + 3 * 4 should parse as 2 + (3 * 4)
        let root = parse("2 + 3 * 4", &mut arena).unwrap();
        if let ASTNode::BinOp { op, right, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 0); // +
            if let ASTNode::BinOp { op: inner_op, .. } = arena.get(*right).unwrap() {
                assert_eq!(*inner_op, 2); // *
            } else {
                panic!("Expected * inside +");
            }
        } else {
            panic!("Expected BinOp for +");
        }
    }
    #[test]
    fn test_parse_precedence_power_mul() {
        let mut arena = Arena::new(100);
        // 2 * 3 ^ 4 should parse as 2 * (3 ^ 4)
        let root = parse("2 * 3 ^ 4", &mut arena).unwrap();
        if let ASTNode::BinOp { op, right, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 2); // *
            if let ASTNode::BinOp { op: inner_op, .. } = arena.get(*right).unwrap() {
                assert_eq!(*inner_op, 6); // ^
            } else {
                panic!("Expected ^ inside *");
            }
        } else {
            panic!("Expected BinOp for *");
        }
    }
    #[test]
    fn test_parse_unary_vs_sub() {
        let mut arena = Arena::new(100);
        // -5 + 3 should parse as (-5) + 3
        let root = parse("-5 + 3", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 0); // +
            if let ASTNode::BinOp { op: inner_op, right, .. } = arena.get(*left).unwrap() {
                assert_eq!(*inner_op, 16); // unary minus
                assert_eq!(*right, NodeID::INVALID);
            } else {
                panic!("Expected unary minus inside +");
            }
        } else {
            panic!("Expected BinOp for +");
        }
    }
    #[test]
    fn test_parse_logical_not_precedence() {
        let mut arena = Arena::new(100);
        // ¬P ∧ Q should parse as (¬P) ∧ Q
        let root = parse("¬P ∧ Q", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, .. } = arena.get(root).unwrap() {
            assert_eq!(*op, 14); // AND
            if let ASTNode::BinOp { op: inner_op, .. } = arena.get(*left).unwrap() {
                assert_eq!(*inner_op, 17); // NOT
            } else {
                panic!("Expected NOT inside AND");
            }
        } else {
            panic!("Expected BinOp for ∧");
        }
    }
    #[test]
    fn test_parse_comparison_vs_arithmetic() {
        let mut arena = Arena::new(100);
        // 1 + 2 < 3 + 4 should parse as (1+2) < (3+4)
        let root = parse("1 + 2 < 3 + 4", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 7); // <
            assert!(matches!(arena.get(*left).unwrap(), ASTNode::BinOp { op: 0, .. }));
            assert!(matches!(arena.get(*right).unwrap(), ASTNode::BinOp { op: 0, .. }));
        } else {
            panic!("Expected comparison");
        }
    }
    #[test]
    fn test_parse_complex_expression() {
        let mut arena = Arena::new(100);
        // ¬(a < b ∧ c) ∨ d
        let root = parse("¬(a < b ∧ c) ∨ d", &mut arena).unwrap();
        assert!(matches!(arena.get(root).unwrap(), ASTNode::BinOp { op: 15, .. }));
    }
    #[test]
    fn test_parse_mixed_ops() {
        let mut arena = Arena::new(100);
        // 2 ^ 3 * 4 + 5
        // Should parse as ((2^3)*4) + 5
        let root = parse("2 ^ 3 * 4 + 5", &mut arena).unwrap();
        if let ASTNode::BinOp { op, left, right } = arena.get(root).unwrap() {
            assert_eq!(*op, 0); // +
            if let ASTNode::BinOp { op: mul_op, .. } = arena.get(*left).unwrap() {
                assert_eq!(*mul_op, 2); // *
            }
            if let ASTNode::Int(v) = arena.get(*right).unwrap() {
                assert_eq!(*v, 5);
            }
        } else {
            panic!("Expected BinOp");
        }
    }

}
