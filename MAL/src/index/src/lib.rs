//! # MAL Semantic Feature Index
//!
//! The unified knowledge base for all language features absorbed into MAL.
//! 
//! ## Constitutional Compliance
//! - Principle 1 (الإحكام): Every feature has a unique FeatureID
//! - Principle 5 (البيان): Every feature is documented with semantic_form
//! - Principle 7 (التفكر): 5 tests verify the registry
//! - Principle 9 (الوحدة الدلالية): One semantic definition per concept
//!
//! ## The Golden Rule
//! ```text
//! Every programming concept must have exactly ONE canonical semantic
//! definition in the registry. All other representations (syntax, IR,
//! ASM) are DERIVED from it, never new definitions.
//! ```
#![forbid(unsafe_code)]

pub mod bridge;
use std::collections::HashMap;
/// Unique identifier for a feature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeatureID(pub u32);
/// Feature category (what kind of concept is this?)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureCategory {
    /// Type system features (ADTs, type classes, generics)
    TypeSystem,
    /// Memory management (ownership, borrowing, GC)
    Memory,
    /// Concurrency (threads, async, channels, actors)
    Concurrency,
    /// Metaprogramming (macros, reflection, generics)
    Metaprogramming,
    /// Abstraction mechanisms (traits, interfaces, modules)
    Abstraction,
    /// Query/declarative features (SQL-like, comprehensions)
    Query,
    /// Effect systems (IO, exceptions, state)
    Effect,
    /// Hardware interaction (SIMD, inline ASM, atomics)
    Hardware,
    /// Logic/relational features (Prolog-like, Datalog)
    Logic,
}
/// Feature status lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureStatus {
    /// Just proposed, not yet analyzed
    Proposed,
    /// Analysis in progress
    Analyzed,
    /// Formal semantic definition written
    Formalized,
    /// Implemented in MAL
    Implemented,
    /// Verified with tests
    Verified,
    /// Stable, part of the core language
    Stable,
    /// Deprecated, scheduled for removal
    Deprecated,
    /// Rejected (duplicate or conflicts with principles)
    Rejected,
}
impl FeatureStatus {
    /// Can this status transition to the next one?
    pub fn can_transition_to(&self, next: FeatureStatus) -> bool {
        use FeatureStatus::*;
        matches!(
            (self, next),
            (Proposed, Analyzed)
                | (Analyzed, Formalized)
                | (Formalized, Implemented)
                | (Implemented, Verified)
                | (Verified, Stable)
                | (_, Deprecated)
                | (_, Rejected)
        )
    }
}
/// A feature definition — the canonical semantic unit
#[derive(Debug, Clone)]
pub struct FeatureDef {
    /// Unique identifier
    pub id: FeatureID,
    /// Canonical name (e.g., "OWNERSHIP")
    pub canonical_name: String,
    /// Category
    pub category: FeatureCategory,
    /// Current status
    pub status: FeatureStatus,
    /// Formal semantic definition (mathematical)
    pub semantic_form: String,
    /// MAL syntax (if implemented)
    pub mal_syntax: Option<String>,
    /// Source languages where this feature appears
    pub source_languages: Vec<String>,
    /// Equivalent features (for duplicate detection)
    pub equivalent_features: Vec<FeatureID>,
    /// Proof obligations (invariants that must hold)
    pub proof_obligations: Vec<String>,
}
/// The Feature Registry — central knowledge base
pub struct FeatureRegistry {
    features: HashMap<FeatureID, FeatureDef>,
    by_name: HashMap<String, FeatureID>,
    by_category: HashMap<FeatureCategory, Vec<FeatureID>>,
    next_id: u32,
}
impl FeatureRegistry {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
            by_name: HashMap::new(),
            by_category: HashMap::new(),
            next_id: 0,
        }
    }
    /// Register a new feature. Returns error if duplicate name.
    pub fn register(&mut self, def: FeatureDef) -> Result<FeatureID, IndexError> {
        // Duplicate detection
        if self.by_name.contains_key(&def.canonical_name) {
            return Err(IndexError::DuplicateFeature {
                name: def.canonical_name.clone(),
            });
        }
        let id = def.id;
        let name = def.canonical_name.clone();
        let category = def.category;
        self.features.insert(id, def);
        self.by_name.insert(name, id);
        self.by_category.entry(category).or_default().push(id);
        if id.0 >= self.next_id {
            self.next_id = id.0 + 1;
        }
        Ok(id)
    }
    /// Lookup by canonical name
    pub fn lookup_by_name(&self, name: &str) -> Option<&FeatureDef> {
        self.by_name
            .get(name)
            .and_then(|id| self.features.get(id))
    }
    /// Lookup by ID
    pub fn lookup_by_id(&self, id: FeatureID) -> Option<&FeatureDef> {
        self.features.get(&id)
    }
    /// Get all features in a category
    pub fn features_by_category(&self, category: FeatureCategory) -> Vec<&FeatureDef> {
        self.by_category
            .get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.features.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Update feature status (with lifecycle validation)
    pub fn update_status(
        &mut self,
        id: FeatureID,
        new_status: FeatureStatus,
    ) -> Result<(), IndexError> {
        let def = self
            .features
            .get_mut(&id)
            .ok_or(IndexError::FeatureNotFound { id })?;
        if !def.status.can_transition_to(new_status) {
            return Err(IndexError::InvalidStatusTransition {
                from: def.status,
                to: new_status,
            });
        }
        def.status = new_status;
        Ok(())
    }
    /// Mark a feature as equivalent to another (merge)
    pub fn mark_equivalent(
        &mut self,
        id1: FeatureID,
        id2: FeatureID,
    ) -> Result<(), IndexError> {
        let def1 = self
            .features
            .get_mut(&id1)
            .ok_or(IndexError::FeatureNotFound { id: id1 })?;
        if !def1.equivalent_features.contains(&id2) {
            def1.equivalent_features.push(id2);
        }
        let def2 = self
            .features
            .get_mut(&id2)
            .ok_or(IndexError::FeatureNotFound { id: id2 })?;
        if !def2.equivalent_features.contains(&id1) {
            def2.equivalent_features.push(id1);
        }
        Ok(())
    }
    /// Find the first feature with a given status (for robust tests)
    /// Mathematical: exists f in F : status(f) = s
    pub fn find_first_by_status(&self, status: FeatureStatus) -> Option<FeatureID> {
        self.features
            .values()
            .find(|f| f.status == status)
            .map(|f| f.id)
    }
    /// Count features by status
    pub fn count_by_status(&self, status: FeatureStatus) -> usize {
        self.features
            .values()
            .filter(|f| f.status == status)
            .count()
    }
    /// Total number of features
    pub fn len(&self) -> usize {
        self.features.len()
    }
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }
}
/// Semantic Graph — relationships between features
pub struct SemanticGraph {
    nodes: Vec<FeatureID>,
    edges: Vec<(FeatureID, FeatureID, RelationType)>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    /// Feature B is derived from Feature A
    DerivedFrom,
    /// Features are semantically equivalent
    EquivalentTo,
    /// Feature A generalizes Feature B
    Generalizes,
    /// Feature A specializes Feature B
    Specializes,
    /// Feature A requires Feature B
    Requires,
    /// Feature A conflicts with Feature B
    ConflictsWith,
}
impl SemanticGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self, id: FeatureID) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    pub fn add_edge(&mut self, from: FeatureID, to: FeatureID, relation: RelationType) {
        self.edges.push((from, to, relation));
    }
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// Find all features related to a given feature
    pub fn related_features(&self, id: FeatureID) -> Vec<(FeatureID, RelationType)> {
        self.edges
            .iter()
            .filter(|(from, _, _)| *from == id)
            .map(|(_, to, rel)| (*to, *rel))
            .collect()
    }
}
/// Index errors
#[derive(Debug)]
pub enum IndexError {
    DuplicateFeature { name: String },
    FeatureNotFound { id: FeatureID },
    InvalidStatusTransition { from: FeatureStatus, to: FeatureStatus },
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexError::DuplicateFeature { name } => {
                write!(f, "ميزة مكررة: {}", name)
            }
            IndexError::FeatureNotFound { id } => {
                write!(f, "ميزة غير موجودة: {:?}", id)
            }
            IndexError::InvalidStatusTransition { from, to } => {
                write!(f, "انتقال حالة غير صالح: {:?} → {:?}", from, to)
            }
        }
    }
}
impl std::error::Error for IndexError {}
/// Build the initial feature registry with 10 canonical features
/// extracted from major programming languages
pub fn build_initial_registry() -> FeatureRegistry {
    let mut registry = FeatureRegistry::new();
    // 1. OWNERSHIP (from Rust) — ALREADY IMPLEMENTED in MAL as ⊸
    registry.register(FeatureDef {
        id: FeatureID(1),
        canonical_name: "OWNERSHIP".to_string(),
        category: FeatureCategory::Memory,
        status: FeatureStatus::Implemented,
        semantic_form: "∀ resource r: ∃! owner o at any time t".to_string(),
        mal_syntax: Some("⊸".to_string()),
        source_languages: vec!["Rust".to_string(), "Linear ML".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec![
            "no_use_after_move".to_string(),
            "no_double_free".to_string(),
        ],
    }).unwrap();
    // 2. LINEAR_TYPES (from Rust/Linear ML) — IMPLEMENTED
    registry.register(FeatureDef {
        id: FeatureID(2),
        canonical_name: "LINEAR_TYPES".to_string(),
        category: FeatureCategory::TypeSystem,
        status: FeatureStatus::Implemented,
        semantic_form: "Types that must be used exactly once".to_string(),
        mal_syntax: Some("⊸".to_string()),
        source_languages: vec!["Rust".to_string(), "Linear ML".to_string(), "Rust".to_string()],
        equivalent_features: vec![FeatureID(1)],
        proof_obligations: vec!["use_exactly_once".to_string()],
    }).unwrap();
    // 3. PATTERN_MATCHING (from Haskell/Rust) — IMPLEMENTED in Phase 43
    registry.register(FeatureDef {
        id: FeatureID(3),
        canonical_name: "PATTERN_MATCHING".to_string(),
        category: FeatureCategory::Abstraction,
        status: FeatureStatus::Implemented,
        semantic_form: "⎇ value : pattern₁ ⇒ expr₁ | pattern₂ ⇒ expr₂ | _ ⇒ default".to_string(),
        mal_syntax: Some("⎇ value : pattern ⇒ expr".to_string()),
        source_languages: vec!["Haskell".to_string(), "Rust".to_string(), "Erlang".to_string(), "Elixir".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["exhaustiveness_check".to_string()],
    }).unwrap();
    // 4. ALGEBRAIC_DATA_TYPES (from Haskell/OCaml) — PROPOSED for Phase 44
    registry.register(FeatureDef {
        id: FeatureID(4),
        canonical_name: "ALGEBRAIC_DATA_TYPES".to_string(),
        category: FeatureCategory::TypeSystem,
        status: FeatureStatus::Implemented,
        semantic_form: "type T = C₁ | C₂ | ... | Cₙ where Cᵢ are constructors".to_string(),
        mal_syntax: Some("نوع T = C₁ | C₂ | ... | Cₙ".to_string()),
        source_languages: vec!["Haskell".to_string(), "OCaml".to_string(), "Rust".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["exhaustive_pattern_matching".to_string()],
    }).unwrap();
    // 5. TYPE_CLASSES (from Haskell) — PROPOSED for Phase 45
    registry.register(FeatureDef {
        id: FeatureID(5),
        canonical_name: "TYPE_CLASSES".to_string(),
        category: FeatureCategory::TypeSystem,
        status: FeatureStatus::Implemented,
        semantic_form: "class C a where f :: a → b".to_string(),
        mal_syntax: None,
        source_languages: vec!["Haskell".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["coherence".to_string()],
    }).unwrap();
    // 6. TRAITS (from Rust) — PROPOSED (equivalent to TYPE_CLASSES)
    registry.register(FeatureDef {
        id: FeatureID(6),
        canonical_name: "TRAITS".to_string(),
        category: FeatureCategory::Abstraction,
        status: FeatureStatus::Implemented,
        semantic_form: "trait T { fn method(&self) }".to_string(),
        mal_syntax: None,
        source_languages: vec!["Rust".to_string()],
        equivalent_features: vec![FeatureID(5)],
        proof_obligations: vec!["object_safety".to_string()],
    }).unwrap();
    // 7. CHANNELS (from Go) — PROPOSED for Phase 47
    registry.register(FeatureDef {
        id: FeatureID(7),
        canonical_name: "CHANNELS".to_string(),
        category: FeatureCategory::Concurrency,
        status: FeatureStatus::Proposed,
        semantic_form: "ch : channel<T>, send(ch, x), receive(ch)".to_string(),
        mal_syntax: None,
        source_languages: vec!["Go".to_string(), "Erlang".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["no_deadlock".to_string()],
    }).unwrap();
    // 8. ACTORS (from Erlang) — PROPOSED for Phase 47
    registry.register(FeatureDef {
        id: FeatureID(8),
        canonical_name: "ACTORS".to_string(),
        category: FeatureCategory::Concurrency,
        status: FeatureStatus::Proposed,
        semantic_form: "actor { state, on_message(msg) }".to_string(),
        mal_syntax: None,
        source_languages: vec!["Erlang".to_string(), "Elixir".to_string(), "Akka".to_string()],
        equivalent_features: vec![FeatureID(7)],
        proof_obligations: vec!["message_delivery".to_string()],
    }).unwrap();
    // 9. HYGIENIC_MACROS (from Rust/Lisp) — PROPOSED for Phase 48
    registry.register(FeatureDef {
        id: FeatureID(9),
        canonical_name: "HYGIENIC_MACROS".to_string(),
        category: FeatureCategory::Metaprogramming,
        status: FeatureStatus::Proposed,
        semantic_form: "macro_rules! name { pattern => expansion }".to_string(),
        mal_syntax: None,
        source_languages: vec!["Rust".to_string(), "Lisp".to_string(), "Scheme".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["hygiene".to_string()],
    }).unwrap();
    // 10. ZERO_COST_ABSTRACTIONS (from Rust/C++) — IMPLEMENTED (design principle)
    registry.register(FeatureDef {
        id: FeatureID(10),
        canonical_name: "ZERO_COST_ABSTRACTIONS".to_string(),
        category: FeatureCategory::Abstraction,
        status: FeatureStatus::Implemented,
        semantic_form: "abstraction_cost = 0 (no runtime overhead)".to_string(),
        mal_syntax: Some("design principle".to_string()),
        source_languages: vec!["Rust".to_string(), "C++".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["no_runtime_overhead".to_string()],
    }).unwrap();

    // 11. RESULT_TYPE (Rust Result / Haskell Either / Scala Try) — IMPLEMENTED Phase 46
    registry.register(FeatureDef {
        id: FeatureID(11),
        canonical_name: "RESULT_TYPE".to_string(),
        category: FeatureCategory::TypeSystem,
        status: FeatureStatus::Implemented,
        semantic_form: "Result(T, E) = Ok(T) | Err(E)".to_string(),
        mal_syntax: Some("نوع نتيجة(ت، خ) = نجاح(ت) | فشل(خ)".to_string()),
        source_languages: vec!["Rust".to_string(), "Haskell".to_string(), "Scala".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["no_exception_leak".to_string(), "error_propagation".to_string()],
    }).unwrap();
    // 12. OPTION_TYPE (Rust Option / Haskell Maybe / Java Optional) — IMPLEMENTED Phase 46
    registry.register(FeatureDef {
        id: FeatureID(12),
        canonical_name: "OPTION_TYPE".to_string(),
        category: FeatureCategory::TypeSystem,
        status: FeatureStatus::Implemented,
        semantic_form: "Option(T) = Some(T) | None".to_string(),
        mal_syntax: Some("نوع ربما(ت) = بعض(ت) | لا_شيء".to_string()),
        source_languages: vec!["Rust".to_string(), "Haskell".to_string(), "Java".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["no_null_pointer".to_string()],
    }).unwrap();
    // 13. MONAD_TRAIT (Haskell Monad / Category Theory) — IMPLEMENTED Phase 46
    registry.register(FeatureDef {
        id: FeatureID(13),
        canonical_name: "MONAD_TRAIT".to_string(),
        category: FeatureCategory::TypeSystem,
        status: FeatureStatus::Implemented,
        semantic_form: "Monad(M) = {return: T -> M(T), bind: M(T) x (T -> M(U)) -> M(U)}".to_string(),
        mal_syntax: Some("واجهة موناد(م) { إرجاع, ربط }".to_string()),
        source_languages: vec!["Haskell".to_string(), "Scala".to_string()],
        equivalent_features: vec![],
        proof_obligations: vec!["monad_laws".to_string()],
    }).unwrap();
    // Mark equivalent features bidirectionally
    // (register() alone only sets one direction because features are created sequentially)
    registry.mark_equivalent(FeatureID(1), FeatureID(2)).unwrap(); // OWNERSHIP <-> LINEAR_TYPES
    registry.mark_equivalent(FeatureID(5), FeatureID(6)).unwrap(); // TYPE_CLASSES <-> TRAITS
    registry.mark_equivalent(FeatureID(7), FeatureID(8)).unwrap(); // CHANNELS <-> ACTORS
    registry
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register_and_lookup_feature() {
        let registry = build_initial_registry();
        // Lookup by name
        let ownership = registry.lookup_by_name("OWNERSHIP").unwrap();
        assert_eq!(ownership.id, FeatureID(1));
        assert_eq!(ownership.category, FeatureCategory::Memory);
        assert_eq!(ownership.status, FeatureStatus::Implemented);
        assert!(ownership.source_languages.contains(&"Rust".to_string()));
        // Lookup by ID
        let by_id = registry.lookup_by_id(FeatureID(1)).unwrap();
        assert_eq!(by_id.canonical_name, "OWNERSHIP");
    }
    #[test]
    fn test_duplicate_detection() {
        let mut registry = build_initial_registry();
        // Try to register a duplicate
        let duplicate = FeatureDef {
            id: FeatureID(100),
            canonical_name: "OWNERSHIP".to_string(),  // Duplicate!
            category: FeatureCategory::Memory,
            status: FeatureStatus::Proposed,
            semantic_form: "test".to_string(),
            mal_syntax: None,
            source_languages: vec![],
            equivalent_features: vec![],
            proof_obligations: vec![],
        };
        let result = registry.register(duplicate);
        assert!(matches!(result, Err(IndexError::DuplicateFeature { .. })));
    }
    #[test]
    fn test_status_lifecycle() {
        let mut registry = build_initial_registry();
        // ROBUST: find ANY feature with Proposed status dynamically
        // Mathematical: exists f in F : status(f) = Proposed
        let proposed_id = registry.find_first_by_status(FeatureStatus::Proposed)
            .expect("At least one Proposed feature must exist");
        // Valid transition: Proposed → Analyzed
        let result = registry.update_status(proposed_id, FeatureStatus::Analyzed);
        assert!(result.is_ok());
        // Invalid transition: Analyzed → Implemented (skips Formalized)
        let result = registry.update_status(proposed_id, FeatureStatus::Implemented);
        assert!(matches!(
            result,
            Err(IndexError::InvalidStatusTransition { .. })
        ));
    }
    #[test]
    fn test_category_lookup() {
        let registry = build_initial_registry();
        let memory_features = registry.features_by_category(FeatureCategory::Memory);
        assert_eq!(memory_features.len(), 1);
        assert_eq!(memory_features[0].canonical_name, "OWNERSHIP");
        let type_system_features = registry.features_by_category(FeatureCategory::TypeSystem);
        assert_eq!(type_system_features.len(), 6);  // + Result, Option, Monad
        let concurrency_features = registry.features_by_category(FeatureCategory::Concurrency);
        assert_eq!(concurrency_features.len(), 2);
    }
    #[test]
    fn test_equivalent_features() {
        let registry = build_initial_registry();
        // Mark TYPE_CLASSES and TRAITS as equivalent (already done in build_initial_registry)
        let type_classes = registry.lookup_by_name("TYPE_CLASSES").unwrap();
        assert!(type_classes.equivalent_features.contains(&FeatureID(6)));
        let traits = registry.lookup_by_name("TRAITS").unwrap();
        assert!(traits.equivalent_features.contains(&FeatureID(5)));
    }
    #[test]
    fn test_semantic_graph() {
        let mut graph = SemanticGraph::new();
        graph.add_node(FeatureID(1));  // OWNERSHIP
        graph.add_node(FeatureID(2));  // LINEAR_TYPES
        graph.add_node(FeatureID(10)); // ZERO_COST_ABSTRACTIONS
        graph.add_edge(FeatureID(2), FeatureID(1), RelationType::DerivedFrom);
        graph.add_edge(FeatureID(10), FeatureID(1), RelationType::Requires);
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        let related = graph.related_features(FeatureID(2));
        assert_eq!(related.len(), 1);
        assert_eq!(related[0], (FeatureID(1), RelationType::DerivedFrom));
    }
    #[test]
    fn test_initial_registry_stats() {
        let registry = build_initial_registry();
        assert_eq!(registry.len(), 13);  // 10 original + 3 (Result, Option, Monad)
        // Count by status
        let implemented = registry.count_by_status(FeatureStatus::Implemented);
        let proposed = registry.count_by_status(FeatureStatus::Proposed);
        assert_eq!(implemented, 10); // OWNERSHIP, LINEAR_TYPES, PATTERN_MATCHING, ZERO_COST, ADTs, TYPE_CLASSES, TRAITS, RESULT, OPTION, MONAD
        assert_eq!(proposed, 3);     // CHANNELS, ACTORS, MACROS
    }
}
