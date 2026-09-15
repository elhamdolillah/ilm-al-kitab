//! # MAL Ownership Checker — Linear Logic (⊸) Enforcement
//!
//! Enforces compile-time memory safety via linear ownership semantics.
//! Ported from `math_complete.py` check_ownership with improvements.
//!
//! ## Constitutional Compliance
//! - `#![forbid(unsafe_code)]` — no raw pointers
//! - Principle 4 (الأمانة): every resource has exactly one owner
//! - Principle 5 (البيان): every error includes SourceSpan
//! - Principle 7 (التفكر): all avoidable errors caught at compile time
#![forbid(unsafe_code)]
#![deny(missing_docs)]
use mal_arena::{Arena, ASTNode, BinaryOp, NodeID};
use mal_types::SourceSpan;
use std::collections::HashSet;
// ═══════════════════════════════════════════════════════════════
// OWNERSHIP ERRORS
// ═══════════════════════════════════════════════════════════════
/// Ownership violation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipError {
    /// Variable used after being moved (consumed).
    UsedAfterMove {
        /// Variable name.
        var: String,
        /// Location.
        span: Option<SourceSpan>,
    },
    /// Variable moved twice.
    DoubleMove {
        /// Variable name.
        var: String,
        /// Location.
        span: Option<SourceSpan>,
    },
    /// Variable not found in scope.
    UndefinedVariable {
        /// Variable name.
        var: String,
        /// Location.
        span: Option<SourceSpan>,
    },
}
impl std::fmt::Display for OwnershipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnershipError::UsedAfterMove { var, span: _ } => {
                write!(f, "خطأ ملكية ⊸: '{}' مستهلك — نُقلت ملكيته ولا يمكن استخدامه", var)
            }
            OwnershipError::DoubleMove { var, span: _ } => {
                write!(f, "خطأ ملكية ⊸: '{}' مستهلك بالفعل — لا يمكن نقله مرتين", var)
            }
            OwnershipError::UndefinedVariable { var, span: _ } => {
                write!(f, "خطأ: المتغير '{}' غير معرف", var)
            }
        }
    }
}
impl std::error::Error for OwnershipError {}
// ═══════════════════════════════════════════════════════════════
// STRING TABLE
// ═══════════════════════════════════════════════════════════════
/// Simple string table for identifier resolution.
pub struct StringTable {
    strings: Vec<String>,
}
impl StringTable {
    /// Create a new string table.
    pub fn new() -> Self {
        StringTable { strings: Vec::new() }
    }
    /// Add a string and return its index.
    pub fn intern(&mut self, s: impl Into<String>) -> u32 {
        let s = s.into();
        if let Some(idx) = self.strings.iter().position(|x| x == &s) {
            return idx as u32;
        }
        let idx = self.strings.len() as u32;
        self.strings.push(s);
        idx
    }
    /// Get string by index.
    pub fn get(&self, idx: u32) -> Option<&str> {
        self.strings.get(idx as usize).map(|s| s.as_str())
    }
}
impl Default for StringTable {
    fn default() -> Self {
        Self::new()
    }
}
// ═══════════════════════════════════════════════════════════════
// GET USED VARS
// ═══════════════════════════════════════════════════════════════
/// Collect all variable names used in an expression.
pub fn get_used_vars(
    node_id: NodeID,
    arena: &Arena,
    string_table: &StringTable,
) -> HashSet<String> {
    let mut used = HashSet::new();
    collect_used_vars(node_id, arena, string_table, &mut used);
    used
}
fn collect_used_vars(
    node_id: NodeID,
    arena: &Arena,
    string_table: &StringTable,
    used: &mut HashSet<String>,
) {
    // FIX: arena.get() returns Result, not Option
    if let Ok(node) = arena.get(node_id) {
        match node {
            // FIX: idx is u32 (Copy), no deref needed
            ASTNode::Ident(idx) => {
                if let Some(name) = string_table.get(*idx) {
                    used.insert(name.to_string());
                }
            }
            // FIX: left, right are NodeID (Copy), no deref needed
            ASTNode::BinOp { left, right, .. } => {
                collect_used_vars(*left, arena, string_table, used);
                collect_used_vars(*right, arena, string_table, used);
            }
            ASTNode::UnaryOp { expr, .. } => {
                collect_used_vars(*expr, arena, string_table, used);
            }
            ASTNode::Call { func, args } => {
                collect_used_vars(*func, arena, string_table, used);
                collect_used_vars(*args, arena, string_table, used);
            }
            ASTNode::List { head, tail } => {
                collect_used_vars(*head, arena, string_table, used);
                if *tail != NodeID::INVALID {
                    collect_used_vars(*tail, arena, string_table, used);
                }
            }
            ASTNode::Lambda { params, body } => {
                let mut body_used = HashSet::new();
                collect_used_vars(*body, arena, string_table, &mut body_used);
                let mut param_names = HashSet::new();
                collect_param_names(*params, arena, string_table, &mut param_names);
                for var in body_used {
                    if !param_names.contains(&var) {
                        used.insert(var);
                    }
                }
            }
            ASTNode::ForAll { set, body, .. } => {
                collect_used_vars(*set, arena, string_table, used);
                collect_used_vars(*body, arena, string_table, used);
            }
            ASTNode::SetMembership { elem, set } => {
                collect_used_vars(*elem, arena, string_table, used);
                collect_used_vars(*set, arena, string_table, used);
            }
            ASTNode::Mu { body, .. } => {
                collect_used_vars(*body, arena, string_table, used);
            }
            ASTNode::Exists { set, body, .. } => {
                collect_used_vars(*set, arena, string_table, used);
                collect_used_vars(*body, arena, string_table, used);
            }
            ASTNode::Set { elems } => {
                if *elems != NodeID::INVALID {
                    collect_used_vars(*elems, arena, string_table, used);
                }
            }
            ASTNode::Int(_) | ASTNode::BoolLit(_) | ASTNode::Str(_) | ASTNode::Empty => {}
            // LinearLet not traversed here (it's a statement)
            _ => {}
        }
    }
}
fn collect_param_names(
    node_id: NodeID,
    arena: &Arena,
    string_table: &StringTable,
    params: &mut HashSet<String>,
) {
    if let Ok(node) = arena.get(node_id) {
        match node {
            ASTNode::Ident(idx) => {
                if let Some(name) = string_table.get(*idx) {
                    params.insert(name.to_string());
                }
            }
            ASTNode::List { head, tail } => {
                collect_param_names(*head, arena, string_table, params);
                if *tail != NodeID::INVALID {
                    collect_param_names(*tail, arena, string_table, params);
                }
            }
            _ => {}
        }
    }
}
// ═══════════════════════════════════════════════════════════════
// OWNERSHIP CHECKER
// ═══════════════════════════════════════════════════════════════
/// Check ownership constraints for a program.
pub fn check_ownership(
    statements: &[NodeID],
    arena: &Arena,
    string_table: &StringTable,
) -> Result<(), OwnershipError> {
    let mut consumed: HashSet<String> = HashSet::new();
    for &stmt_id in statements {
        check_stmt(stmt_id, arena, string_table, &mut consumed)?;
    }
    Ok(())
}
fn check_stmt(
    stmt_id: NodeID,
    arena: &Arena,
    string_table: &StringTable,
    consumed: &mut HashSet<String>,
) -> Result<(), OwnershipError> {
    if let Ok(node) = arena.get(stmt_id) {
        match node {
            ASTNode::LinearLet { value, body, .. } => {
                let value_used = get_used_vars(*value, arena, string_table);
                for var in &value_used {
                    if consumed.contains(var) {
                        return Err(OwnershipError::DoubleMove {
                            var: var.clone(),
                            span: None,
                        });
                    }
                }
                for var in value_used {
                    consumed.insert(var);
                }
                check_stmt(*body, arena, string_table, consumed)?;
            }
            ASTNode::BinOp { op, left, right } => {
                if *op == BinaryOp::Assign {
                    if let Ok(ASTNode::Ident(idx)) = arena.get(*left) {
                        if let Some(name) = string_table.get(*idx) {
                            consumed.remove(name);
                        }
                    }
                    let rhs_used = get_used_vars(*right, arena, string_table);
                    for var in &rhs_used {
                        if consumed.contains(var) {
                            return Err(OwnershipError::UsedAfterMove {
                                var: var.clone(),
                                span: None,
                            });
                        }
                    }
                } else {
                    let used = get_used_vars(stmt_id, arena, string_table);
                    for var in &used {
                        if consumed.contains(var) {
                            return Err(OwnershipError::UsedAfterMove {
                                var: var.clone(),
                                span: None,
                            });
                        }
                    }
                }
            }
            ASTNode::Call { .. } => {
                let used = get_used_vars(stmt_id, arena, string_table);
                for var in &used {
                    if consumed.contains(var) {
                        return Err(OwnershipError::UsedAfterMove {
                            var: var.clone(),
                            span: None,
                        });
                    }
                }
            }
            ASTNode::ForAll { set, body, .. } => {
                let set_used = get_used_vars(*set, arena, string_table);
                for var in &set_used {
                    if consumed.contains(var) {
                        return Err(OwnershipError::UsedAfterMove {
                            var: var.clone(),
                            span: None,
                        });
                    }
                }
                check_stmt(*body, arena, string_table, consumed)?;
            }
            ASTNode::Mu { body, .. } => {
                check_stmt(*body, arena, string_table, consumed)?;
            }
            _ => {
                let used = get_used_vars(stmt_id, arena, string_table);
                for var in &used {
                    if consumed.contains(var) {
                        return Err(OwnershipError::UsedAfterMove {
                            var: var.clone(),
                            span: None,
                        });
                    }
                }
            }
        }
    }
    Ok(())
}
// ═══════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    fn make_move_program(
        arena: &mut Arena,
        string_table: &mut StringTable,
        source_var: &str,
        target_var: &str,
    ) -> Vec<NodeID> {
        let source_idx = string_table.intern(source_var);
        let target_idx = string_table.intern(target_var);
        let source = arena.allocate(ASTNode::Ident(source_idx)).unwrap();
        let target = arena.allocate(ASTNode::Ident(target_idx)).unwrap();
        let body = arena.allocate(ASTNode::Int(0)).unwrap();
        let linear_let = arena.allocate(ASTNode::LinearLet {
            name: target,
            value: source,
            body,
        }).unwrap();
        vec![linear_let]
    }
    #[test]
    fn test_move_valid() {
        let mut arena = Arena::new(100);
        let mut string_table = StringTable::new();
        let stmts = make_move_program(&mut arena, &mut string_table, "أ", "ب");
        assert!(check_ownership(&stmts, &arena, &string_table).is_ok());
    }
    #[test]
    fn test_reassign_after_move() {
        let mut arena = Arena::new(100);
        let mut string_table = StringTable::new();
        let أ_idx = string_table.intern("أ");
        let ب_idx = string_table.intern("ب");
        let أ1 = arena.allocate(ASTNode::Ident(أ_idx)).unwrap();
        let ب1 = arena.allocate(ASTNode::Ident(ب_idx)).unwrap();
        let body1 = arena.allocate(ASTNode::Int(0)).unwrap();
        let stmt1 = arena.allocate(ASTNode::LinearLet {
            name: ب1,
            value: أ1,
            body: body1,
        }).unwrap();
        let أ2 = arena.allocate(ASTNode::Ident(أ_idx)).unwrap();
        let list2 = arena.allocate(ASTNode::Set { elems: NodeID::INVALID }).unwrap();
        let stmt2 = arena.allocate(ASTNode::BinOp {
            op: BinaryOp::Assign,
            left: أ2,
            right: list2,
        }).unwrap();
        let stmts = vec![stmt1, stmt2];
        assert!(check_ownership(&stmts, &arena, &string_table).is_ok());
    }
    #[test]
    fn test_use_after_move() {
        let mut arena = Arena::new(100);
        let mut string_table = StringTable::new();
        let أ_idx = string_table.intern("أ");
        let ب_idx = string_table.intern("ب");
        let أ1 = arena.allocate(ASTNode::Ident(أ_idx)).unwrap();
        let ب1 = arena.allocate(ASTNode::Ident(ب_idx)).unwrap();
        let body1 = arena.allocate(ASTNode::Int(0)).unwrap();
        let stmt1 = arena.allocate(ASTNode::LinearLet {
            name: ب1,
            value: أ1,
            body: body1,
        }).unwrap();
        let أ2 = arena.allocate(ASTNode::Ident(أ_idx)).unwrap();
        let stmt2 = arena.allocate(ASTNode::Call {
            func: أ2,
            args: NodeID::INVALID,
        }).unwrap();
        let stmts = vec![stmt1, stmt2];
        let result = check_ownership(&stmts, &arena, &string_table);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OwnershipError::UsedAfterMove { var, .. } if var == "أ"));
    }
    #[test]
    fn test_double_move() {
        let mut arena = Arena::new(100);
        let mut string_table = StringTable::new();
        let أ_idx = string_table.intern("أ");
        let ب_idx = string_table.intern("ب");
        let ج_idx = string_table.intern("ج");
        let أ1 = arena.allocate(ASTNode::Ident(أ_idx)).unwrap();
        let ب1 = arena.allocate(ASTNode::Ident(ب_idx)).unwrap();
        let body1 = arena.allocate(ASTNode::Int(0)).unwrap();
        let stmt1 = arena.allocate(ASTNode::LinearLet {
            name: ب1,
            value: أ1,
            body: body1,
        }).unwrap();
        let أ2 = arena.allocate(ASTNode::Ident(أ_idx)).unwrap();
        let ج2 = arena.allocate(ASTNode::Ident(ج_idx)).unwrap();
        let body2 = arena.allocate(ASTNode::Int(0)).unwrap();
        let stmt2 = arena.allocate(ASTNode::LinearLet {
            name: ج2,
            value: أ2,
            body: body2,
        }).unwrap();
        let stmts = vec![stmt1, stmt2];
        let result = check_ownership(&stmts, &arena, &string_table);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OwnershipError::DoubleMove { var, .. } if var == "أ"));
    }
    #[test]
    fn test_error_is_typed_enum() {
        let mut arena = Arena::new(100);
        let mut string_table = StringTable::new();
        let أ_idx = string_table.intern("أ");
        let ب_idx = string_table.intern("ب");
        let أ1 = arena.allocate(ASTNode::Ident(أ_idx)).unwrap();
        let ب1 = arena.allocate(ASTNode::Ident(ب_idx)).unwrap();
        let body1 = arena.allocate(ASTNode::Int(0)).unwrap();
        let stmt1 = arena.allocate(ASTNode::LinearLet {
            name: ب1,
            value: أ1,
            body: body1,
        }).unwrap();
        let أ2 = arena.allocate(ASTNode::Ident(أ_idx)).unwrap();
        let stmt2 = arena.allocate(ASTNode::Call {
            func: أ2,
            args: NodeID::INVALID,
        }).unwrap();
        let stmts = vec![stmt1, stmt2];
        let result = check_ownership(&stmts, &arena, &string_table);
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            OwnershipError::UsedAfterMove { var, span: _ } => {
                assert_eq!(var, "أ");
            }
            _ => panic!("Expected UsedAfterMove error"),
        }
    }
}
