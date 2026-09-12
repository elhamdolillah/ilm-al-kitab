# Phase C1 Design - Comprehensive Mathematics Integration
## Sub-phase C1.1: Linear Logic + Quantifiers
### New ASTNode Variants (7 total)
#### Layer 1: Linear Logic (4 variants)
/// Linear implication: x ⊸ expr (ownership transfer)
LinearLet {
    /// Variable being bound
    name: NodeID,
    /// Value being transferred
    value: NodeID,
    /// Body expression
    body: NodeID,
},
/// Tensor product: A ⊗ B (multiplicative conjunction)
Tensor {
    left: NodeID,
    right: NodeID,
},
/// Par: A ⅋ B (multiplicative disjunction)
Par {
    left: NodeID,
    right: NodeID,
},
/// Of course: !A (exponential, allows weakening)
OfCourse {
    expr: NodeID,
},
