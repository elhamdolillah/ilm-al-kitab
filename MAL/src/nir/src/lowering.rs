//! AST -> NIR lowering (FIXED: use lower_node result)
use mal_arena::{ASTNode, NodeID, BinaryOp, UnaryOp, Arena};
use crate::{NIRFunction, NIRInstruction, NIRValue, NIROp, NIRUnOp, ValueID, BlockID};
use std::collections::HashMap;
pub struct ASTLowerer<'a> {
    arena: &'a Arena,
    func: NIRFunction,
    map: HashMap<u32, ValueID>,
    entry: BlockID,
}
impl<'a> ASTLowerer<'a> {
    pub fn new(arena: &'a Arena) -> Self {
        let mut func = NIRFunction::new("main".to_string());
        let entry = func.fresh_block();
        func.entry_block = entry;
        Self { arena, func, map: HashMap::new(), entry }
    }
    pub fn lower(mut self, root: NodeID) -> NIRFunction {
        // ✅ FIX: Use the result from lower_node as the return value
        if let Some(result_val) = self.lower_node(root) {
            self.func.add_instruction(
                self.entry,
                NIRInstruction::Return { value: Some(result_val) }
            );
        } else {
            // Fallback: return 0 if lowering failed
            let z = self.func.fresh_value(NIRValue::IntConst(0));
            self.func.add_instruction(
                self.entry,
                NIRInstruction::Return { value: Some(z) }
            );
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
                let nop = match op {
                    UnaryOp::Neg => NIRUnOp::Neg,
                    UnaryOp::Not => NIRUnOp::Not,
                };
                let res = self.func.fresh_value(NIRValue::Undef);
                self.func.add_instruction(self.entry, NIRInstruction::UnOp {
                    op: nop, operand: e, result: res,
                });
                Some(res)
            }
            _ => None,
        };
        if let Some(v) = result {
            self.map.insert(nid.0, v);
        }
        result
    }
}
pub fn lower_ast_to_nir(arena: &Arena, root: NodeID) -> NIRFunction {
    ASTLowerer::new(arena).lower(root)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn module_compiles() {
        let f = NIRFunction::new("t".to_string());
        assert_eq!(f.name, "t");
    }
}
