//! MAL Lexer — deterministic tokenizer for the Mathematical Arabic Language.
//!
//! ## Declared scope
//! - ASCII digits only for numbers (matches reference lexer).
//! - Double-quoted strings, no escapes.
//! - Alphabetic (Unicode) identifiers; digits/underscore allowed after first char.
//! - Fixed single-char symbol table; symbols checked before identifiers.
//! - No comments in this scope.
//!
//! ## Constitutional compliance
//! - `#![forbid(unsafe_code)]`
//! - No per-token heap allocation (byte ranges into source only).
//! - No time, no randomness, no network.
//! - Fail-closed: invalid input yields `LexerError`, never panic.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Kind of a lexical token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Integer literal (payload in `Token::num`).
    Num,
    /// Identifier (byte range into source).
    Ident,
    /// String literal including quotes (byte range into source).
    Str,
    /// `≔` assignment.
    Assign,
    /// `≡` definition.
    Define,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `·` (also `*` and `×`).
    Mul,
    /// `⊕` concatenation.
    Concat,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `,` or `،`
    Comma,
    /// `:`
    Colon,
    /// `.`
    Dot,
    /// `⟨`
    LAngle,
    /// `⟩`
    RAngle,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `=`
    Eq,
    /// `≠`
    Neq,
    /// `؟`
    Question,
    /// `∀`
    Forall,
    /// `∃`
    Exists,
    /// `∈`
    In,
    /// `μ`
    Mu,
    /// `λ`
    Lambda,
    /// `⊸` linear implication (ownership transfer)
    LinearImplication,
    /// `﴿`
    LBlock,
    /// `﴾`
    RBlock,
    /// `⋄`
    Diamond,
    /// `⊸` move.
    Move,
    /// `⊙` read stdin.
    Read,
    /// `⎕` print.
    Print,
    /// End of input.
    Eof,
}

/// A token: kind + byte range (or numeric payload) + position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// Token kind.
    pub kind: TokenKind,
    /// Start byte offset in source (meaningful for Ident/Str).
    pub start: u32,
    /// Byte length (meaningful for Ident/Str).
    pub len: u32,
    /// Numeric payload (meaningful for Num).
    pub num: i64,
    /// 1-based line number.
    pub line: u32,
    /// 1-based column number (in characters).
    pub col: u32,
}

/// Lexer failure modes (fail-closed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexerError {
    /// Character not part of the language.
    UnexpectedChar {
        /// Line of the offending character.
        line: u32,
        /// Column of the offending character.
        col: u32,
    },
    /// String literal without closing quote.
    UnterminatedString {
        /// Line where the string started.
        line: u32,
        /// Column where the string started.
        col: u32,
    },
    /// Integer literal exceeds i64.
    NumberOverflow {
        /// Line where the number started.
        line: u32,
        /// Column where the number started.
        col: u32,
    },
}

fn symbol_kind(c: char) -> Option<TokenKind> {
    Some(match c {
        '≔' => TokenKind::Assign,
        '≡' => TokenKind::Define,
        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        '·' | '*' | '×' => TokenKind::Mul,
        '⊕' => TokenKind::Concat,
        '(' => TokenKind::LParen,
        ')' => TokenKind::RParen,
        ',' | '،' => TokenKind::Comma,
        ':' => TokenKind::Colon,
        '.' => TokenKind::Dot,
        '⟨' => TokenKind::LAngle,
        '⟩' => TokenKind::RAngle,
        '<' => TokenKind::Lt,
        '>' => TokenKind::Gt,
        '=' => TokenKind::Eq,
        '≠' => TokenKind::Neq,
        '؟' => TokenKind::Question,
        '∀' => TokenKind::Forall,
        '∃' => TokenKind::Exists,
        '∈' => TokenKind::In,
        'μ' => TokenKind::Mu,
        'λ' => TokenKind::Lambda,
        '﴿' => TokenKind::LBlock,
        '﴾' => TokenKind::RBlock,
        '⋄' => TokenKind::Diamond,
        '⊸' => TokenKind::LinearImplication,
        '⊙' => TokenKind::Read,
        '⎕' => TokenKind::Print,
        _ => return None,
    })
}

/// Deterministic lexer over a UTF-8 source string.
pub struct Lexer<'a> {
    src: &'a str,
    rest: &'a str,
    line: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    /// Create a lexer for the given source.
    pub fn new(src: &'a str) -> Self {
        Self { src, rest: src, line: 1, col: 1 }
    }

    fn peek(&self) -> Option<char> {
        self.rest.chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.rest.chars().next()?;
        self.rest = &self.rest[c.len_utf8()..];
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn offset(&self) -> u32 {
        (self.src.len() - self.rest.len()) as u32
    }

    /// Tokenize the whole source. Fail-closed on any invalid input.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut out: Vec<Token> = Vec::new();
        loop {
            while let Some(c) = self.peek() {
                if c.is_whitespace() {
                    self.bump();
                } else {
                    break;
                }
            }
            let (line, col) = (self.line, self.col);
            let start = self.offset();
            let Some(c) = self.peek() else {
                out.push(Token { kind: TokenKind::Eof, start, len: 0, num: 0, line, col });
                return Ok(out);
            };
            if c.is_ascii_digit() {
                let mut num: i64 = 0;
                while let Some(d) = self.peek() {
                    if !d.is_ascii_digit() {
                        break;
                    }
                    self.bump();
                    let dv = i64::from(d as u32 - '0' as u32);
                    num = num
                        .checked_mul(10)
                        .and_then(|v| v.checked_add(dv))
                        .ok_or(LexerError::NumberOverflow { line, col })?;
                }
                out.push(Token { kind: TokenKind::Num, start, len: self.offset() - start, num, line, col });
                continue;
            }
            if c == '"' {
                self.bump();
                loop {
                    match self.peek() {
                        None => return Err(LexerError::UnterminatedString { line, col }),
                        Some('"') => {
                            self.bump();
                            break;
                        }
                        Some(_) => {
                            self.bump();
                        }
                    }
                }
                out.push(Token { kind: TokenKind::Str, start, len: self.offset() - start, num: 0, line, col });
                continue;
            }
            if let Some(kind) = symbol_kind(c) {
                self.bump();
                out.push(Token { kind, start, len: self.offset() - start, num: 0, line, col });
                continue;
            }
            if c.is_alphabetic() || c == '_' {
                while let Some(d) = self.peek() {
                    if d.is_alphabetic() || d == '_' || d.is_ascii_digit() {
                        self.bump();
                    } else {
                        break;
                    }
                }
                out.push(Token { kind: TokenKind::Ident, start, len: self.offset() - start, num: 0, line, col });
                continue;
            }
            return Err(LexerError::UnexpectedChar { line, col });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        Lexer::new(src).tokenize().unwrap().into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_tokenize_assignment() {
        assert_eq!(
            kinds("أ ≔ 5"),
            vec![TokenKind::Ident, TokenKind::Assign, TokenKind::Num, TokenKind::Eof]
        );
    }

    #[test]
    fn test_tokenize_lambda_define() {
        assert_eq!(
            kinds("ضعف ≡ λس. س · 2"),
            vec![
                TokenKind::Ident, TokenKind::Define, TokenKind::Lambda, TokenKind::Ident,
                TokenKind::Dot, TokenKind::Ident, TokenKind::Mul, TokenKind::Num, TokenKind::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_print_call() {
        assert_eq!(
            kinds("⎕ ضعف(5)"),
            vec![
                TokenKind::Print, TokenKind::Ident, TokenKind::LParen,
                TokenKind::Num, TokenKind::RParen, TokenKind::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_block_and_list() {
        assert_eq!(
            kinds("﴿ أ ⋄ ب ﴾ ⟨1،2⟩"),
            vec![
                TokenKind::LBlock, TokenKind::Ident, TokenKind::Diamond, TokenKind::Ident, TokenKind::RBlock,
                TokenKind::LAngle, TokenKind::Num, TokenKind::Comma, TokenKind::Num, TokenKind::RAngle,
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn test_ident_and_str_ranges() {
        let src = "سلام ≔ \"مرحبا\"";
        let toks = Lexer::new(src).tokenize().unwrap();
        let ident = &toks[0];
        assert_eq!(ident.kind, TokenKind::Ident);
        assert_eq!(&src[ident.start as usize..(ident.start + ident.len) as usize], "سلام");
        let s = &toks[2];
        assert_eq!(s.kind, TokenKind::Str);
        assert_eq!(&src[s.start as usize..(s.start + s.len) as usize], "\"مرحبا\"");
    }

    #[test]
    fn test_line_col_tracking() {
        let toks = Lexer::new("أ ≔ 1\n⎕ أ").tokenize().unwrap();
        assert_eq!((toks[0].line, toks[0].col), (1, 1));
        assert_eq!((toks[3].line, toks[3].col), (2, 1));
        assert_eq!((toks[4].line, toks[4].col), (2, 3));
    }

    #[test]
    fn test_unterminated_string_abstain() {
        let err = Lexer::new("س ≔ \"نص").tokenize().unwrap_err();
        assert!(matches!(err, LexerError::UnterminatedString { .. }));
    }

    #[test]
    fn test_unexpected_char_abstain() {
        let err = Lexer::new("أ ≔ 5 @").tokenize().unwrap_err();
        assert!(matches!(err, LexerError::UnexpectedChar { .. }));
    }

    #[test]
    fn test_number_overflow_abstain() {
        let err = Lexer::new("9999999999999999999999").tokenize().unwrap_err();
        assert!(matches!(err, LexerError::NumberOverflow { .. }));
    }
}
// ═══════════════════════════════════════════════════════════════
// Mathematical Semantics — Lexer
// ═══════════════════════════════════════════════════════════════
//
// Lexer: L: String → List[Token]
//
// Token Types:
//   Token = {type: TokenType, value: String, span: (usize, usize)}
//   TokenType ∈ {Int, String, Ident, Operator, Keyword, EOF}
//
// Lexing Rules:
//   ∀ input s: L(s) = [t₁, t₂, ..., tₙ] where:
//   - ∀ i: tᵢ.type ∈ TokenType
//   - ∀ i: tᵢ.span ⊆ [0, |s|)
//   - Σ(|tᵢ.value| for i ∈ [1,n]) = |s| (complete coverage)
//   - ∀ i < j: tᵢ.span.end ≤ tⱼ.span.start (no overlap)
//
// Determinism:
//   ∀ input s: L(s) is identical across all calls
//   SHA-256(tokens) is invariant
//
// Error Handling:
//   ∀ invalid token t: raise LexerError(t.span, t.value)
//   No silent failures (fail-closed)
// ═══════════════════════════════════════════════════════════════
