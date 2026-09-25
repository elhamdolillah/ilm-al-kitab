//! # MAL Types — Mathematical Foundation Types
//!
//! Pure mathematical types for the Mathematical Arabic Language (MAL).
//! Zero dependencies — this is the foundation layer.
//!
//! ## Constitutional Compliance
//! - `#![forbid(unsafe_code)]` — no raw pointers
//! - All types are `Clone`, `Debug`, `PartialEq`, `Eq` where possible
//! - Mathematical rigor: types follow formal relational algebra semantics
//! - No semantic confusion: NULL ≠ 0, NULL ≠ false, NULL ≠ ∅
#![forbid(unsafe_code)]
#![allow(missing_docs)]
// ═══════════════════════════════════════════════════════════
// DATA TYPES (τ)
// ═══════════════════════════════════════════════════════════
/// Core data types for MAL (mathematical foundation).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Boolean values (true/false) — 𝔹
    Bool,
    /// 64-bit signed integers — ℤ₆₄
    Int64,
    /// Unicode text strings — Text
    Text,
    /// Decimal with precision and scale — 𝔻_{p,s}
    Decimal {
        precision: u8,
        scale: u8,
    },
    /// 64-bit IEEE 754 floating point — 𝔽₆₄
    Float64,
    /// Byte sequences — Bytes
    Bytes,
    /// Calendar date — Date
    Date,
    /// Timestamp with optional timezone — Timestamp
    Timestamp {
        with_timezone: bool,
    },
    /// Universally unique identifier — UUID
    Uuid,
    /// JSON document — JSON
    Json,
    // ═══════════════════════════════════════════════════════════
    // AI / Tensor Types Extensions (Phase: ML Infrastructure)
    // ═══════════════════════════════════════════════════════════
    /// Tensor with specific dtype and shape (for AI/ML operations)
    Tensor {
        dtype: TensorDtype,
        shape: Shape,
    },
    /// Reference to a trained model or computational graph
    Model {
        name: String,
    },
    /// First-class function type (Higher-Order Functions support)
    Function {
        /// Argument types
        arg_types: Vec<DataType>,
        /// Return type
        return_type: Box<DataType>,
    },
}
// ═══════════════════════════════════════════════════════════
// AI / Tensor Supporting Types
// ═══════════════════════════════════════════════════════════
/// Data type for tensor elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TensorDtype {
    F32,
    F64,
    I32,
    I64,
    Bool,
}
/// Runtime shape representation for tensors.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shape {
    pub dims: Vec<usize>,
}
impl Shape {
    pub fn new(dims: Vec<usize>) -> Self {
        Self { dims }
    }
    pub fn rank(&self) -> usize {
        self.dims.len()
    }
    pub fn numel(&self) -> usize {
        self.dims.iter().product()
    }
    /// Check if shapes are broadcast-compatible (NumPy-style broadcasting)
    pub fn can_broadcast(&self, other: &Shape) -> bool {
        let max_rank = self.rank().max(other.rank());
        let mut s1 = self.dims.clone();
        let mut s2 = other.dims.clone();
        // Pad with 1s on the left
        while s1.len() < max_rank { s1.insert(0, 1); }
        while s2.len() < max_rank { s2.insert(0, 1); }
        // Check broadcast rules
        for (d1, d2) in s1.iter().zip(s2.iter()) {
            if d1 != d2 && *d1 != 1 && *d2 != 1 {
                return false;
            }
        }
        true
    }
    /// Compute the resulting shape after broadcasting
    pub fn broadcast_shape(&self, other: &Shape) -> Option<Shape> {
        if !self.can_broadcast(other) {
            return None;
        }
        let max_rank = self.rank().max(other.rank());
        let mut s1 = self.dims.clone();
        let mut s2 = other.dims.clone();
        while s1.len() < max_rank { s1.insert(0, 1); }
        while s2.len() < max_rank { s2.insert(0, 1); }
        let result_dims = s1.iter()
            .zip(s2.iter())
            .map(|(&d1, &d2)| d1.max(d2))
            .collect();
        Some(Shape::new(result_dims))
    }
}
