//! MAL-NIR: SSA-based IR for MAL
#![forbid(unsafe_code)]
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueID(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockID(pub u32);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NIROp {
    Add, Sub, Mul, Div, Mod,
    Shl, Shr,
    And, Or, Xor,
    Eq, Ne, Lt, Le, Gt, Ge,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NIRUnOp { Neg, Not }
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
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockID,
    pub instructions: Vec<NIRInstruction>,
    pub predecessors: Vec<BlockID>,
    pub successors: Vec<BlockID>,
}
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
        Self { name, params: Vec::new(), entry_block: BlockID(0),
            blocks: Vec::new(), values: HashMap::new(),
            next_value_id: 0, next_block_id: 0 }
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
        self.blocks.push(BasicBlock { id, instructions: Vec::new(),
            predecessors: Vec::new(), successors: Vec::new() });
        id
    }
    pub fn add_instruction(&mut self, block_id: BlockID, inst: NIRInstruction) {
        if let Some(b) = self.blocks.iter_mut().find(|b| b.id == block_id) {
            b.instructions.push(inst);
        }
    }
    pub fn verify_ssa(&self) -> bool {
        let mut assigned: HashMap<ValueID, u32> = HashMap::new();
        for block in &self.blocks {
            for inst in &block.instructions {
                let r = match inst {
                    NIRInstruction::BinOp { result, .. } |
                    NIRInstruction::UnOp { result, .. } |
                    NIRInstruction::Call { result, .. } |
                    NIRInstruction::Load { result, .. } |
                    NIRInstruction::Alloc { result, .. } |
                    NIRInstruction::Move { result, .. } |
                    NIRInstruction::Borrow { result, .. } => Some(*result),
                    _ => None,
                };
                if let Some(v) = r { *assigned.entry(v).or_insert(0) += 1; }
            }
        }
        assigned.values().all(|&c| c <= 1)
    }
    pub fn instruction_count(&self) -> usize {
        self.blocks.iter().map(|b| b.instructions.len()).sum()
    }
}
pub mod lowering;
