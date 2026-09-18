//! AST -> NIR lowering (with nested calls + multi-arg builtins)
use mal_arena::{ASTNode, NodeID, BinaryOp, UnaryOp, Arena};
use crate::{NIRFunction, NIRInstruction, NIRValue, NIROp, NIRUnOp, ValueID, BlockID};
use std::collections::HashMap;
pub struct ASTLowerer<'a> {
    arena: &'a Arena,
    func: NIRFunction,
    map: HashMap<u32, ValueID>,
    vars: HashMap<u32, ValueID>,
    entry: BlockID,
    last_value: Option<ValueID>,
    string_table: Vec<String>,
}
impl<'a> ASTLowerer<'a> {
    pub fn new(arena: &'a Arena, source: &str) -> Self {
        let mut func = NIRFunction::new("main".to_string());
        let entry = func.fresh_block();
        func.entry_block = entry;
        let mut table = Vec::new();
        let chars: Vec<(usize, char)> = source.char_indices().collect();
        let mut i = 0;
        while i < chars.len() {
            let (byte_idx, ch) = chars[i];
            if ch.is_whitespace() { i += 1; continue; }
            let is_ident_start = ch == '_' || ch.is_ascii_alphabetic() ||
                ch >= '\u{0600}' && ch <= '\u{06FF}' ||
                ch >= '\u{0750}' && ch <= '\u{077F}' ||
                ch >= '\u{FB50}' && ch <= '\u{FDFF}' ||
                ch >= '\u{FE70}' && ch <= '\u{FEFF}';
            if is_ident_start {
                let start_byte = byte_idx;
                while i < chars.len() {
                    let c = chars[i].1;
                    let is_ident_cont = c == '_' || c.is_ascii_alphanumeric() ||
                        c >= '\u{0600}' && c <= '\u{06FF}' ||
                        c >= '\u{0750}' && c <= '\u{077F}' ||
                        c >= '\u{FB50}' && c <= '\u{FDFF}' ||
                        c >= '\u{FE70}' && c <= '\u{FEFF}';
                    if is_ident_cont { i += 1; } else { break; }
                }
                let end_byte = if i < chars.len() { chars[i].0 } else { source.len() };
                let s = &source[start_byte..end_byte];
                if !table.contains(&s.to_string()) {
                    table.push(s.to_string());
                }
            } else {
                i += 1;
            }
        }
        Self {
            arena, func,
            map: HashMap::new(),
            vars: HashMap::new(),
            entry,
            last_value: None,
            string_table: table,
        }
    }
    pub fn lower(mut self, root: NodeID) -> NIRFunction {
        let result = self.lower_node(root);
        let return_val = result.or(self.last_value);
        if let Some(v) = return_val {
            self.func.add_instruction(self.entry,
                NIRInstruction::Return { value: Some(v) });
        } else {
            let z = self.func.fresh_value(NIRValue::IntConst(0));
            self.func.add_instruction(self.entry,
                NIRInstruction::Return { value: Some(z) });
        }
        self.func
    }
    fn lower_node(&mut self, nid: NodeID) -> Option<ValueID> {
        if let Some(&v) = self.map.get(&nid.0) {
            return Some(v);
        }
        let node = self.arena.get(nid).ok()?;
        let result = match node {
            ASTNode::Int(n) => {
                Some(self.func.fresh_value(NIRValue::IntConst(*n)))
            }
            ASTNode::BoolLit(b) => {
                Some(self.func.fresh_value(NIRValue::BoolConst(*b)))
            }
            ASTNode::FixedPoint(q) => {
                Some(self.func.fresh_value(NIRValue::IntConst(*q)))
            }
            ASTNode::Ident(idx) => {
                self.vars.get(idx).copied()
            }
            ASTNode::BinOp { op: BinaryOp::Assign, left, right } => {
                if let Ok(name_node) = self.arena.get(*left) {
                    if let ASTNode::Ident(name_idx) = name_node {
                        if let Some(val) = self.lower_node(*right) {
                            self.vars.insert(*name_idx, val);
                            self.last_value = Some(val);
                            return Some(val);
                        }
                    }
                }
                None
            }
            ASTNode::BinOp { op, left, right } => {
                let l = self.lower_node(*left)?;
                let r = self.lower_node(*right)?;
                let nop = match op {
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
                self.func.add_instruction(self.entry, NIRInstruction::BinOp {
                    op: nop, lhs: l, rhs: r, result: res,
                });
                Some(res)
            }
            ASTNode::UnaryOp { op, expr } => {
                let e = self.lower_node(*expr)?;
                match op {
                    UnaryOp::Neg => {
                        let res = self.func.fresh_value(NIRValue::Undef);
                        self.func.add_instruction(self.entry, NIRInstruction::UnOp {
                            op: NIRUnOp::Neg, operand: e, result: res,
                        });
                        Some(res)
                    }
                    UnaryOp::Not => {
                        let zero = self.func.fresh_value(NIRValue::IntConst(0));
                        let res = self.func.fresh_value(NIRValue::Undef);
                        self.func.add_instruction(self.entry, NIRInstruction::BinOp {
                            op: NIROp::Eq, lhs: e, rhs: zero, result: res,
                        });
                        Some(res)
                    }
                }
            }
            // Call: builtin or user function
            ASTNode::Call { func: func_nid, args } => {
                // Special case: Read builtin (func = INVALID)
                if func_nid.0 == u32::MAX {
                    let callee = self.func.fresh_value(NIRValue::IntConst(122));
                    let res = self.func.fresh_value(NIRValue::Undef);
                    self.func.add_instruction(self.entry,
                        NIRInstruction::Call { callee, args: vec![], result: res });
                    return Some(res);
                }
                // Collect ALL args (handles nested calls + multi-arg)
                let mut arg_vals = Vec::new();
                let mut cur = *args;
                // Walk the List chain
                loop {
                    if cur.0 == u32::MAX { break; }
                    if let Ok(arg_node) = self.arena.get(cur) {
                        match arg_node {
                            ASTNode::List { head, tail } => {
                                // Recursively lower head (supports nested calls)
                                if let Some(v) = self.lower_node(*head) {
                                    arg_vals.push(v);
                                }
                                cur = *tail;
                            }
                            ASTNode::Empty => break,
                            // Single value (not in list) — lower it
                            _ => {
                                if let Some(v) = self.lower_node(cur) {
                                    arg_vals.push(v);
                                }
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                // Get function name for builtin dispatch
                if let Ok(func_node) = self.arena.get(*func_nid) {
                    if let ASTNode::Ident(name_idx) = func_node {
                        if let Some(builtin_tag) = self.ident_to_builtin(*name_idx) {
                            let callee = self.func.fresh_value(NIRValue::IntConst(builtin_tag));
                            let res = self.func.fresh_value(NIRValue::Undef);
                            self.func.add_instruction(self.entry,
                                NIRInstruction::Call {
                                    callee, args: arg_vals, result: res,
                                });
                            return Some(res);
                        }
                    }
                }
                // Unknown function — return None
                None
            }
            ASTNode::LinearLet { name, value, body } => {
                if let Ok(name_node) = self.arena.get(*name) {
                    if let ASTNode::Ident(name_idx) = name_node {
                        if let Some(val) = self.lower_node(*value) {
                            self.vars.insert(*name_idx, val);
                            self.last_value = Some(val);
                            if body.0 != u32::MAX && body.0 != 0 {
                                if let Ok(body_node) = self.arena.get(*body) {
                                    if !matches!(body_node, ASTNode::Empty) {
                                        return self.lower_node(*body);
                                    }
                                }
                            }
                            return Some(val);
                        }
                    }
                }
                None
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
        let name = self.string_table.get(idx as usize)?;
        match name.as_str() {
            "sqrt" | "جذر" => Some(100),
            "abs" | "مطلق" => Some(101),
            "floor" | "أرضية" => Some(102),
            "power" | "قوة" => Some(103),
            "exp" | "أسي" => Some(104),
            "print_int" | "اطبع" => Some(105),
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
