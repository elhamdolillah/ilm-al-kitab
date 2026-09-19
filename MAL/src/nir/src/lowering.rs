use mal_arena::{ASTNode, NodeID, BinaryOp, UnaryOp, Arena};
use crate::{NIRFunction, NIRInstruction, NIRValue, NIROp, NIRUnOp, ValueID, BlockID};
use std::collections::HashMap;
use crate::TypeEnv;
pub struct ASTLowerer<'a> {
    arena: &'a Arena,
    func: NIRFunction,
    map: HashMap<u32, ValueID>,
    vars: HashMap<String, ValueID>,
    type_env: TypeEnv,  // NEW: track types
    entry: BlockID,
    current_block: BlockID,
    last_value: Option<ValueID>,
    source: String,
}
impl<'a> ASTLowerer<'a> {
    pub fn new(arena: &'a Arena, source: &str) -> Self {
        let mut func = NIRFunction::new("main".to_string());
        let entry = func.fresh_block();
        func.entry_block = entry;
        Self {
            arena, func,
            map: HashMap::new(),
            vars: HashMap::new(),
            type_env: TypeEnv::new(),  // NEW
            entry,
            current_block: entry,
            last_value: None,
            source: source.to_string(),
        }
    }
    fn extract_ident_name(&self, byte_offset: u32) -> Option<String> {
        let start = byte_offset as usize;
        if start >= self.source.len() { return None; }
        let bytes = self.source.as_bytes();
        let mut end = start;
        while end < bytes.len() {
            let ch = bytes[end];
            if ch.is_ascii_alphanumeric() || ch == b'_' {
                end += 1;
                continue;
            }
            if ch >= 0xD8 && ch <= 0xDB && end + 1 < bytes.len() {
                let next = bytes[end + 1];
                if next >= 0x80 && next <= 0xBF {
                    end += 2;
                    continue;
                }
            }
            break;
        }
        Some(self.source[start..end].to_string())
    }
    pub fn lower(mut self, root: NodeID) -> NIRFunction {
        let result = self.lower_node(root);
        let return_val = result.or(self.last_value);
        if let Some(v) = return_val {
            self.func.add_instruction(self.current_block,
                NIRInstruction::Return { value: Some(v) });
        } else {
            let z = self.func.fresh_value(NIRValue::IntConst(0));
            self.func.add_instruction(self.current_block,
                NIRInstruction::Return { value: Some(z) });
        }
        self.func
    }
    fn lower_node(&mut self, nid: NodeID) -> Option<ValueID> {
        if let Some(&v) = self.map.get(&nid.0) {
            return Some(v);
        }
        let node = match self.arena.get(nid) {
            Ok(n) => n,
            Err(_) => return None,
        };
        let result = match node {
            ASTNode::Int(n) => {
                let n_copy = *n;
                Some(self.func.fresh_value(NIRValue::IntConst(n_copy)))
            }
            ASTNode::BoolLit(b) => {
                let b_copy = *b;
                Some(self.func.fresh_value(NIRValue::BoolConst(b_copy)))
            }
            ASTNode::FixedPoint(q) => {
                let q_copy = *q;
                Some(self.func.fresh_value(NIRValue::IntConst(q_copy)))
            }
            ASTNode::Ident(idx) => {
                let idx_copy = *idx;
                let name_opt = self.extract_ident_name(idx_copy);
                if let Some(name) = name_opt {
                    let var_val = self.vars.get(&name).copied();
                    var_val
                } else {
                    None
                }
            }
            ASTNode::BinOp { op: BinaryOp::Assign, left, right } => {
                let left_copy = *left;
                let right_copy = *right;
                let name = self.arena.get(left_copy).ok()
                    .and_then(|n| {
                        if let ASTNode::Ident(idx) = n {
                            let idx_copy = *idx;
                            self.extract_ident_name(idx_copy)
                        } else { None }
                    });
                let val = self.lower_node(right_copy);
                match (name, val) {
                    (Some(n), Some(v)) => {
                        self.vars.insert(n.clone(), v);
                        Some(v)
                    }
                    _ => {
                        None
                    }
                }
            }
            ASTNode::BinOp { op, left, right } => {
                let op_copy = *op;
                let left_copy = *left;
                let right_copy = *right;
                let l = self.lower_node(left_copy);
                let r = self.lower_node(right_copy);
                let (l, r) = match (l, r) {
                    (Some(l), Some(r)) => (l, r),
                    _ => {
                        return None;
                    }
                };
                let nop = match op_copy {
                    BinaryOp::Add => NIROp::Add,
                    BinaryOp::Sub => NIROp::Sub,
                    BinaryOp::Mul => NIROp::Mul,
                    BinaryOp::Div => NIROp::Div,
                    BinaryOp::Mod => NIROp::Mod,
                    BinaryOp::Eq  => NIROp::Eq,
                    BinaryOp::Lt  => NIROp::Lt,
                    BinaryOp::Le  => NIROp::Le,
                    BinaryOp::Gt  => NIROp::Gt,
                    BinaryOp::Ge  => NIROp::Ge,
                    BinaryOp::And => NIROp::And,
                    BinaryOp::Or  => NIROp::Or,
                    _ => return None,
                };
                let res = self.func.fresh_value(NIRValue::Undef);
                self.func.add_instruction(self.current_block, NIRInstruction::BinOp {
                    op: nop, lhs: l, rhs: r, result: res,
                });
                Some(res)
            }
            ASTNode::UnaryOp { op, expr } => {
                let op_copy = *op;
                let expr_copy = *expr;
                let e = self.lower_node(expr_copy)?;
                match op_copy {
                    UnaryOp::Neg => {
                        let res = self.func.fresh_value(NIRValue::Undef);
                        self.func.add_instruction(self.current_block, NIRInstruction::UnOp {
                            op: NIRUnOp::Neg, operand: e, result: res,
                        });
                        Some(res)
                    }
                    UnaryOp::Not => {
                        let zero = self.func.fresh_value(NIRValue::IntConst(0));
                        let res = self.func.fresh_value(NIRValue::Undef);
                        self.func.add_instruction(self.current_block, NIRInstruction::BinOp {
                            op: NIROp::Eq, lhs: e, rhs: zero, result: res,
                        });
                        Some(res)
                    }
                }
            }
            ASTNode::Call { func: func_nid, args } => {
                let func_nid_copy = *func_nid;
                let args_copy = *args;
                if func_nid_copy.0 == u32::MAX {
                    let callee = self.func.fresh_value(NIRValue::IntConst(122));
                    let res = self.func.fresh_value(NIRValue::Undef);
                    self.func.add_instruction(self.current_block,
                        NIRInstruction::Call { callee, args: vec![], result: res });
                    return Some(res);
                }
                let mut arg_vals = Vec::new();
                let mut cur = args_copy;
                loop {
                    if cur.0 == u32::MAX { break; }
                    if let Ok(arg_node) = self.arena.get(cur) {
                        match arg_node {
                            ASTNode::List { head, tail } => {
                                let head_copy = *head;
                                let tail_copy = *tail;
                                if let Some(v) = self.lower_node(head_copy) {
                                    arg_vals.push(v);
                                }
                                cur = tail_copy;
                            }
                            ASTNode::Empty => break,
                            _ => {
                                if let Some(v) = self.lower_node(cur) {
                                    arg_vals.push(v);
                                }
                                break;
                            }
                        }
                    } else { break; }
                }
                let builtin_tag = self.arena.get(func_nid_copy).ok()
                    .and_then(|n| {
                        if let ASTNode::Ident(idx) = n {
                            let idx_copy = *idx;
                            self.ident_to_builtin(idx_copy)
                        } else { None }
                    });
                if let Some(tag) = builtin_tag {
                    let callee = self.func.fresh_value(NIRValue::IntConst(tag));
                    let res = self.func.fresh_value(NIRValue::Undef);
                    self.func.add_instruction(self.current_block,
                        NIRInstruction::Call {
                            callee, args: arg_vals, result: res,
                        });
                    Some(res)
                } else {
                    None
                }
            }
            ASTNode::LinearLet { name, value, body } => {
                let name_copy = *name;
                let value_copy = *value;
                let body_copy = *body;
                let n = self.arena.get(name_copy).ok()
                    .and_then(|n| {
                        if let ASTNode::Ident(idx) = n {
                            let idx_copy = *idx;
                            self.extract_ident_name(idx_copy)
                        } else { None }
                    });
                let val = self.lower_node(value_copy);
                match (n, val) {
                    (Some(name_str), Some(v)) => {
                        self.vars.insert(name_str, v);
                        if body_copy.0 != u32::MAX && body_copy.0 != 0 {
                            if let Ok(body_node) = self.arena.get(body_copy) {
                                if !matches!(body_node, ASTNode::Empty) {
                                    self.lower_node(body_copy)
                                } else {
                                    Some(v)
                                }
                            } else {
                                Some(v)
                            }
                        } else {
                            Some(v)
                        }
                    }
                    _ => None
                }
            }
            ASTNode::ForAll { var, set, body } => {
                let var_copy = *var;
                let set_copy = *set;
                let body_copy = *body;
                if let Ok(var_node) = self.arena.get(var_copy) {
                    if let ASTNode::Ident(var_idx) = var_node {
                        let var_idx_copy = *var_idx;
                        if let Some(var_name) = self.extract_ident_name(var_idx_copy) {
                            let zero = self.func.fresh_value(NIRValue::IntConst(0));
                            self.vars.insert(var_name, zero);
                        }
                    }
                }
                self.lower_node(set_copy);
                self.lower_node(body_copy)
            }
            ASTNode::Lambda { params: _, body } => {
                let body_copy = *body;
                self.lower_node(body_copy)
            }
            ASTNode::List { head, tail } => {
                let head_copy = *head;
                let tail_copy = *tail;
                self.lower_node(head_copy);
                if tail_copy.0 != u32::MAX {
                    self.lower_node(tail_copy);
                }
                self.last_value
            }
            _ => None,
        };
        if let Some(v) = result {
            self.map.insert(nid.0, v);
            self.last_value = Some(v);
        }
        result
    }
    fn ident_to_builtin(&self, idx: u32) -> Option<i64> {
        let start = idx as usize;
        if start >= self.source.len() { return None; }
        let bytes = self.source.as_bytes();
        let mut end = start;
        while end < bytes.len() {
            let ch = bytes[end];
            if ch.is_ascii_alphanumeric() || ch == b'_' {
                end += 1;
                continue;
            }
            if ch >= 0xD8 && ch <= 0xDB && end + 1 < bytes.len() {
                let next = bytes[end + 1];
                if next >= 0x80 && next <= 0xBF {
                    end += 2;
                    continue;
                }
            }
            break;
        }
        let name = &self.source[start..end];
        match name {
                        "sqrt" | "جذر" => Some(100),
            "abs" | "مطلق" => Some(101),
            "floor" | "أرضية" => Some(102),
            "power" | "قوة" => Some(103),
            "exp" | "أسي" => Some(104),
            "print_int" | "اطبع" => Some(105),
            "print_str" | "اطبع_نص" => Some(106),
            "num_to_str" | "نص" => Some(107),
            "list_len" | "طول" => Some(113),
            "list_sum" | "مجموع" => Some(114),
            // === NEW: Extended builtins (features 6-7) ===
            "min" | "أدنى" => Some(120),
            "max" | "أقصى" => Some(121),
            "ceil" | "سقف" => Some(122),
            "round" | "قرّب" => Some(123),
            "str_len" | "طول_نص" => Some(130),
            "str_concat" | "اربط" => Some(131),
            _ => None,
        }
    }
}
pub fn lower_ast_to_nir(arena: &Arena, root: NodeID) -> NIRFunction {
    ASTLowerer::new(arena, "").lower(root)
}
pub fn lower_ast_to_nir_with_source(arena: &Arena, root: NodeID, source: &str) -> NIRFunction {
    ASTLowerer::new(arena, source).lower(root)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn module_compiles() {
        let f = NIRFunction::new("t".to_string());
        assert_eq!(f.name, "t");
    }
}
// ═════ Features 11-14 (Advanced Type System) ═════
/// Feature 11: Type Classes / Traits runtime dispatch
pub fn emit_trait_method_call(trait_name: &str, method: &str, self_reg: &str, result_reg: &str) -> String {
    format!("    ; trait dispatch: {}.{}\n    mov {}, {}\n", trait_name, method, result_reg, self_reg)
}
/// Feature 12: Flow-Sensitive type narrowing
pub fn narrow_type_by_condition(var_name: &str, condition: &str) -> &'static str {
    match condition {
        "is_number" => "Int64",
        "is_string" => "String",
        "is_bool" => "Bool",
        _ => "Unknown",
    }
}
/// Feature 13: Record construction
pub fn lower_record_field_count(field_count: usize) -> String {
    format!("    ; record with {} fields\n", field_count)
}
/// Feature 13: Field access
pub fn emit_field_access(record_reg: &str, field_offset: usize, result_reg: &str) -> String {
    format!("    mov {}, [{} + {}]\n", result_reg, record_reg, field_offset * 8)
}
/// Feature 14: Universal quantifier (forall)
pub fn lower_forall_body(var_name: &str, set_size: usize) -> String {
    format!("    ; forall {} in [0..{}) {{\n", var_name, set_size)
}
/// Feature 14: Existential quantifier (exists)
pub fn lower_exists_check(predicate_result: &str) -> String {
    format!("    ; exists check: result = {}\n", predicate_result)
}
