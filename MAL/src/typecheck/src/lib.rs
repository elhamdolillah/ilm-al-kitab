//! # MAL Type Checker — type inference and checking
#![forbid(unsafe_code)]
use mal_arena::{Arena, ASTNode, BinaryOp, UnaryOp, NodeID};
use mal_types::{DataType, TensorDtype, Shape};
use mal_ownership::StringTable;
use std::collections::HashMap;
/// Type checking errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    /// Incompatible types in binary operation.
    IncompatibleTypes {
        expected: DataType,
        actual: DataType,
    },
    /// Variable not found in type environment.
    UndefinedVariable { var: String },
    /// Type mismatch in function call.
    TypeMismatch { expected: DataType, actual: DataType },
    /// Unknown type (cannot infer).
    UnknownType { expr: String },
    /// Matrix multiplication requires 2D tensors.
    MatMulRequires2D { lhs_rank: usize, rhs_rank: usize },
    /// Matrix multiplication inner dimensions must match.
    MatMulDimMismatch { lhs_inner: usize, rhs_inner: usize },
    /// Shapes are not compatible for broadcasting.
    BroadcastIncompatible { shape1: Shape, shape2: Shape },
    /// Tensor operation on non-tensor type.
    NotATensor { expected: DataType },

}
impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::IncompatibleTypes { expected, actual } => {
                write!(f, "خطأ أنواع: متوقع {:?} لكن وجد {:?}", expected, actual)
            }
            TypeError::UndefinedVariable { var } => {
                write!(f, "خطأ: المتغير '{}' غير معرف", var)
            }
            TypeError::TypeMismatch { expected, actual } => {
                write!(f, "خطأ أنواع: متوقع {:?} لكن وجد {:?}", expected, actual)
            }
            TypeError::UnknownType { expr } => {
                write!(f, "خطأ: لا يمكن استنتاج نوع التعبير '{}'", expr)
            }
        }
    }
}
impl std::error::Error for TypeError {}
/// Type environment: maps variable names to their types.
pub type TypeEnv = HashMap<String, DataType>;
/// Infer the type of an expression.
pub fn infer_type(
    node_id: NodeID,
    arena: &Arena,
    string_table: &StringTable,
    type_env: &TypeEnv,
) -> Result<DataType, TypeError> {
    if let Ok(node) = arena.get(node_id) {
        match node {
            ASTNode::Int(_) => Ok(DataType::Int64),
            ASTNode::BoolLit(_) => Ok(DataType::Bool),
            ASTNode::Str(_) => Ok(DataType::Text),
            ASTNode::Ident(idx) => {
                if let Some(name) = string_table.get(*idx) {
                    type_env.get(name).cloned().ok_or_else(|| TypeError::UndefinedVariable {
                        var: name.to_string(),
                    })
                } else {
                    Err(TypeError::UnknownType { expr: "unknown identifier".to_string() })
                }
            }
            ASTNode::BinOp { op, left, right } => {
                let left_type = infer_type(*left, arena, string_table, type_env)?;
                let right_type = infer_type(*right, arena, string_table, type_env)?;
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod | BinaryOp::Pow => {
                        if left_type != DataType::Int64 {
                            return Err(TypeError::IncompatibleTypes { expected: DataType::Int64, actual: left_type });
                        }
                        if right_type != DataType::Int64 {
                            return Err(TypeError::IncompatibleTypes { expected: DataType::Int64, actual: right_type });
                        }
                        Ok(DataType::Int64)
                    }
                    BinaryOp::Concat => {
                        if left_type != DataType::Text {
                            return Err(TypeError::IncompatibleTypes { expected: DataType::Text, actual: left_type });
                        }
                        if right_type != DataType::Text {
                            return Err(TypeError::IncompatibleTypes { expected: DataType::Text, actual: right_type });
                        }
                        Ok(DataType::Text)
                    }
                    BinaryOp::Lt | BinaryOp::Gt | BinaryOp::Le | BinaryOp::Ge | BinaryOp::Eq | BinaryOp::Neq => {
                        if left_type != right_type {
                            return Err(TypeError::TypeMismatch { expected: left_type.clone(), actual: right_type });
                        }
                        Ok(DataType::Bool)
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type != DataType::Bool {
                            return Err(TypeError::IncompatibleTypes { expected: DataType::Bool, actual: left_type });
                        }
                        if right_type != DataType::Bool {
                            return Err(TypeError::IncompatibleTypes { expected: DataType::Bool, actual: right_type });
                        }
                        Ok(DataType::Bool)
                    }
                    BinaryOp::Assign => Ok(right_type),
                }
            }
            ASTNode::UnaryOp { op, expr } => {
                let expr_type = infer_type(*expr, arena, string_table, type_env)?;
                match op {
                    UnaryOp::Neg => {
                        if expr_type != DataType::Int64 {
                            return Err(TypeError::IncompatibleTypes { expected: DataType::Int64, actual: expr_type });
                        }
                        Ok(DataType::Int64)
                    }
                    UnaryOp::Not => {
                        if expr_type != DataType::Bool {
                            return Err(TypeError::IncompatibleTypes { expected: DataType::Bool, actual: expr_type });
                        }
                        Ok(DataType::Bool)
                    }
                }
            }
            ASTNode::Lambda { params, body } => {
                let mut body_env = type_env.clone();
                if let Ok(ASTNode::Ident(param_idx)) = arena.get(*params) {
                    if let Some(param_name) = string_table.get(*param_idx) {
                        body_env.insert(param_name.to_string(), DataType::Int64);
                    }
                }
                let body_type = infer_type(*body, arena, string_table, &body_env)?;
                // Return Int64 for now (simplified; real implementation would need Function type)
                Ok(body_type)
            }
            ASTNode::Call { func, args: _ } => {
                let func_type = infer_type(*func, arena, string_table, type_env)?;
                Ok(func_type)
            }
            ASTNode::List { head, .. } => {
                infer_type(*head, arena, string_table, type_env)
            }
            ASTNode::Set { elems } => {
                if *elems == NodeID::INVALID {
                    Ok(DataType::Text)
                } else {
                    infer_type(*elems, arena, string_table, type_env)
                }
            }
            ASTNode::ForAll { body, .. } | ASTNode::Mu { body, .. } | ASTNode::Exists { body, .. } => {
                infer_type(*body, arena, string_table, type_env)
            }
            ASTNode::Empty => Ok(DataType::Text),
            _ => Ok(DataType::Text),
        }
    } else {
        Err(TypeError::UnknownType { expr: "invalid node".to_string() })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_int_literal() {
        let mut arena = Arena::new(100);
        let st = StringTable::new();
        let env = TypeEnv::new();
        let node = arena.allocate(ASTNode::Int(42)).unwrap();
        assert_eq!(infer_type(node, &arena, &st, &env).unwrap(), DataType::Int64);
    }
    #[test]
    fn test_text_literal() {
        let mut arena = Arena::new(100);
        let st = StringTable::new();
        let env = TypeEnv::new();
        let node = arena.allocate(ASTNode::Str(0)).unwrap();
        assert_eq!(infer_type(node, &arena, &st, &env).unwrap(), DataType::Text);
    }
    #[test]
    fn test_arithmetic() {
        let mut arena = Arena::new(100);
        let st = StringTable::new();
        let env = TypeEnv::new();
        let left = arena.allocate(ASTNode::Int(5)).unwrap();
        let right = arena.allocate(ASTNode::Int(3)).unwrap();
        let node = arena.allocate(ASTNode::BinOp { op: BinaryOp::Add, left, right }).unwrap();
        assert_eq!(infer_type(node, &arena, &st, &env).unwrap(), DataType::Int64);
    }
    #[test]
    fn test_concat_type_error() {
        let mut arena = Arena::new(100);
        let st = StringTable::new();
        let env = TypeEnv::new();
        let left = arena.allocate(ASTNode::Int(5)).unwrap();
        let right = arena.allocate(ASTNode::Str(0)).unwrap();
        let node = arena.allocate(ASTNode::BinOp { op: BinaryOp::Concat, left, right }).unwrap();
        assert!(matches!(infer_type(node, &arena, &st, &env).unwrap_err(), TypeError::IncompatibleTypes { .. }));
    }
    #[test]
    fn test_lambda_type() {
        let mut arena = Arena::new(100);
        let mut st = StringTable::new();
        let env = TypeEnv::new();
        let param_idx = st.intern("س");
        let param = arena.allocate(ASTNode::Ident(param_idx)).unwrap();
        let param_ref = arena.allocate(ASTNode::Ident(param_idx)).unwrap();
        let param_ref2 = arena.allocate(ASTNode::Ident(param_idx)).unwrap();
        let body = arena.allocate(ASTNode::BinOp { op: BinaryOp::Mul, left: param_ref, right: param_ref2 }).unwrap();
        let lambda = arena.allocate(ASTNode::Lambda { params: param, body }).unwrap();
        assert_eq!(infer_type(lambda, &arena, &st, &env).unwrap(), DataType::Int64);
    }
}

// ═══════════════════════════════════════════════════════════
// Shape Inference Helpers for Tensor Operations
// ═══════════════════════════════════════════════════════════
/// Infer output shape for matrix multiplication: (M x K) @ (K x N) -> (M x N)
pub fn infer_matmul_shape(lhs_shape: &Shape, rhs_shape: &Shape) -> Result<Shape, TypeError> {
    if lhs_shape.rank() != 2 || rhs_shape.rank() != 2 {
        return Err(TypeError::MatMulRequires2D {
            lhs_rank: lhs_shape.rank(),
            rhs_rank: rhs_shape.rank(),
        });
    }
    let m = lhs_shape.dims[0];
    let k1 = lhs_shape.dims[1];
    let k2 = rhs_shape.dims[0];
    let n = rhs_shape.dims[1];
    if k1 != k2 {
        return Err(TypeError::MatMulDimMismatch {
            lhs_inner: k1,
            rhs_inner: k2,
        });
    }
    Ok(Shape::new(vec![m, n]))
}
/// Infer output shape for element-wise operations with NumPy-style broadcasting
pub fn infer_broadcast_shape(shape1: &Shape, shape2: &Shape) -> Result<Shape, TypeError> {
    if !shape1.can_broadcast(shape2) {
        return Err(TypeError::BroadcastIncompatible {
            shape1: shape1.clone(),
            shape2: shape2.clone(),
        });
    }
    // Compute the broadcasted shape
    let max_rank = shape1.rank().max(shape2.rank());
    let mut s1 = shape1.dims.clone();
    let mut s2 = shape2.dims.clone();
    while s1.len() < max_rank { s1.insert(0, 1); }
    while s2.len() < max_rank { s2.insert(0, 1); }
    let result_dims = s1.iter()
        .zip(s2.iter())
        .map(|(d1, d2)| d1.max(d2))
        .collect();
    Ok(Shape::new(result_dims))
}

// ═══════════════════════════════════════════════════════════
// Type Checking for New Types (Tensor & Function)
// ═══════════════════════════════════════════════════════════
/// Check if two DataTypes are compatible for assignment
pub fn check_type_compatible(expected: &DataType, actual: &DataType) -> Result<(), TypeError> {
    match (expected, actual) {
        (DataType::Int64, DataType::Int64) => Ok(()),
        (DataType::Float64, DataType::Float64) => Ok(()),
        (DataType::Bool, DataType::Bool) => Ok(()),
        (DataType::Text, DataType::Text) => Ok(()),
        // Tensor compatibility: check shape and dtype
        (DataType::Tensor { dtype: d1, shape: s1 }, DataType::Tensor { dtype: d2, shape: s2 }) => {
            if d1 != d2 {
                return Err(TypeError::TypeMismatch {
                    expected: expected.clone(),
                    actual: actual.clone(),
                });
            }
            // Check broadcasting compatibility
            if !s1.can_broadcast(s2) {
                return Err(TypeError::BroadcastIncompatible {
                    shape1: s1.clone(),
                    shape2: s2.clone(),
                });
            }
            Ok(())
        }
        // Function type compatibility: check arg types and return type
        (DataType::Function { arg_types: a1, return_type: r1 }, 
         DataType::Function { arg_types: a2, return_type: r2 }) => {
            if a1.len() != a2.len() {
                return Err(TypeError::TypeMismatch {
                    expected: expected.clone(),
                    actual: actual.clone(),
                });
            }
            for (t1, t2) in a1.iter().zip(a2.iter()) {
                check_type_compatible(t1, t2)?;
            }
            check_type_compatible(r1, r2)?;
            Ok(())
        }
        _ => Err(TypeError::TypeMismatch {
            expected: expected.clone(),
            actual: actual.clone(),
        }),
    }
}
/// Infer type for a Tensor creation expression
pub fn infer_tensor_type(shape: &Shape, dtype: &TensorDtype) -> DataType {
    DataType::Tensor {
        dtype: *dtype,
        shape: shape.clone(),
    }
}
/// Infer type for a Function expression (higher-order function)
pub fn infer_function_type(arg_types: Vec<DataType>, return_type: DataType) -> DataType {
    DataType::Function {
        arg_types,
        return_type: Box::new(return_type),
    }
}
/// Validate a function call with higher-order function arguments
pub fn check_function_call(
    func_type: &DataType,
    arg_types: &[DataType],
) -> Result<DataType, TypeError> {
    match func_type {
        DataType::Function { arg_types: expected_args, return_type } => {
            if expected_args.len() != arg_types.len() {
                return Err(TypeError::TypeMismatch {
                    expected: func_type.clone(),
                    actual: DataType::Text, // Placeholder
                });
            }
            for (expected, actual) in expected_args.iter().zip(arg_types.iter()) {
                check_type_compatible(expected, actual)?;
            }
            Ok(*return_type.clone())
        }
        _ => Err(TypeError::NotATensor {
            expected: func_type.clone(),
        }),
    }
}
