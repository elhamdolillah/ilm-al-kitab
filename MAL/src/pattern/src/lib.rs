//! # MAL Pattern Matching — Phase 43
//!
//! `⎇ value : pattern₁ ⇒ expr₁ pattern₂ ⇒ expr₂ _ ⇒ default`
//!
//! Ported semantics from Rust (`match`), Erlang (`case`), Haskell (`case of`).
#![forbid(unsafe_code)]
use mal_arena::{Arena, ASTNode, NodeID, Pattern};
use mal_ownership::StringTable;
/// Pattern matching errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchError {
    /// No arm matched the scrutinee.
    NoMatch { value: String },
    /// Pattern is malformed.
    InvalidPattern { reason: String },
}
impl std::fmt::Display for MatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchError::NoMatch { value } => {
                write!(f, "خطأ مطابقة: لا يوجد نمط يطابق القيمة '{}'", value)
            }
            MatchError::InvalidPattern { reason } => {
                write!(f, "خطأ مطابقة: نمط غير صالح — {}", reason)
            }
        }
    }
}
impl std::error::Error for MatchError {}
/// Value produced by evaluating an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Text(String),
    Bool(bool),
    Unknown,
}
/// Evaluate an AST node to a Value.
pub fn eval_node(
    node_id: NodeID,
    arena: &Arena,
    string_table: &StringTable,
    bindings: &std::collections::HashMap<String, Value>,
) -> Value {
    if let Ok(node) = arena.get(node_id) {
        match node {
            ASTNode::Int(n) => Value::Int(*n),
            ASTNode::BoolLit(b) => Value::Bool(*b),
            ASTNode::Str(idx) => {
                let s = string_table.get(*idx).unwrap_or("").to_string();
                Value::Text(s)
            }
            ASTNode::Ident(idx) => {
                if let Some(name) = string_table.get(*idx) {
                    bindings.get(name).cloned().unwrap_or(Value::Unknown)
                } else {
                    Value::Unknown
                }
            }
            ASTNode::BinOp { op, left, right } => {
                use mal_arena::BinaryOp;
                let lv = eval_node(*left, arena, string_table, bindings);
                let rv = eval_node(*right, arena, string_table, bindings);
                match (op, lv, rv) {
                    (BinaryOp::Add, Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                    (BinaryOp::Sub, Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                    (BinaryOp::Mul, Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                    (BinaryOp::Eq, Value::Int(a), Value::Int(b)) => Value::Bool(a == b),
                    _ => Value::Unknown,
                }
            }
            _ => Value::Unknown,
        }
    } else {
        Value::Unknown
    }
}
/// Try to match a Value against a Pattern.
pub fn match_pattern(
    pattern: &Pattern,
    value: &Value,
    arena: &Arena,
    string_table: &StringTable,
) -> Option<std::collections::HashMap<String, Value>> {
    let mut new_bindings = std::collections::HashMap::new();
    match pattern {
        Pattern::Wildcard => Some(new_bindings),
        Pattern::Literal(lit_id) => {
            let lit_val = eval_node(*lit_id, arena, string_table, &std::collections::HashMap::new());
            if &lit_val == value {
                Some(new_bindings)
            } else {
                None
            }
        }
        Pattern::Binding(name_idx) => {
            if let Some(name) = string_table.get(*name_idx) {
                new_bindings.insert(name.to_string(), value.clone());
                Some(new_bindings)
            } else {
                None
            }
        }
    }
}
/// Evaluate a Match expression.
pub fn eval_match(
    match_node_id: NodeID,
    arena: &Arena,
    string_table: &StringTable,
    outer_bindings: &std::collections::HashMap<String, Value>,
) -> Result<Value, MatchError> {
    let node = arena.get(match_node_id).map_err(|_| MatchError::InvalidPattern {
        reason: "invalid match node".to_string(),
    })?;
    let (scrutinee_id, arms_id) = match node {
        ASTNode::Match { scrutinee, arms } => (*scrutinee, *arms),
        _ => return Err(MatchError::InvalidPattern { reason: "not a Match node".to_string() }),
    };
    let scrutinee_val = eval_node(scrutinee_id, arena, string_table, outer_bindings);
    let mut current_arm = arms_id;
    while current_arm != NodeID::INVALID {
        if let Ok(ASTNode::List { head, tail }) = arena.get(current_arm) {
            let arm_id = *head;
            let next = *tail;
            if let Ok(ASTNode::MatchArm { pattern, body }) = arena.get(arm_id) {
                if let Some(new_bindings) = match_pattern(pattern, &scrutinee_val, arena, string_table) {
                    let mut combined = outer_bindings.clone();
                    combined.extend(new_bindings);
                    let result = eval_node(*body, arena, string_table, &combined);
                    return Ok(result);
                }
            }
            current_arm = next;
        } else {
            break;
        }
    }
    Err(MatchError::NoMatch { value: format!("{:?}", scrutinee_val) })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn build_match_with_literal_and_wildcard(
        arena: &mut Arena,
        string_table: &mut StringTable,
        scrutinee_val: i64,
        literal_val: i64,
        body_val: i64,
        default_val: i64,
    ) -> NodeID {
        let scrutinee = arena.allocate(ASTNode::Int(scrutinee_val)).unwrap();
        let lit_pat_node = arena.allocate(ASTNode::Int(literal_val)).unwrap();
        let body1 = arena.allocate(ASTNode::Int(body_val)).unwrap();
        let arm1 = arena.allocate(ASTNode::MatchArm {
            pattern: Pattern::Literal(lit_pat_node),
            body: body1,
        }).unwrap();
        let body2 = arena.allocate(ASTNode::Int(default_val)).unwrap();
        let arm2 = arena.allocate(ASTNode::MatchArm {
            pattern: Pattern::Wildcard,
            body: body2,
        }).unwrap();
        let arms_tail = arena.allocate(ASTNode::List { head: arm2, tail: NodeID::INVALID }).unwrap();
        let arms = arena.allocate(ASTNode::List { head: arm1, tail: arms_tail }).unwrap();
        arena.allocate(ASTNode::Match { scrutinee, arms }).unwrap()
    }
    #[test]
    fn test_match_int_literal() {
        let mut arena = Arena::new(100);
        let mut st = StringTable::new();
        let bindings = std::collections::HashMap::new();
        let m = build_match_with_literal_and_wildcard(&mut arena, &mut st, 5, 5, 1, 0);
        let result = eval_match(m, &arena, &st, &bindings).unwrap();
        assert_eq!(result, Value::Int(1));
    }
    #[test]
    fn test_match_wildcard() {
        let mut arena = Arena::new(100);
        let mut st = StringTable::new();
        let bindings = std::collections::HashMap::new();
        let m = build_match_with_literal_and_wildcard(&mut arena, &mut st, 99, 5, 1, 42);
        let result = eval_match(m, &arena, &st, &bindings).unwrap();
        assert_eq!(result, Value::Int(42));
    }
    #[test]
    fn test_match_binding() {
        let mut arena = Arena::new(100);
        let mut st = StringTable::new();
        let bindings = std::collections::HashMap::new();
        let scrutinee = arena.allocate(ASTNode::Int(7)).unwrap();
        let س_idx = st.intern("س");
        let binding_id = arena.allocate(ASTNode::Ident(س_idx)).unwrap();
        let one = arena.allocate(ASTNode::Int(1)).unwrap();
        let body = arena.allocate(ASTNode::BinOp {
            op: mal_arena::BinaryOp::Add,
            left: binding_id,
            right: one,
        }).unwrap();
        let arm = arena.allocate(ASTNode::MatchArm {
            pattern: Pattern::Binding(س_idx),
            body,
        }).unwrap();
        let arms = arena.allocate(ASTNode::List { head: arm, tail: NodeID::INVALID }).unwrap();
        let m = arena.allocate(ASTNode::Match { scrutinee, arms }).unwrap();
        let result = eval_match(m, &arena, &st, &bindings).unwrap();
        assert_eq!(result, Value::Int(8));
    }
    #[test]
    fn test_match_multi_arm_fallthrough() {
        let mut arena = Arena::new(100);
        let mut st = StringTable::new();
        let bindings = std::collections::HashMap::new();
        let scrutinee = arena.allocate(ASTNode::Int(3)).unwrap();
        let lit1_node = arena.allocate(ASTNode::Int(1)).unwrap();
        let body1 = arena.allocate(ASTNode::Int(10)).unwrap();
        let arm1 = arena.allocate(ASTNode::MatchArm {
            pattern: Pattern::Literal(lit1_node),
            body: body1,
        }).unwrap();
        let lit2_node = arena.allocate(ASTNode::Int(2)).unwrap();
        let body2 = arena.allocate(ASTNode::Int(20)).unwrap();
        let arm2 = arena.allocate(ASTNode::MatchArm {
            pattern: Pattern::Literal(lit2_node),
            body: body2,
        }).unwrap();
        let body3 = arena.allocate(ASTNode::Int(30)).unwrap();
        let arm3 = arena.allocate(ASTNode::MatchArm {
            pattern: Pattern::Wildcard,
            body: body3,
        }).unwrap();
        let tail3 = arena.allocate(ASTNode::List { head: arm3, tail: NodeID::INVALID }).unwrap();
        let tail2 = arena.allocate(ASTNode::List { head: arm2, tail: tail3 }).unwrap();
        let arms = arena.allocate(ASTNode::List { head: arm1, tail: tail2 }).unwrap();
        let m = arena.allocate(ASTNode::Match { scrutinee, arms }).unwrap();
        let result = eval_match(m, &arena, &st, &bindings).unwrap();
        assert_eq!(result, Value::Int(30));
    }
    #[test]
    fn test_match_no_match_error() {
        let mut arena = Arena::new(100);
        let mut st = StringTable::new();
        let bindings = std::collections::HashMap::new();
        let scrutinee = arena.allocate(ASTNode::Int(5)).unwrap();
        let lit_node = arena.allocate(ASTNode::Int(6)).unwrap();
        let body = arena.allocate(ASTNode::Int(1)).unwrap();
        let arm = arena.allocate(ASTNode::MatchArm {
            pattern: Pattern::Literal(lit_node),
            body,
        }).unwrap();
        let arms = arena.allocate(ASTNode::List { head: arm, tail: NodeID::INVALID }).unwrap();
        let m = arena.allocate(ASTNode::Match { scrutinee, arms }).unwrap();
        let result = eval_match(m, &arena, &st, &bindings);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MatchError::NoMatch { .. }));
    }
}
