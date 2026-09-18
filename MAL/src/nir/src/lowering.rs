//! AST -> NIR lowering with CORRECT multi-statement support
use mal_arena::{ASTNode, NodeID, BinaryOp, UnaryOp, Arena};
use crate::{NIRFunction, NIRInstruction, NIRValue, NIROp, NIRUnOp, ValueID, BlockID};
use std::collections::HashMap;
pub struct ASTLowerer<'a> {
    arena: &'a Arena,
    func: NIRFunction,
    map: HashMap<u32, ValueID>,
    vars: HashMap<String, ValueID>,
    entry: BlockID,
    current_block: BlockID,
    last_value: Option<ValueID>,
    source: String,
    string_table: Vec<String>,
    loop_break_target: Option<BlockID>,
    loop_continue_target: Option<BlockID>,
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
            current_block: entry,
            last_value: None,
            source: source.to_string(),
            string_table: table,
            loop_break_target: None,
            loop_continue_target: None,
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
                self.extract_ident_name(*idx)
                    .and_then(|name| self.vars.get(&name).copied())
            }
            ASTNode::BinOp { op: BinaryOp::Assign, left, right } => {
                let name = self.arena.get(*left).ok()
                    .and_then(|n| if let ASTNode::Ident(idx) = n { self.extract_ident_name(*idx) } else { None });
                let val = self.lower_node(*right);
                match (name, val) {
                    (Some(n), Some(v)) => {
                        self.vars.insert(n, v);
                        Some(v)
                    }
                    _ => None
                }
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
                self.func.add_instruction(self.current_block, NIRInstruction::BinOp {
                    op: nop, lhs: l, rhs: r, result: res,
                });
                Some(res)
            }
            ASTNode::UnaryOp { op, expr } => {
                let e = self.lower_node(*expr)?;
                match op {
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
                if func_nid.0 == u32::MAX {
                    let callee = self.func.fresh_value(NIRValue::IntConst(122));
                    let res = self.func.fresh_value(NIRValue::Undef);
                    self.func.add_instruction(self.current_block,
                        NIRInstruction::Call { callee, args: vec![], result: res });
                    Some(res)
                } else {
                    let mut arg_vals = Vec::new();
                    let mut cur = *args;
                    loop {
                        if cur.0 == u32::MAX { break; }
                        if let Ok(arg_node) = self.arena.get(cur) {
                            match arg_node {
                                ASTNode::List { head, tail } => {
                                    if let Some(v) = self.lower_node(*head) {
                                        arg_vals.push(v);
                                    }
                                    cur = *tail;
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
                    let builtin_tag = self.arena.get(*func_nid).ok()
                        .and_then(|n| if let ASTNode::Ident(idx) = n { 
                            self.ident_to_builtin(*idx) 
                        } else { None });
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
            }
            ASTNode::LinearLet { name, value, body } => {
                let n = self.arena.get(*name).ok()
                    .and_then(|n| if let ASTNode::Ident(idx) = n { self.extract_ident_name(*idx) } else { None });
                let val = self.lower_node(*value);
                match (n, val) {
                    (Some(name), Some(v)) => {
                        self.vars.insert(name, v);
                        if body.0 != u32::MAX && body.0 != 0 {
                            if let Ok(body_node) = self.arena.get(*body) {
                                if !matches!(body_node, ASTNode::Empty) {
                                    self.lower_node(*body)
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
                if let Ok(var_node) = self.arena.get(*var) {
                    if let ASTNode::Ident(var_idx) = var_node {
                        if let Some(var_name) = self.extract_ident_name(*var_idx) {
                            let zero = self.func.fresh_value(NIRValue::IntConst(0));
                            self.vars.insert(var_name, zero);
                        }
                    }
                }
                self.lower_node(*set);
                self.lower_node(*body)
            }
            ASTNode::Lambda { params: _, body } => {
                self.lower_node(*body)
            }
            // MULTI-STATEMENT LIST HANDLER - THE CORRECT FIX!
            ASTNode::List { head, tail } => {
                eprintln!("[LIST] Processing List node: head={}, tail={}", head.0, tail.0);
                // Step 1: Lower head (first statement)
                let head_val = self.lower_node(*head);
                eprintln!("[LIST] Lowered head, value={:?}", head_val);
                // Step 2: Lower tail recursively if it exists
                if tail.0 != u32::MAX {
                    eprintln!("[LIST] Lowering tail recursively...");
                    let tail_val = self.lower_node(*tail);
                    eprintln!("[LIST] Tail returned: {:?}", tail_val);
                }
                // Step 3: Return the last computed value
                eprintln!("[LIST] Returning last_value: {:?}", self.last_value);
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
