//! # Semantic Bridge — Linking mal_index ↔ mal_arena
//!
//! This module bridges the gap between:
//! - **mal_arena**: computational representation (ASTNode, TypeTag, BinaryOp)
//! - **mal_index**: semantic representation (FeatureID, FeatureCategory)
//!
//! ## Constitutional Compliance
//! - Principle 9 (الوحدة الدلالية): Each ASTNode maps to exactly ONE FeatureID
//! - Principle 1 (الإحكام): Each mapping is deterministic and documented
//! - Principle 6 (الحفظ): No memory allocation on hot paths (static maps)
use crate::{FeatureID, FeatureCategory};
/// Mathematical operators — semantic view of BinaryOp
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MathematicalOperator {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or, Xor,
}
impl MathematicalOperator {
    /// Convert from string representation of BinaryOp
    pub fn from_binary_op_str(op: &str) -> Option<Self> {
        match op {
            "Add" => Some(Self::Add),
            "Sub" => Some(Self::Sub),
            "Mul" => Some(Self::Mul),
            "Div" => Some(Self::Div),
            "Mod" => Some(Self::Mod),
            "Eq" => Some(Self::Eq),
            "Ne" => Some(Self::Ne),
            "Lt" => Some(Self::Lt),
            "Le" => Some(Self::Le),
            "Gt" => Some(Self::Gt),
            "Ge" => Some(Self::Ge),
            "And" => Some(Self::And),
            "Or" => Some(Self::Or),
            "Xor" => Some(Self::Xor),
            _ => None,
        }
    }
    /// Get the FeatureID associated with this operator
    pub fn feature_id(&self) -> FeatureID {
        FeatureID(100) // All binary operators map to a single BIN_OP feature
    }
}
/// Semantic Bridge — bidirectional mapping between computational and semantic views
pub struct SemanticBridge {
    ast_node_to_feature: &'static [(&'static str, FeatureID)],
    category_to_ast_tags: &'static [(FeatureCategory, &'static [&'static str])],
}
impl SemanticBridge {
    /// Create the semantic bridge with static mappings
    pub const fn new() -> Self {
        const AST_TO_FEATURE: &[(&str, FeatureID)] = &[
            ("LinearLet", FeatureID(1)),
            ("Lambda", FeatureID(2)),
            ("Match", FeatureID(3)),
            ("MatchArm", FeatureID(3)),
            ("TypeDecl", FeatureID(4)),
            ("Constructor", FeatureID(4)),
            ("ForAll", FeatureID(200)),
            ("Exists", FeatureID(201)),
            ("Mu", FeatureID(202)),
            ("Set", FeatureID(203)),
            ("SetMembership", FeatureID(204)),
        ];
        const CATEGORY_TO_TAGS: &[(FeatureCategory, &[&str])] = &[
            (FeatureCategory::Memory, &["LinearLet"]),
            (FeatureCategory::TypeSystem, &["Lambda", "TypeDecl", "Constructor"]),
            (FeatureCategory::Abstraction, &["Match", "MatchArm"]),
            (FeatureCategory::Query, &["ForAll", "Exists", "Mu", "Set", "SetMembership"]),
        ];
        Self {
            ast_node_to_feature: AST_TO_FEATURE,
            category_to_ast_tags: CATEGORY_TO_TAGS,
        }
    }
    pub fn ast_to_feature(&self, node_name: &str) -> Option<FeatureID> {
        self.ast_node_to_feature
            .iter()
            .find(|(name, _)| *name == node_name)
            .map(|(_, id)| *id)
    }
    pub fn feature_to_ast(&self, id: FeatureID) -> Vec<&'static str> {
        self.ast_node_to_feature
            .iter()
            .filter(|(_, fid)| *fid == id)
            .map(|(name, _)| *name)
            .collect()
    }
    pub fn category_tags(&self, category: FeatureCategory) -> Vec<&'static str> {
        self.category_to_ast_tags
            .iter()
            .find(|(cat, _)| *cat == category)
            .map(|(_, tags)| tags.to_vec())
            .unwrap_or_default()
    }
    pub fn verify_completeness(&self) -> bool {
        self.ast_to_feature("LinearLet").is_some()
            && self.ast_to_feature("Lambda").is_some()
            && self.ast_to_feature("Match").is_some()
    }
}
impl Default for SemanticBridge {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ast_to_feature_mapping() {
        let bridge = SemanticBridge::new();
        assert_eq!(bridge.ast_to_feature("LinearLet"), Some(FeatureID(1)));
        assert_eq!(bridge.ast_to_feature("Lambda"), Some(FeatureID(2)));
        assert_eq!(bridge.ast_to_feature("Match"), Some(FeatureID(3)));
        assert_eq!(bridge.ast_to_feature("NonExistent"), None);
    }
    #[test]
    fn test_feature_to_ast_mapping() {
        let bridge = SemanticBridge::new();
        let ast_nodes = bridge.feature_to_ast(FeatureID(3));
        assert!(ast_nodes.contains(&"Match"));
        assert!(ast_nodes.contains(&"MatchArm"));
        assert_eq!(ast_nodes.len(), 2);
        let ast_nodes = bridge.feature_to_ast(FeatureID(1));
        assert_eq!(ast_nodes, vec!["LinearLet"]);
    }
    #[test]
    fn test_category_tags() {
        let bridge = SemanticBridge::new();
        let memory_tags = bridge.category_tags(FeatureCategory::Memory);
        assert!(memory_tags.contains(&"LinearLet"));
        let abstraction_tags = bridge.category_tags(FeatureCategory::Abstraction);
        assert!(abstraction_tags.contains(&"Match"));
    }
    #[test]
    fn test_mathematical_operator_mapping() {
        assert_eq!(
            MathematicalOperator::from_binary_op_str("Add"),
            Some(MathematicalOperator::Add)
        );
        assert_eq!(MathematicalOperator::from_binary_op_str("Unknown"), None);
        assert_eq!(
            MathematicalOperator::Add.feature_id(),
            MathematicalOperator::Sub.feature_id()
        );
    }
    #[test]
    fn test_bridge_completeness() {
        let bridge = SemanticBridge::new();
        assert!(bridge.verify_completeness());
    }
}
