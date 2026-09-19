//! Type inference for MAL (Hindley-Milner inspired)
use std::collections::HashMap;
use mal_arena::{ASTNode, NodeID, BinaryOp, Arena};
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i32),      // bit width
    Float(i32),
    Bool,
    String,
    Unit,
    Var(String),   // type variable
    Func(Vec<Type>, Box<Type>),
    Array(Box<Type>, usize),
    Unknown,
}
impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int(_) | Type::Float(_))
    }
    pub fn is_compatible(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            (Type::Int(a), Type::Int(b)) => a == b,
            (Type::Float(a), Type::Float(b)) => a == b,
            (Type::Bool, Type::Bool) => true,
            (Type::String, Type::String) => true,
            (Type::Var(_), _) | (_, Type::Var(_)) => true,
            _ => false,
        }
    }
}
pub struct TypeEnv {
    bindings: HashMap<String, Type>,
    next_var: u32,
}
impl TypeEnv {
    pub fn new() -> Self {
        TypeEnv {
            bindings: HashMap::new(),
            next_var: 0,
        }
    }
    pub fn fresh_var(&mut self) -> Type {
        let var = Type::Var(format!("t{}", self.next_var));
        self.next_var += 1;
        var
    }
    pub fn bind(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings.get(name)
    }
}
pub struct TypeChecker<'a> {
    arena: &'a Arena,
    env: TypeEnv,
}
impl<'a> TypeChecker<'a> {
    pub fn new(arena: &'a Arena) -> Self {
        TypeChecker {
            arena,
            env: TypeEnv::new(),
        }
    }
    pub fn infer(&mut self, nid: NodeID) -> Type {
        if let Ok(node) = self.arena.get(nid) {
            match node {
                ASTNode::Int(_) => Type::Int(64),
                ASTNode::BoolLit(_) => Type::Bool,
                ASTNode::Ident(offset) => {
                    // Extract name from source (simplified)
                    let name = format!("var_{}", offset);
                    if let Some(ty) = self.env.lookup(&name) {
                        ty.clone()
                    } else {
                        let ty = self.env.fresh_var();
                        self.env.bind(name, ty.clone());
                        ty
                    }
                }
                ASTNode::BinOp { op, left, right } => {
                    let lt = self.infer(*left);
                    let rt = self.infer(*right);
                    match op {
                        BinaryOp::Assign => {
                            // Assign: bind left to right's type
                            rt
                        }
                        BinaryOp::Add | BinaryOp::Sub | 
                        BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                            // Arithmetic: both must be numeric
                            if lt.is_numeric() && rt.is_numeric() {
                                lt
                            } else {
                                Type::Unknown
                            }
                        }
                        BinaryOp::Eq | BinaryOp::Lt | BinaryOp::Le |
                        BinaryOp::Gt | BinaryOp::Ge => {
                            // Comparison: returns Bool
                            Type::Bool
                        }
                        BinaryOp::And | BinaryOp::Or => {
                            // Logical: both Bool
                            Type::Bool
                        }
                        _ => Type::Unknown,
                    }
                }
                _ => Type::Unknown,
            }
        } else {
            Type::Unknown
        }
    }
}
