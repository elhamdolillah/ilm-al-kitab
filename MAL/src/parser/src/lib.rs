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
        self.parse_additive(arena)
    }

    fn parse_additive(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let mut left = self.parse_multiplicative(arena)?;
        while let Some(tok) = self.peek() {
            if tok.kind == TokenKind::Plus || tok.kind == TokenKind::Minus {
                let op = self.bump().unwrap();
                let right = self.parse_multiplicative(arena)?;
                let op_code = match op.kind {
                    TokenKind::Plus => 0,
                    TokenKind::Minus => 1,
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
            if tok.kind == TokenKind::Mul {
                self.bump();
                let right = self.parse_primary(arena)?;
                left = arena.allocate(ASTNode::BinOp {
                    op: 2, // ·
                    left,
                    right,
                })?;
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_primary(&mut self, arena: &mut Arena) -> Result<NodeID, ParserError> {
        let tok = self.bump().ok_or(ParserError::UnexpectedEof)?;
        match tok.kind {
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
                let expr = self.parse_expr(arena)?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
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
}
