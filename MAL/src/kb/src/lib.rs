//! # MAL Knowledge Base (MAL-KB)
//!
//! Hybrid triple-store: Relational (mal_index) + Graph + Vector
//!
//! ## Constitutional Compliance
//! - Principle 1 (الإحكام): Formal semantic definitions
//! - Principle 5 (البيان): Every feature documented
//! - Principle 7 (التفكر): 5 tests verify the KB
//! - Principle 9 (الوحدة الدلالية): One canonical definition per concept
#![forbid(unsafe_code)]
use mal_index::FeatureID;
use std::collections::HashMap;
/// Formal semantic definition (Σ component of a Feature)
#[derive(Debug, Clone)]
pub struct FormalSemantics {
    pub formula: String,
    pub axioms: Vec<String>,
    pub inference_rules: Vec<InferenceRule>,
}
/// Inference rule: Premises ⊢ Conclusion [RuleName]
#[derive(Debug, Clone)]
pub struct InferenceRule {
    pub name: String,
    pub premises: Vec<String>,
    pub conclusion: String,
}
/// Node in the knowledge graph
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KBNode {
    Feature(FeatureID),
    Type(String),
    Rule(String),
    Proof(String),
}
/// Edge in the knowledge graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KBRelation {
    DerivesFrom,
    Requires,
    EquivalentTo,
    Generalizes,
    Specializes,
    Proves,
    Implements,
}
/// Knowledge Graph
pub struct KnowledgeGraph {
    nodes: Vec<KBNode>,
    edges: Vec<(KBNode, KBNode, KBRelation)>,
}
impl KnowledgeGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), edges: Vec::new() }
    }
    pub fn add_node(&mut self, node: KBNode) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }
    pub fn add_edge(&mut self, from: KBNode, to: KBNode, relation: KBRelation) {
        self.edges.push((from, to, relation));
    }
    pub fn find_related(&self, node: KBNode, relation: KBRelation) -> Vec<KBNode> {
        self.edges
            .iter()
            .filter(|(from, _, rel)| *from == node && *rel == relation)
            .map(|(_, to, _)| to.clone())
            .collect()
    }
    pub fn proof_obligations(&self, feature: FeatureID) -> Vec<String> {
        let feature_node = KBNode::Feature(feature);
        self.edges
            .iter()
            .filter(|(from, _, rel)| *from == feature_node && *rel == KBRelation::Requires)
            .filter_map(|(_, to, _)| {
                if let KBNode::Rule(name) = to {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
}
impl Default for KnowledgeGraph {
    fn default() -> Self { Self::new() }
}
/// Semantic embedding vector
#[derive(Debug, Clone)]
pub struct Embedding {
    pub feature_id: FeatureID,
    pub vector: Vec<f32>,
    pub dimension: usize,
}
/// Vector store for semantic similarity search
pub struct VectorStore {
    embeddings: HashMap<FeatureID, Embedding>,
    dimension: usize,
}
impl VectorStore {
    pub fn new(dimension: usize) -> Self {
        Self { embeddings: HashMap::new(), dimension }
    }
    pub fn insert(&mut self, embedding: Embedding) {
        assert_eq!(embedding.dimension, self.dimension);
        self.embeddings.insert(embedding.feature_id, embedding);
    }
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 { 0.0 } else { dot / (norm_a * norm_b) }
    }
    pub fn find_similar(&self, query: &[f32], k: usize) -> Vec<(FeatureID, f32)> {
        let mut similarities: Vec<(FeatureID, f32)> = self.embeddings
            .values()
            .map(|e| (e.feature_id, Self::cosine_similarity(&e.vector, query)))
            .collect();
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(k);
        similarities
    }
}
/// The unified MAL Knowledge Base
pub struct MALKnowledgeBase {
    pub semantics: HashMap<FeatureID, FormalSemantics>,
    pub graph: KnowledgeGraph,
    pub vectors: VectorStore,
}
impl MALKnowledgeBase {
    pub fn new() -> Self {
        Self {
            semantics: HashMap::new(),
            graph: KnowledgeGraph::new(),
            vectors: VectorStore::new(128),
        }
    }
    pub fn register_feature(&mut self, id: FeatureID, semantics: FormalSemantics) {
        self.semantics.insert(id, semantics);
        self.graph.add_node(KBNode::Feature(id));
    }
    pub fn get_semantics(&self, id: FeatureID) -> Option<&FormalSemantics> {
        self.semantics.get(&id)
    }
    pub fn get_proof_obligations(&self, id: FeatureID) -> Vec<String> {
        self.graph.proof_obligations(id)
    }
}
impl Default for MALKnowledgeBase {
    fn default() -> Self { Self::new() }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn build_test_kb() -> MALKnowledgeBase {
        let mut kb = MALKnowledgeBase::new();
        kb.register_feature(
            FeatureID(1),
            FormalSemantics {
                formula: "∀ r ∈ Resources, ∃! o ∈ Owners: owns(o, r, t)".to_string(),
                axioms: vec!["unique_ownership".to_string(), "no_use_after_move".to_string()],
                inference_rules: vec![InferenceRule {
                    name: "Move-Semantics".to_string(),
                    premises: vec!["Γ ⊢ e : T^linear".to_string()],
                    conclusion: "Γ ⊢ move(e) : T^linear".to_string(),
                }],
            },
        );
        kb.graph.add_node(KBNode::Rule("no_use_after_move".to_string()));
        kb.graph.add_node(KBNode::Rule("no_double_free".to_string()));
        kb.graph.add_node(KBNode::Type("linear_type".to_string()));
        kb.graph.add_edge(
            KBNode::Feature(FeatureID(1)),
            KBNode::Rule("no_use_after_move".to_string()),
            KBRelation::Requires,
        );
        kb.graph.add_edge(
            KBNode::Feature(FeatureID(1)),
            KBNode::Rule("no_double_free".to_string()),
            KBRelation::Requires,
        );
        kb.vectors.insert(Embedding {
            feature_id: FeatureID(1),
            vector: vec![0.8; 128],
            dimension: 128,
        });
        kb
    }
    #[test]
    fn test_register_and_retrieve_semantics() {
        let kb = build_test_kb();
        let sem = kb.get_semantics(FeatureID(1)).unwrap();
        assert!(sem.formula.contains("Resources"));
        assert_eq!(sem.axioms.len(), 2);
        assert_eq!(sem.inference_rules.len(), 1);
    }
    #[test]
    fn test_proof_obligations() {
        let kb = build_test_kb();
        let obligations = kb.get_proof_obligations(FeatureID(1));
        assert_eq!(obligations.len(), 2);
        assert!(obligations.contains(&"no_use_after_move".to_string()));
    }
    #[test]
    fn test_knowledge_graph_structure() {
        let kb = build_test_kb();
        assert_eq!(kb.graph.node_count(), 4);
        assert_eq!(kb.graph.edge_count(), 2);
    }
    #[test]
    fn test_vector_similarity() {
        let kb = build_test_kb();
        let query = vec![0.75; 128];
        let similar = kb.vectors.find_similar(&query, 3);
        assert!(!similar.is_empty());
        assert_eq!(similar[0].0, FeatureID(1));
    }
    #[test]
    fn test_cosine_similarity_math() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((VectorStore::cosine_similarity(&a, &b) - 1.0).abs() < 0.001);
        let c = vec![0.0, 1.0, 0.0];
        assert!(VectorStore::cosine_similarity(&a, &c).abs() < 0.001);
    }
}
