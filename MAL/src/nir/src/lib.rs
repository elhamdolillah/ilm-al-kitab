//! # MAL Native Intermediate Representation (MAL-NIR)
//!
//! SSA-based IR: the unified mathematical bridge between MAL source
//! and target backends (x86, ARM, WASM).
//!
//! ## Constitutional Compliance
//! - Principle 1 (الإحكام): SSA = one definition per variable
//! - Principle 4 (الأمانة): Ownership tracked in IR
//! - Principle 6 (الحفظ): No unsafe code
//! - Principle 9 (الوحدة الدلالية): One IR, many backends
#![forbid(unsafe_code)]
use std::collections::HashMap;
/// Unique identifier for an SSA value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueID(pub u32);
/// Unique identifier for a basic block
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockID(pub u32);
/// SSA Value types
#[derive(Debug, Clone, PartialEq)]
pub enum NIRValue {
    IntConst(i64),
    FloatConst(f64),
    BoolConst(bool),
    StrConst(u32),
    FunctionResult(ValueID),
    Phi(Vec<(BlockID, ValueID)>),
    Undef,
}
/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NIROp {
    Add, Sub, Mul, Div, Mod,
    Shl, Shr,
    And, Or, Xor,
    Eq, Ne, Lt, Le, Gt, Ge,
}
/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NIRUnOp {
    Neg,
    Not,
}
/// NIR instruction types
#[derive(Debug, Clone, PartialEq)]
pub enum NIRInstruction {
    BinOp { op: NIROp, lhs: ValueID, rhs: ValueID, result: ValueID },
    UnOp { op: NIRUnOp, operand: ValueID, result: ValueID },
    Call { callee: ValueID, args: Vec<ValueID>, result: ValueID },
    Load { ptr: ValueID, result: ValueID },
    Store { ptr: ValueID, value: ValueID },
    Alloc { size: ValueID, result: ValueID },
    Free { ptr: ValueID },
    Move { value: ValueID, result: ValueID },
    Borrow { value: ValueID, mutable: bool, result: ValueID },
    CondBranch { cond: ValueID, then_bb: BlockID, else_bb: BlockID },
    Branch { target: BlockID },
    Return { value: Option<ValueID> },
}
/// Basic block
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockID,
    pub instructions: Vec<NIRInstruction>,
    pub predecessors: Vec<BlockID>,
    pub successors: Vec<BlockID>,
}
/// Function in NIR
#[derive(Debug, Clone)]
pub struct NIRFunction {
    pub name: String,
    pub params: Vec<ValueID>,
    pub entry_block: BlockID,
    pub blocks: Vec<BasicBlock>,
    pub values: HashMap<ValueID, NIRValue>,
    pub next_value_id: u32,
    pub next_block_id: u32,
}
impl NIRFunction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            entry_block: BlockID(0),
            blocks: Vec::new(),
            values: HashMap::new(),
            next_value_id: 0,
            next_block_id: 0,
        }
    }
    pub fn fresh_value(&mut self, val: NIRValue) -> ValueID {
        let id = ValueID(self.next_value_id);
        self.next_value_id += 1;
        self.values.insert(id, val);
        id
    }
    pub fn fresh_block(&mut self) -> BlockID {
        let id = BlockID(self.next_block_id);
        self.next_block_id += 1;
        self.blocks.push(BasicBlock {
            id,
            instructions: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
        });
        id
    }
    pub fn add_instruction(&mut self, block_id: BlockID, inst: NIRInstruction) {
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == block_id) {
            block.instructions.push(inst);
        }
    }
    /// Verify SSA property: each value assigned at most once
    pub fn verify_ssa(&self) -> bool {
        let mut assigned: HashMap<ValueID, u32> = HashMap::new();
        for block in &self.blocks {
            for inst in &block.instructions {
                let result = match inst {
                    NIRInstruction::BinOp { result, .. } => Some(*result),
                    NIRInstruction::UnOp { result, .. } => Some(*result),
                    NIRInstruction::Call { result, .. } => Some(*result),
                    NIRInstruction::Load { result, .. } => Some(*result),
                    NIRInstruction::Alloc { result, .. } => Some(*result),
                    NIRInstruction::Move { result, .. } => Some(*result),
                    NIRInstruction::Borrow { result, .. } => Some(*result),
                    _ => None,
                };
                if let Some(val_id) = result {
                    *assigned.entry(val_id).or_insert(0) += 1;
                }
            }
        }
        assigned.values().all(|&count| count <= 1)
    }
    pub fn instruction_count(&self) -> usize {
        self.blocks.iter().map(|b| b.instructions.len()).sum()
    }
}
/// Lowering pass: converts MAL AST to NIR
pub struct ASTToNIRLowering;
impl ASTToNIRLowering {
    pub fn new() -> Self { Self }
    pub fn lower_binop_int(
        &self,
        func: &mut NIRFunction,
        op: NIROp,
        lhs_val: i64,
        rhs_val: i64,
    ) -> ValueID {
        let lhs_id = func.fresh_value(NIRValue::IntConst(lhs_val));
        let rhs_id = func.fresh_value(NIRValue::IntConst(rhs_val));
        let result_id = func.fresh_value(NIRValue::Undef);
        let inst = NIRInstruction::BinOp {
            op,
            lhs: lhs_id,
            rhs: rhs_id,
            result: result_id,
        };
        let entry = func.entry_block;
        func.add_instruction(entry, inst);
        result_id
    }
}
impl Default for ASTToNIRLowering {
    fn default() -> Self { Self::new() }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ssa_value_creation() {
        let mut func = NIRFunction::new("test_fn".to_string());
        let v1 = func.fresh_value(NIRValue::IntConst(5));
        let v2 = func.fresh_value(NIRValue::IntConst(3));
        assert_eq!(v1, ValueID(0));
        assert_eq!(v2, ValueID(1));
        assert_eq!(func.values.get(&v1), Some(&NIRValue::IntConst(5)));
    }
    #[test]
    fn test_basic_block_creation() {
        let mut func = NIRFunction::new("test_fn".to_string());
        let bb0 = func.fresh_block();
        let bb1 = func.fresh_block();
        assert_eq!(bb0, BlockID(0));
        assert_eq!(bb1, BlockID(1));
        assert_eq!(func.blocks.len(), 2);
    }
    #[test]
    fn test_binop_lowering() {
        let mut func = NIRFunction::new("add_test".to_string());
        func.fresh_block();
        let lowering = ASTToNIRLowering::new();
        let result = lowering.lower_binop_int(&mut func, NIROp::Add, 5, 3);
        assert_eq!(func.next_value_id, 3);
        assert_eq!(func.instruction_count(), 1);
        let inst = &func.blocks[0].instructions[0];
        if let NIRInstruction::BinOp { op, lhs, rhs, result: res } = inst {
            assert_eq!(*op, NIROp::Add);
            assert_eq!(*lhs, ValueID(0));
            assert_eq!(*rhs, ValueID(1));
            assert_eq!(*res, result);
        } else {
            panic!("Expected BinOp instruction");
        }
    }
    #[test]
    fn test_ssa_property_verification() {
        let mut func = NIRFunction::new("ssa_test".to_string());
        func.fresh_block();
        let lowering = ASTToNIRLowering::new();
        lowering.lower_binop_int(&mut func, NIROp::Add, 1, 2);
        lowering.lower_binop_int(&mut func, NIROp::Mul, 3, 4);
        assert!(func.verify_ssa());
        assert_eq!(func.instruction_count(), 2);
    }
    #[test]
    fn test_function_structure() {
        let mut func = NIRFunction::new("main".to_string());
        let entry = func.fresh_block();
        let then_bb = func.fresh_block();
        let else_bb = func.fresh_block();
        let merge_bb = func.fresh_block();
        func.blocks[0].successors = vec![then_bb, else_bb];
        func.blocks[1].predecessors = vec![entry];
        func.blocks[1].successors = vec![merge_bb];
        func.blocks[2].predecessors = vec![entry];
        func.blocks[2].successors = vec![merge_bb];
        func.blocks[3].predecessors = vec![then_bb, else_bb];
        assert_eq!(func.blocks.len(), 4);
        assert_eq!(func.blocks[0].successors.len(), 2);
        assert!(func.verify_ssa());
    }
}
