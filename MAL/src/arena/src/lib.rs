//! # MAL Arena — Memory Arena for the Mathematical Arabic Language
//!
//! ## Dependencies
//! - `mal_types` — mathematical foundation types (re-exported)
//!
//! ## Constitutional Compliance
//! - `#![forbid(unsafe_code)]` — no raw pointers allowed
//! - `NodeID(u32)` — typed index, not raw `Handle`
//! - `Box<[ASTNode]>` — single fixed allocation, zero drift in hot path
//! - `Result<..., ArenaError>` — explicit failure modes
//! - Fail-closed: `ABSTAIN` on capacity, invalid ID, or type mismatch

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Typed node identifier. Newtype around u32 prevents accidental arithmetic.
// ═══════════════════════════════════════════════════════════════
// Re-exported from mal_types (mathematical foundation)
// ═══════════════════════════════════════════════════════════════
pub use mal_types::DataType;

/// Errors that can occur when working with the Arena.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArenaError {
    /// The Arena has reached its maximum capacity.
    CapacityExceeded,
    /// An invalid NodeID was provided.
    InvalidNodeID,
    /// A type mismatch occurred during an operation.
    TypeMismatch,
}
pub enum ASTNode {
    Empty,
    Int(i64),
    FixedPoint(i64),
    Str(u32),
    Ident(u32),
    BoolLit(bool),
    BinOp {
        op: BinaryOp,
        left: NodeID,
        right: NodeID,
    },
    UnaryOp {
        op: UnaryOp,
        expr: NodeID,
    },
    Call {
        func: NodeID,
        args: NodeID,
    },
    List {
        head: NodeID,
        tail: NodeID,
    },
    Lambda {
        params: NodeID,
        body: NodeID,
    },
    LinearLet {
        name: NodeID,
        value: NodeID,
        body: NodeID,
    },
    ForAll {
        var: NodeID,
        set: NodeID,
        body: NodeID,
    },
    Exists {
        var: NodeID,
        set: NodeID,
        body: NodeID,
    },
    Set {
        elems: NodeID,
    },
    SetMembership {
        elem: NodeID,
        set: NodeID,
    },
    Mu {
        var: NodeID,
        body: NodeID,
    },
    Match {
        scrutinee: NodeID,
        arms: NodeID,
    },
    MatchArm {
        pattern: Pattern,
        body: NodeID,
    },
    /// Tensor operation node.
    TensorOp {
        op: TensorOpKind,
        args: NodeID,
    },
    /// Automatic differentiation node.
    AutodiffOp {
        op: AutodiffOpKind,
        target: NodeID,
        params: NodeID,
    },
}

/// AST node with inline payload (no heap allocation per node).
#[derive(Debug, Clone)]
/// Tensor operations for AI/ML.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/// Pattern for match expressions (Phase 43).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    /// Wildcard: matches anything, binds nothing.
    Wildcard,
    /// Literal pattern: matches a literal value.
    Literal(i64),
    /// Variable binding: matches anything, binds to name.
    Binding(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeTag {
    Empty,
    Int,
    FixedPoint,
    Str,
    Ident,
    Bool,
    BinOp,
    UnaryOp,
    Call,
    List,
    Lambda,
    LinearLet,
    ForAll,
    Exists,
    Set,
    SetMembership,
    Mu,
    Match,
    MatchArm,
    TensorOp,
    AutodiffOp,
}

impl ASTNode {
    /// Returns the type tag of this node.
    pub fn type_tag(&self) -> TypeTag {
        match self {
            ASTNode::Empty => TypeTag::Empty,
            ASTNode::Int(_) => TypeTag::Int,
            ASTNode::FixedPoint(_) => TypeTag::FixedPoint,
            ASTNode::Str(_) => TypeTag::Str,
            ASTNode::Ident(_) => TypeTag::Ident,
            ASTNode::BinOp { .. } => TypeTag::BinOp,
            ASTNode::Call { .. } => TypeTag::Call,
            ASTNode::List { .. } => TypeTag::List,
            ASTNode::Lambda { .. } => TypeTag::Lambda,
            ASTNode::LinearLet { .. } => TypeTag::LinearLet,
            ASTNode::ForAll { .. } => TypeTag::ForAll,
            ASTNode::SetMembership { .. } => TypeTag::SetMembership,
            ASTNode::Mu { .. } => TypeTag::Mu,
            ASTNode::Exists { .. } => TypeTag::Exists,
            ASTNode::Set { .. } => TypeTag::Set,
            ASTNode::BoolLit(_) => TypeTag::Bool,
            ASTNode::UnaryOp { .. } => TypeTag::UnaryOp,
            ASTNode::Match { .. } => TypeTag::Match,
            ASTNode::MatchArm { .. } => TypeTag::MatchArm,
            ASTNode::TensorOp { .. } => TypeTag::TensorOp,
            ASTNode::AutodiffOp { .. } => TypeTag::AutodiffOp,
        }
    }
}

/// Deterministic memory arena with fail-closed semantics.
///
/// # Invariants
/// - Capacity is fixed at creation (zero drift).
/// - Storage uses `Box<[ASTNode]>` (single allocation).
/// - All failures return `ArenaError`, never panic.
pub struct Arena {
    nodes: Box<[ASTNode]>,
    capacity: u32,
    next_id: u32,
}

impl Arena {
    /// Create an arena with fixed capacity.
    ///
    /// # Panics
    /// Only if `capacity` exceeds `u32::MAX - 1` (reserved for `NodeID::INVALID`).
    pub fn new(capacity: u32) -> Self {
        assert!(capacity < u32::MAX, "capacity must be < u32::MAX");
        let cap_usize = capacity as usize;
        // Single fixed allocation — no Vec, no reallocation.
        let nodes: Vec<ASTNode> = (0..cap_usize).map(|_| ASTNode::Empty).collect();
        Self {
            nodes: nodes.into_boxed_slice(),
            capacity,
            next_id: 0,
        }
    }

    /// Allocate a new node. Returns `NodeID` or `CapacityExceeded`.
    pub fn allocate(&mut self, node: ASTNode) -> Result<NodeID, ArenaError> {
        if self.next_id >= self.capacity {
            return Err(ArenaError::CapacityExceeded);
        }
        let id = NodeID(self.next_id);
        self.nodes[self.next_id as usize] = node;
        self.next_id += 1;
        Ok(id)
    }

    /// Read a node by ID. Returns `InvalidNodeID` if out of range.
    pub fn get(&self, id: NodeID) -> Result<&ASTNode, ArenaError> {
        if id.0 >= self.next_id || id.0 >= self.capacity {
            return Err(ArenaError::InvalidNodeID);
        }
        Ok(&self.nodes[id.0 as usize])
    }

    /// Read a node with type checking. Returns `TypeMismatch` if wrong type.
    pub fn get_typed(&self, id: NodeID, expected: TypeTag) -> Result<&ASTNode, ArenaError> {
        let node = self.get(id)?;
        if node.type_tag() != expected {
            return Err(ArenaError::TypeMismatch);
        }
        Ok(node)
    }

    /// Number of allocated nodes.
    pub fn used(&self) -> u32 {
        self.next_id
    }

    /// Total capacity.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_allocation() {
        let mut arena = Arena::new(10);
        let id = arena.allocate(ASTNode::Int(42)).expect("alloc");
        assert_eq!(id.0, 0);
        match arena.get(id).unwrap() {
            ASTNode::Int(v) => assert_eq!(*v, 42),
            _ => panic!("wrong type"),
        }
    }

    #[test]
    fn test_invalid_id_rejected() {
        let arena = Arena::new(10);
        // Out of range: rejected
        assert_eq!(arena.get(NodeID(0)).unwrap_err(), ArenaError::InvalidNodeID);
        assert_eq!(
            arena.get(NodeID(999)).unwrap_err(),
            ArenaError::InvalidNodeID
        );
        assert_eq!(
            arena.get(NodeID::INVALID).unwrap_err(),
            ArenaError::InvalidNodeID
        );
    }

    #[test]
    fn test_capacity_exceeded_abstain() {
        let mut arena = Arena::new(3);
        arena.allocate(ASTNode::Int(1)).unwrap();
        arena.allocate(ASTNode::Int(2)).unwrap();
        arena.allocate(ASTNode::Int(3)).unwrap();
        // 4th allocation: fail-closed
        assert_eq!(
            arena.allocate(ASTNode::Int(4)).unwrap_err(),
            ArenaError::CapacityExceeded
        );
        // Existing nodes still valid
        assert!(arena.get(NodeID(0)).is_ok());
        assert!(arena.get(NodeID(2)).is_ok());
    }

    #[test]
    fn test_type_mismatch_abstain() {
        let mut arena = Arena::new(5);
        let id = arena.allocate(ASTNode::Int(7)).unwrap();
        // Correct type: OK
        assert!(arena.get_typed(id, TypeTag::Int).is_ok());
        // Wrong type: rejected
        assert_eq!(
            arena.get_typed(id, TypeTag::Str).unwrap_err(),
            ArenaError::TypeMismatch
        );
        assert_eq!(
            arena.get_typed(id, TypeTag::BinOp).unwrap_err(),
            ArenaError::TypeMismatch
        );
    }

    #[test]
    fn test_stale_handle_rejected() {
        let mut arena = Arena::new(5);
        // Allocate and read: OK
        let id = arena.allocate(ASTNode::Int(100)).unwrap();
        assert!(arena.get(id).is_ok());
        // Simulate "stale" by querying ID beyond allocation count
        let stale = NodeID(4);
        assert_eq!(arena.get(stale).unwrap_err(), ArenaError::InvalidNodeID);
    }

    #[test]
    fn test_10k_nodes_stress() {
        const N: u32 = 10_000;
        let mut arena = Arena::new(N);

        // Allocate 10,000 nodes
        for i in 0..N {
            let id = arena.allocate(ASTNode::Int(i as i64)).expect("alloc");
            assert_eq!(id.0, i);
        }

        assert_eq!(arena.used(), N);

        // Read back all nodes
        for i in 0..N {
            match arena.get(NodeID(i)).unwrap() {
                ASTNode::Int(v) => assert_eq!(*v, i as i64),
                _ => panic!("wrong type at {}", i),
            }
        }

        // One more allocation: fail-closed
        assert_eq!(
            arena.allocate(ASTNode::Int(99999)).unwrap_err(),
            ArenaError::CapacityExceeded
        );
    }

    #[test]
    fn test_lambda_variant() {
        let mut arena = Arena::new(10);
        let param = arena.allocate(ASTNode::Ident(0)).unwrap();
        let body = arena.allocate(ASTNode::Int(42)).unwrap();
        let lambda = arena
            .allocate(ASTNode::Lambda {
                params: param,
                body,
            })
            .unwrap();
        assert!(matches!(arena.get(lambda).unwrap(), ASTNode::Lambda { .. }));
        assert_eq!(arena.get(lambda).unwrap().type_tag(), TypeTag::Lambda);
    }
}
// ═══════════════════════════════════════════════════════════════
// Mathematical Semantics — Arena Allocator
// ═══════════════════════════════════════════════════════════════
//
// Arena: A = {nodes: Vec<Node>, next_id: NodeId}
//
// Allocation:
//   alloc: A × NodeData → (A', NodeId)
//   where A'.nodes = A.nodes ++ [node]
//   and A'.next_id = A.next_id + 1
//
// Invariants:
//   ∀ node n ∈ A.nodes: n.id ∈ [0, A.next_id)
//   ∀ n₁, n₂ ∈ A.nodes: n₁.id ≠ n₂.id (unique IDs)
//   |A.nodes| = A.next_id (size invariant)
//
// Memory Model:
//   ∀ node n: size(n) ∈ {8, 16, 24, 32} bytes
//   ∀ A: total_size(A) = Σ(|n| for n ∈ A.nodes)
//
// Determinism:
//   ∀ allocation sequence S: alloc_sequence(S) is deterministic
//   No heap fragmentation, no GC pauses
// ═══════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════
// COMPILER OPTIMIZATION: Constant Folding Pass
// ═══════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════
// COMPILER OPTIMIZATION: Constant Folding Pass (Corrected)
// ═══════════════════════════════════════════════════════════
