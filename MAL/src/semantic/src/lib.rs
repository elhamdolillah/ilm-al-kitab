//! # MAL Semantic Types (Phase 61)
//!
//! Mathematical Foundation: Classified Semantics with Evidence
//!
//! Honest Caveat (Principle 5 - البيان):
//!   Numbers have clear semantics ONLY in these contexts:
//!   1. Physical quantities (5 kg, 10 m) - confirmed semantics
//!   2. Contextual conventions (الله=66) - traditional, debatable
//!   3. ML-learned semantics - requires training data + proof
//!   4. Numerical coincidences (π_A ≈ π) - NOT theorems
//!
//!   Other claims require contextual evidence and proof.
//!   We REFUSE to claim semantics without evidence.
#![forbid(unsafe_code)]
// ═══════════════════════════════════════════════════════════
// EVIDENCE — Proof and source tracking
// ═══════════════════════════════════════════════════════════
/// Evidence for a semantic claim (Principle 5: honesty requires proof)
#[derive(Debug, Clone, PartialEq)]
pub enum Evidence {
    /// Physical measurement with known uncertainty
    PhysicalMeasurement {
        unit: String,
        uncertainty: f64,
        instrument: Option<String>,
    },
    /// Traditional/historical source
    TraditionalSource {
        book: String,
        page: Option<u32>,
        tradition: String,
    },
    /// ML-trained model with known accuracy
    TrainedModel {
        dataset: String,
        accuracy: f64,
        model_name: String,
    },
    /// Mathematical proof (theorem)
    MathematicalProof {
        theorem: String,
        formal_system: String,
    },
    /// Anecdotal observation (lowest confidence)
    Anecdotal {
        note: String,
        observer: Option<String>,
    },
}
impl Evidence {
    pub fn confidence(&self) -> f64 {
        match self {
            Evidence::PhysicalMeasurement { uncertainty, .. } => {
                (1.0 - uncertainty.min(1.0)).max(0.0)
            }
            Evidence::TraditionalSource { .. } => 0.7,
            Evidence::TrainedModel { accuracy, .. } => *accuracy,
            Evidence::MathematicalProof { .. } => 1.0,
            Evidence::Anecdotal { .. } => 0.3,
        }
    }
    pub fn is_strong(&self) -> bool {
        self.confidence() >= 0.8
    }
}
// ═══════════════════════════════════════════════════════════
// SEMANTIC KIND — Classified meaning types
// ═══════════════════════════════════════════════════════════
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticKind {
    /// Quantitative: physical quantities with units (HIGH confidence)
    Quantitative { unit: String },
    /// Conventional: traditional/cultural conventions (MEDIUM confidence)
    Conventional { tradition: String, context: String },
    /// Contextual: learned from data/context (VARIABLE confidence)
    Contextual { domain: String, training_size: usize },
    /// Coincidental: numerical coincidences, NOT theorems (LOW confidence)
    Coincidental { note: String, numerical_error: f64 },
    /// Lost: semantics lost during computation (ZERO confidence)
    Lost { operation: String, original_meanings: Vec<String> },
}
impl SemanticKind {
    pub fn base_confidence(&self) -> f64 {
        match self {
            SemanticKind::Quantitative { .. } => 1.0,
            SemanticKind::Conventional { .. } => 0.7,
            SemanticKind::Contextual { .. } => 0.6,
            SemanticKind::Coincidental { .. } => 0.1,
            SemanticKind::Lost { .. } => 0.0,
        }
    }
    pub fn is_reliable(&self) -> bool {
        matches!(self, SemanticKind::Quantitative { .. })
    }
}
// ═══════════════════════════════════════════════════════════
// CLASSIFIED NUMBER — Number with honest semantics
// ═══════════════════════════════════════════════════════════
#[derive(Debug, Clone)]
pub struct ClassifiedNumber {
    pub value: f64,
    pub kind: SemanticKind,
    pub evidence: Vec<Evidence>,
    pub confidence: f64,
}
impl ClassifiedNumber {
    pub fn quantitative(value: f64, unit: impl Into<String>) -> Self {
        let unit = unit.into();
        Self {
            value,
            kind: SemanticKind::Quantitative { unit: unit.clone() },
            evidence: vec![Evidence::PhysicalMeasurement {
                unit,
                uncertainty: 0.0,
                instrument: None,
            }],
            confidence: 1.0,
        }
    }
    pub fn conventional(
        value: f64,
        tradition: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        let tradition = tradition.into();
        Self {
            value,
            kind: SemanticKind::Conventional {
                tradition: tradition.clone(),
                context: context.into(),
            },
            evidence: vec![Evidence::TraditionalSource {
                book: tradition,
                page: None,
                tradition: "Hisab al-Jummal".into(),
            }],
            confidence: 0.7,
        }
    }
    pub fn coincidental(value: f64, note: impl Into<String>, numerical_error: f64) -> Self {
        let evidence = vec![Evidence::Anecdotal {
            note: "Numerical coincidence, not a theorem".into(),
            observer: None,
        }];
        Self {
            value,
            kind: SemanticKind::Coincidental {
                note: note.into(),
                numerical_error,
            },
            evidence: evidence.clone(),
            confidence: evidence[0].confidence(),  // = 0.3 (consistent)
        }
    }
    pub fn pure(value: f64) -> Self {
        Self {
            value,
            kind: SemanticKind::Lost {
                operation: "creation".into(),
                original_meanings: vec![],
            },
            evidence: vec![],
            confidence: 0.0,
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        let new_value = self.value + other.value;
        match (&self.kind, &other.kind) {
            (SemanticKind::Quantitative { unit: u1 }, SemanticKind::Quantitative { unit: u2 })
                if u1 == u2 =>
            {
                Self {
                    value: new_value,
                    kind: SemanticKind::Quantitative { unit: u1.clone() },
                    evidence: self.evidence.iter().chain(other.evidence.iter()).cloned().collect(),
                    confidence: (self.confidence + other.confidence) / 2.0,
                }
            }
            _ => Self {
                value: new_value,
                kind: SemanticKind::Lost {
                    operation: "addition".into(),
                    original_meanings: vec![format!("{:?}", self.kind), format!("{:?}", other.kind)],
                },
                evidence: vec![],
                confidence: 0.0,
            },
        }
    }
    pub fn multiply_scalar(&self, factor: f64) -> Self {
        let new_value = self.value * factor;
        match &self.kind {
            SemanticKind::Quantitative { unit } => Self {
                value: new_value,
                kind: SemanticKind::Quantitative { unit: unit.clone() },
                evidence: self.evidence.clone(),
                confidence: self.confidence,
            },
            SemanticKind::Conventional { tradition, context }
                if (factor - 1.0).abs() < 1e-10 =>
            {
                Self {
                    value: new_value,
                    kind: SemanticKind::Conventional {
                        tradition: tradition.clone(),
                        context: context.clone(),
                    },
                    evidence: self.evidence.clone(),
                    confidence: self.confidence,
                }
            }
            _ => Self {
                value: new_value,
                kind: SemanticKind::Lost {
                    operation: format!("multiplication by {}", factor),
                    original_meanings: vec![format!("{:?}", self.kind)],
                },
                evidence: vec![],
                confidence: 0.0,
            },
        }
    }
    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        let total: f64 = self.evidence.iter().map(|e| e.confidence()).sum();
        self.confidence = total / self.evidence.len() as f64;
        self
    }
    pub fn is_reliable(&self) -> bool {
        self.kind.is_reliable() && self.confidence >= 0.8
    }
    pub fn is_coincidence(&self) -> bool {
        matches!(self.kind, SemanticKind::Coincidental { .. })
    }
    pub fn is_lost(&self) -> bool {
        matches!(self.kind, SemanticKind::Lost { .. })
    }
}
// ═══════════════════════════════════════════════════════════
// SEMANTIC REGISTER — Central registry for semantic claims
// ═══════════════════════════════════════════════════════════
#[derive(Debug, Default)]
pub struct SemanticRegistry {
    claims: Vec<(String, ClassifiedNumber)>,
}
impl SemanticRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, name: impl Into<String>, number: ClassifiedNumber) {
        self.claims.push((name.into(), number));
    }
    pub fn find(&self, name: &str) -> Option<&ClassifiedNumber> {
        self.claims.iter().find(|(n, _)| n == name).map(|(_, num)| num)
    }
    pub fn reliable_claims(&self) -> Vec<&ClassifiedNumber> {
        self.claims.iter().filter(|(_, num)| num.is_reliable()).map(|(_, num)| num).collect()
    }
    pub fn coincidences(&self) -> Vec<&ClassifiedNumber> {
        self.claims.iter().filter(|(_, num)| num.is_coincidence()).map(|(_, num)| num).collect()
    }
    pub fn stats(&self) -> RegistryStats {
        let total = self.claims.len();
        let reliable = self.claims.iter().filter(|(_, n)| n.is_reliable()).count();
        let lost = self.claims.iter().filter(|(_, n)| n.is_lost()).count();
        let coincidences = self.claims.iter().filter(|(_, n)| n.is_coincidence()).count();
        RegistryStats {
            total,
            reliable,
            lost,
            coincidences,
            conventional: total - reliable - lost - coincidences,
        }
    }
}
#[derive(Debug)]
pub struct RegistryStats {
    pub total: usize,
    pub reliable: usize,
    pub conventional: usize,
    pub coincidences: usize,
    pub lost: usize,
}
// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quantitative_semantics() {
        let mass = ClassifiedNumber::quantitative(5.0, "kg");
        assert!(mass.is_reliable());
        assert_eq!(mass.confidence, 1.0);
        let mass2 = ClassifiedNumber::quantitative(3.0, "kg");
        let total = mass.add(&mass2);
        assert_eq!(total.value, 8.0);
        assert!(total.is_reliable());
        match total.kind {
            SemanticKind::Quantitative { unit } => assert_eq!(unit, "kg"),
            _ => panic!("Expected Quantitative"),
        }
        let length = ClassifiedNumber::quantitative(10.0, "m");
        let mixed = mass.add(&length);
        assert!(mixed.is_lost());
        assert_eq!(mixed.confidence, 0.0);
        let doubled = mass.multiply_scalar(2.0);
        assert!(doubled.is_reliable());
    }
    #[test]
    fn test_conventional_semantics() {
        let allah = ClassifiedNumber::conventional(66.0, "Hisab al-Jummal", "Name of God");
        assert!(!allah.is_reliable());
        assert_eq!(allah.confidence, 0.7);
        let same = allah.multiply_scalar(1.0);
        assert!(!same.is_lost());
        let muhammad = ClassifiedNumber::conventional(92.0, "Hisab al-Jummal", "Prophet");
        let sum = allah.add(&muhammad);
        assert!(sum.is_lost());
        assert_eq!(sum.confidence, 0.0);
        let doubled = allah.multiply_scalar(2.0);
        assert!(doubled.is_lost());
    }
    #[test]
    fn test_coincidental_semantics() {
        let abjad_pi = ClassifiedNumber::coincidental(
            3.1411547619,
            "Sum of reciprocals of Abjad values",
            0.0004378916,
        );
        assert!(!abjad_pi.is_reliable());
        assert!(abjad_pi.is_coincidence());
        assert_eq!(abjad_pi.confidence, 0.3);  // Anecdotal evidence = 0.3
        if let SemanticKind::Coincidental { numerical_error, .. } = abjad_pi.kind {
            assert!(numerical_error > 0.0);
            assert!(numerical_error < 0.001);
        }
        let quran_phi = ClassifiedNumber::coincidental(1.6181893, "114/QC", 0.0001553);
        assert!(quran_phi.is_coincidence());
    }
    #[test]
    fn test_evidence_system() {
        let physical = Evidence::PhysicalMeasurement {
            unit: "kg".into(),
            uncertainty: 0.01,
            instrument: Some("Scale".into()),
        };
        assert_eq!(physical.confidence(), 0.99);
        assert!(physical.is_strong());
        let traditional = Evidence::TraditionalSource {
            book: "Manual".into(),
            page: Some(42),
            tradition: "Gematria".into(),
        };
        assert_eq!(traditional.confidence(), 0.7);
        assert!(!traditional.is_strong());
        let trained = Evidence::TrainedModel {
            dataset: "Arabic".into(),
            accuracy: 0.85,
            model_name: "BERT".into(),
        };
        assert!(trained.is_strong());
        let proof = Evidence::MathematicalProof {
            theorem: "Pythagoras".into(),
            formal_system: "ZFC".into(),
        };
        assert_eq!(proof.confidence(), 1.0);
        let anecdotal = Evidence::Anecdotal {
            note: "I noticed".into(),
            observer: None,
        };
        assert!(!anecdotal.is_strong());
        let mut num = ClassifiedNumber::coincidental(3.14, "π approx", 0.001);
        assert_eq!(num.confidence, 0.3);  // Initial: Anecdotal = 0.3
        num = num.with_evidence(proof);
        // After: (0.3 + 1.0) / 2 = 0.65
        assert!((num.confidence - 0.65).abs() < 1e-10);
    }
    #[test]
    fn test_semantic_registry() {
        let mut registry = SemanticRegistry::new();
        registry.register("mass", ClassifiedNumber::quantitative(70.0, "kg"));
        registry.register("allah", ClassifiedNumber::conventional(66.0, "Jummal", "Divine"));
        registry.register("pi_abjad", ClassifiedNumber::coincidental(3.14115, "π", 0.0004));
        registry.register("random", ClassifiedNumber::pure(42.0));
        assert!(registry.find("allah").is_some());
        assert_eq!(registry.reliable_claims().len(), 1);
        assert_eq!(registry.coincidences().len(), 1);
        let stats = registry.stats();
        assert_eq!(stats.total, 4);
        assert_eq!(stats.reliable, 1);
        assert_eq!(stats.conventional, 1);
        assert_eq!(stats.coincidences, 1);
        assert_eq!(stats.lost, 1);
    }
}
