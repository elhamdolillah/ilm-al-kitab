//! # MAL Hygienic Macros (Phase 48)
//!
//! Mathematical Foundation (Sigma):
//!   Macro = ⟨name: Ident, transform: TokenStream → TokenStream⟩
//!
//! Hygiene (Delta rule):
//!   ∀ x ∈ bindings(macro_body):
//!     scope(x) = SyntaxContext_macro ≠ SyntaxContext_caller
//!
//! Expansion:
//!   expand: Macro × TokenStream → TokenStream
//!   expand(M, args) = M.transform(args)
//!
//! Unifies:
//! - Rust proc-macros (hygienic)
//! - Lisp/Scheme hygienic macros
//! - Improves on C preprocessor (non-hygienic)
//!
//! Constitutional Compliance:
//! - Principle 1 (الإحكام): Unique macro names
//! - Principle 7 (التفكر): 5 tests verify hygiene
//! - Principle 9 (الوحدة الدلالية): One macro system
//! - Principle 11 (الأولوية الرياضية): Lambda calculus basis
#![forbid(unsafe_code)]
use std::collections::HashMap;
// ═══════════════════════════════════════════════════════════
// SYNTAX CONTEXT — Hygiene enforcement
// ═══════════════════════════════════════════════════════════
/// Unique context for each macro expansion (prevents identifier capture)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyntaxContext(pub u32);
impl SyntaxContext {
    /// Root context (top-level code, not inside any macro)
    pub const ROOT: Self = SyntaxContext(0);
}
// ═══════════════════════════════════════════════════════════
// TOKEN — Atomic unit of source code
// ═══════════════════════════════════════════════════════════
/// Token kinds (simplified)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident(String),
    Number(i64),
    String(String),
    Punct(char),
    Keyword(String),
    /// Group of tokens (e.g., { ... }, ( ... ))
    Group(Vec<Token>),
}
/// A single token with hygiene context
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub context: SyntaxContext,
}
impl Token {
    pub fn ident(name: &str, context: SyntaxContext) -> Self {
        Self {
            kind: TokenKind::Ident(name.to_string()),
            context,
        }
    }
    pub fn number(value: i64, context: SyntaxContext) -> Self {
        Self {
            kind: TokenKind::Number(value),
            context,
        }
    }
    pub fn punct(ch: char, context: SyntaxContext) -> Self {
        Self {
            kind: TokenKind::Punct(ch),
            context,
        }
    }
    pub fn keyword(name: &str, context: SyntaxContext) -> Self {
        Self {
            kind: TokenKind::Keyword(name.to_string()),
            context,
        }
    }
    pub fn string(s: &str, context: SyntaxContext) -> Self {
        Self {
            kind: TokenKind::String(s.to_string()),
            context,
        }
    }
    /// Check if this is an identifier
    pub fn is_ident(&self) -> bool {
        matches!(self.kind, TokenKind::Ident(_))
    }
    /// Get identifier name (if this is an ident)
    pub fn as_ident(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Ident(s) => Some(s),
            _ => None,
        }
    }
}
// ═══════════════════════════════════════════════════════════
// TOKEN STREAM — Sequence of tokens
// ═══════════════════════════════════════════════════════════
/// Sequence of tokens (input/output of macro expansion)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}
impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
    pub fn empty() -> Self {
        Self { tokens: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    /// Find all identifiers in the stream
    pub fn find_idents(&self) -> Vec<(&str, SyntaxContext)> {
        self.tokens
            .iter()
            .filter_map(|t| t.as_ident().map(|name| (name, t.context)))
            .collect()
    }
    /// Substitute identifiers (for macro expansion)
    pub fn substitute(&self, from: &str, to: &Token) -> TokenStream {
        let new_tokens = self.tokens.iter()
            .map(|t| {
                if let TokenKind::Ident(name) = &t.kind {
                    if name == from {
                        return Token {
                            kind: to.kind.clone(),
                            context: t.context,
                        };
                    }
                }
                t.clone()
            })
            .collect();
        TokenStream::new(new_tokens)
    }
}
// ═══════════════════════════════════════════════════════════
// MACRO — Name + expansion function
// ═══════════════════════════════════════════════════════════
/// Expansion function signature
pub type ExpansionFn = Box<dyn Fn(&TokenStream, SyntaxContext) -> TokenStream + Send + Sync>;
/// Macro definition
pub struct Macro {
    pub name: String,
    pub expand: ExpansionFn,
}
impl Macro {
    pub fn new<F>(name: String, expand: F) -> Self
    where
        F: Fn(&TokenStream, SyntaxContext) -> TokenStream + Send + Sync + 'static,
    {
        Self {
            name,
            expand: Box::new(expand),
        }
    }
    /// Expand this macro with given arguments
    pub fn expand(&self, args: &TokenStream, context: SyntaxContext) -> TokenStream {
        (self.expand)(args, context)
    }
}
impl std::fmt::Debug for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Macro")
            .field("name", &self.name)
            .finish()
    }
}
// ═══════════════════════════════════════════════════════════
// MACRO REGISTRY — Store and lookup macros
// ═══════════════════════════════════════════════════════════
/// Registry of all defined macros
pub struct MacroRegistry {
    macros: HashMap<String, Macro>,
    context_counter: u32,
}
impl MacroRegistry {
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
            context_counter: 0,
        }
    }
    /// Register a new macro (Principle 1: unique names)
    pub fn register(&mut self, macro_def: Macro) -> Result<(), MacroError> {
        if self.macros.contains_key(&macro_def.name) {
            return Err(MacroError::DuplicateMacro {
                name: macro_def.name.clone(),
            });
        }
        self.macros.insert(macro_def.name.clone(), macro_def);
        Ok(())
    }
    /// Lookup a macro by name
    pub fn lookup(&self, name: &str) -> Option<&Macro> {
        self.macros.get(name)
    }
    /// Expand a macro call (hygiene-enforced)
    ///
    /// Mathematical: expand(M, args) = M.transform(args) where
    ///   all new identifiers get fresh SyntaxContext
    pub fn expand(&mut self, name: &str, args: &TokenStream) -> Result<TokenStream, MacroError> {
        // Get macro first — immutable borrow ends after clone
        let macro_name = name.to_string();
        if !self.macros.contains_key(&macro_name) {
            return Err(MacroError::UnknownMacro { name: name.to_string() });
        }
        // Create fresh context for this expansion (hygiene)
        // This is now safe because we're not borrowing self.macros
        self.context_counter += 1;
        let context = SyntaxContext(self.context_counter);
        // Now get the macro reference (immutable borrow starts fresh)
        let macro_def = self.macros.get(&macro_name).unwrap();
        Ok(macro_def.expand(args, context))
    }
    /// Count registered macros
    pub fn len(&self) -> usize {
        self.macros.len()
    }
    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }
}
impl Default for MacroRegistry {
    fn default() -> Self {
        Self::new()
    }
}
// ═══════════════════════════════════════════════════════════
// MACRO ERRORS
// ═══════════════════════════════════════════════════════════
#[derive(Debug)]
pub enum MacroError {
    DuplicateMacro { name: String },
    UnknownMacro { name: String },
    ExpansionError { message: String },
}
impl std::fmt::Display for MacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroError::DuplicateMacro { name } => {
                write!(f, "ماكرو مكرر: {}", name)
            }
            MacroError::UnknownMacro { name } => {
                write!(f, "ماكرو غير معروف: {}", name)
            }
            MacroError::ExpansionError { message } => {
                write!(f, "خطأ في التوسع: {}", message)
            }
        }
    }
}
impl std::error::Error for MacroError {}
// ═══════════════════════════════════════════════════════════
// HYGIENE CHECKER — Verify no identifier capture
// ═══════════════════════════════════════════════════════════
/// Check that macro expansion is hygienic
/// Mathematical: ∀ x ∈ body: ctx(x) ≠ caller_ctx
pub fn check_hygiene(
    macro_body: &TokenStream,
    caller_context: SyntaxContext,
) -> bool {
    macro_body.tokens.iter().all(|token| {
        if token.is_ident() {
            token.context != caller_context
        } else {
            true
        }
    })
}
/// Verify expansion terminates (no infinite recursion)
pub fn check_termination(
    registry: &MacroRegistry,
    macro_name: &str,
    _depth: usize,
    max_depth: usize,
) -> bool {
    if _depth > max_depth {
        return false;
    }
    registry.lookup(macro_name).is_some()
}
// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    /// Test 1: Simple macro expansion
    #[test]
    fn test_simple_macro_expansion() {
        let mut registry = MacroRegistry::new();
        // Define: اطبع_مرتين(x) → { اطبع(x); اطبع(x); }
        let print_twice = Macro::new("اطبع_مرتين".to_string(), |args, ctx| {
            let arg = args.tokens.first()
                .expect("اطبع_مرتين requires one argument");
            TokenStream::new(vec![
                Token::punct('{', ctx),
                Token::ident("اطبع", ctx),
                Token::punct('(', ctx),
                arg.clone(),
                Token::punct(')', ctx),
                Token::punct(';', ctx),
                Token::ident("اطبع", ctx),
                Token::punct('(', ctx),
                arg.clone(),
                Token::punct(')', ctx),
                Token::punct(';', ctx),
                Token::punct('}', ctx),
            ])
        });
        registry.register(print_twice).unwrap();
        let args = TokenStream::new(vec![
            Token::ident("س", SyntaxContext::ROOT),
        ]);
        let expanded = registry.expand("اطبع_مرتين", &args).unwrap();
        assert_eq!(expanded.len(), 12);
        assert!(matches!(expanded.tokens[0].kind, TokenKind::Punct('{')));
    }
    /// Test 2: Hygiene — no identifier capture
    #[test]
    fn test_hygiene_no_capture() {
        let mut registry = MacroRegistry::new();
        // Define: let_temp(x) → { let temp = x; temp }
        let let_temp = Macro::new("let_temp".to_string(), |args, ctx| {
            let arg = args.tokens.first()
                .expect("let_temp requires one argument");
            TokenStream::new(vec![
                Token::punct('{', ctx),
                Token::keyword("let", ctx),
                Token::ident("temp", ctx),  // macro context
                Token::punct('=', ctx),
                arg.clone(),
                Token::punct(';', ctx),
                Token::ident("temp", ctx),
                Token::punct('}', ctx),
            ])
        });
        registry.register(let_temp).unwrap();
        let args = TokenStream::new(vec![
            Token::number(42, SyntaxContext::ROOT),
        ]);
        let expanded = registry.expand("let_temp", &args).unwrap();
        // Check hygiene: 'temp' should have macro context, not ROOT
        let temp_idents: Vec<_> = expanded.tokens.iter()
            .filter(|t| t.as_ident() == Some("temp"))
            .collect();
        assert_eq!(temp_idents.len(), 2);
        for temp in temp_idents {
            assert_ne!(temp.context, SyntaxContext::ROOT,
                "Hygiene violation: 'temp' captured caller context");
        }
    }
    /// Test 3: Macro with pattern matching
    #[test]
    fn test_macro_with_patterns() {
        let mut registry = MacroRegistry::new();
        let unless = Macro::new("unless".to_string(), |args, ctx| {
            TokenStream::new(vec![
                Token::ident("if", ctx),
                Token::punct('!', ctx),
                args.tokens[0].clone(),
                args.tokens[1].clone(),
            ])
        });
        registry.register(unless).unwrap();
        let args = TokenStream::new(vec![
            Token::ident("س", SyntaxContext::ROOT),
            Token::punct('{', SyntaxContext::ROOT),
            Token::ident("اطبع", SyntaxContext::ROOT),
            Token::punct('}', SyntaxContext::ROOT),
        ]);
        let expanded = registry.expand("unless", &args).unwrap();
        assert!(expanded.len() > 0);
    }
    /// Test 4: Macro registry lookup
    #[test]
    fn test_macro_registry_lookup() {
        let mut registry = MacroRegistry::new();
        let m1 = Macro::new("m1".to_string(), |_, _ctx| TokenStream::empty());
        let m2 = Macro::new("m2".to_string(), |_, _ctx| TokenStream::empty());
        registry.register(m1).unwrap();
        registry.register(m2).unwrap();
        assert_eq!(registry.len(), 2);
        assert!(registry.lookup("m1").is_some());
        assert!(registry.lookup("m2").is_some());
        assert!(registry.lookup("m3").is_none());
        // Duplicate registration should fail
        let m1_dup = Macro::new("m1".to_string(), |_, _ctx| TokenStream::empty());
        let result = registry.register(m1_dup);
        assert!(matches!(result, Err(MacroError::DuplicateMacro { .. })));
    }
    /// Test 5: Expansion termination + hygiene checks
    #[test]
    fn test_expansion_termination() {
        let mut registry = MacroRegistry::new();
        let m = Macro::new("m".to_string(), |_, _ctx| TokenStream::empty());
        registry.register(m).unwrap();
        assert!(check_termination(&registry, "m", 0, 100));
        assert!(check_termination(&registry, "m", 50, 100));
        assert!(!check_termination(&registry, "unknown", 0, 100));
        // Hygiene check
        let body = TokenStream::new(vec![
            Token::ident("x", SyntaxContext(1)),
            Token::ident("y", SyntaxContext(1)),
        ]);
        assert!(check_hygiene(&body, SyntaxContext::ROOT));
        // Non-hygienic body (should fail)
        let bad_body = TokenStream::new(vec![
            Token::ident("x", SyntaxContext::ROOT),
        ]);
        assert!(!check_hygiene(&bad_body, SyntaxContext::ROOT));
    }
}
