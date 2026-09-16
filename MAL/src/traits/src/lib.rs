//! # MAL Traits / Typeclasses (Phase 45)
//!
//! Mathematical Foundation:
//!   Trait(C) = ⟨name, {m_i: tau_i}_{i=1}^{n}⟩
//!
//!   Delta rule:
//!     Gamma |- impl T for A,  Gamma |- e_i : tau_i[A/a]
//!     -------------------------------------------------- [Impl]
//!              Gamma |- dict_{T,A} : {m_i -> e_i}
//!
//! This unifies:
//! - Haskell TypeClasses
//! - Rust Traits
//! - Scala Traits
//! - Swift Protocols
//!
//! Constitutional Compliance:
//! - Principle 1 (الإحكام): Unique trait and method names
//! - Principle 2 (التيسير): Arabic keywords (واجهة, تطبيق, لـ)
//! - Principle 4 (الأمانة): Self-kind tracks ownership
//! - Principle 7 (التفكر): 5 tests verify correctness
//! - Principle 9 (الوحدة الدلالية): One trait definition
//! - Principle 11 (الأولوية الرياضية): Sigma formulation
#![forbid(unsafe_code)]
use std::collections::HashMap;
/// Method self-kind: how `self` is passed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfKind {
    /// Consumed: self (move semantics, linear)
    Consumed,
    /// Borrowed: &self (shared reference)
    Borrowed,
    /// Mutably borrowed: &mut self (exclusive reference)
    MutBorrowed,
}
/// Method signature within a trait
#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub self_kind: SelfKind,
    pub param_types: Vec<String>,
    pub return_type: String,
}
impl Method {
    pub fn new(name: String, self_kind: SelfKind) -> Self {
        Self {
            name,
            self_kind,
            param_types: Vec::new(),
            return_type: "()".to_string(),
        }
    }
    pub fn with_return(mut self, ret: String) -> Self {
        self.return_type = ret;
        self
    }
    pub fn with_params(mut self, params: Vec<String>) -> Self {
        self.param_types = params;
        self
    }
    /// Sigma notation: m(self, p1, ...) : ret
    pub fn to_sigma(&self) -> String {
        let self_str = match self.self_kind {
            SelfKind::Consumed => "self",
            SelfKind::Borrowed => "&self",
            SelfKind::MutBorrowed => "&mut self",
        };
        let mut params = vec![self_str.to_string()];
        params.extend(self.param_types.clone());
        format!("{}({}): {}", self.name, params.join(", "), self.return_type)
    }
}
/// Trait definition (equivalent to TypeClass)
#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<Method>,
}
impl Trait {
    pub fn new(name: String) -> Self {
        Self {
            name,
            methods: Vec::new(),
        }
    }
    /// Add a method (Principle 1: unique names)
    pub fn add_method(&mut self, method: Method) -> Result<(), TraitError> {
        if self.methods.iter().any(|m| m.name == method.name) {
            return Err(TraitError::DuplicateMethod {
                trait_name: self.name.clone(),
                method_name: method.name.clone(),
            });
        }
        self.methods.push(method);
        Ok(())
    }
    /// Find a method by name
    pub fn find_method(&self, name: &str) -> Option<&Method> {
        self.methods.iter().find(|m| m.name == name)
    }
    /// Method count
    pub fn method_count(&self) -> usize {
        self.methods.len()
    }
    /// Sigma notation: Trait T = { m1, m2, ... }
    pub fn to_sigma(&self) -> String {
        if self.methods.is_empty() {
            return format!("{} = {{}}", self.name);
        }
        let methods: Vec<String> = self.methods.iter().map(|m| m.to_sigma()).collect();
        format!("{} = {{ {} }}", self.name, methods.join(", "))
    }
}
/// Trait implementation: impl T for A
#[derive(Debug, Clone)]
pub struct Impl {
    pub trait_name: String,
    pub for_type: String,
    pub method_names: Vec<String>,  // names of implemented methods
}
impl Impl {
    pub fn new(trait_name: String, for_type: String) -> Self {
        Self {
            trait_name,
            for_type,
            method_names: Vec::new(),
        }
    }
    /// Add an implemented method
    pub fn add_method(&mut self, method_name: String) {
        if !self.method_names.contains(&method_name) {
            self.method_names.push(method_name);
        }
    }
    /// Check if all trait methods are implemented
    pub fn is_complete(&self, trait_def: &Trait) -> bool {
        trait_def.methods.iter().all(|m| self.method_names.contains(&m.name))
    }
    /// Get missing methods
    pub fn missing_methods<'a>(&self, trait_def: &'a Trait) -> Vec<&'a Method> {
        trait_def.methods.iter()
            .filter(|m| !self.method_names.contains(&m.name))
            .collect()
    }
}
/// Trait Registry: stores all traits and their implementations
pub struct TraitRegistry {
    traits: HashMap<String, Trait>,
    /// Implementations: (trait_name, type_name) -> Impl
    impls: HashMap<(String, String), Impl>,
}
impl TraitRegistry {
    pub fn new() -> Self {
        Self {
            traits: HashMap::new(),
            impls: HashMap::new(),
        }
    }
    /// Register a trait definition
    pub fn register_trait(&mut self, trait_def: Trait) -> Result<(), TraitError> {
        if self.traits.contains_key(&trait_def.name) {
            return Err(TraitError::DuplicateTrait {
                name: trait_def.name.clone(),
            });
        }
        self.traits.insert(trait_def.name.clone(), trait_def);
        Ok(())
    }
    /// Register an implementation (coherence check - Phi)
    pub fn register_impl(&mut self, impl_block: Impl) -> Result<(), TraitError> {
        // Check trait exists
        let trait_def = self.traits.get(&impl_block.trait_name)
            .ok_or_else(|| TraitError::UnknownTrait {
                name: impl_block.trait_name.clone(),
            })?;
        // Check implementation is complete
        if !impl_block.is_complete(trait_def) {
            let missing = impl_block.missing_methods(trait_def);
            return Err(TraitError::IncompleteImpl {
                trait_name: impl_block.trait_name.clone(),
                type_name: impl_block.for_type.clone(),
                missing: missing.iter().map(|m| m.name.clone()).collect(),
            });
        }
        // Coherence check: one impl per (trait, type)
        let key = (impl_block.trait_name.clone(), impl_block.for_type.clone());
        if self.impls.contains_key(&key) {
            return Err(TraitError::CoherenceViolation {
                trait_name: impl_block.trait_name.clone(),
                type_name: impl_block.for_type.clone(),
            });
        }
        self.impls.insert(key, impl_block);
        Ok(())
    }
    /// Resolve a method call: given (type, method), find the impl
    pub fn resolve_method(&self, type_name: &str, method_name: &str) -> Option<(&Trait, &Impl, &Method)> {
        for ((trait_name, t_name), impl_block) in &self.impls {
            if t_name == type_name && impl_block.method_names.contains(&method_name.to_string()) {
                if let Some(trait_def) = self.traits.get(trait_name) {
                    if let Some(method) = trait_def.find_method(method_name) {
                        return Some((trait_def, impl_block, method));
                    }
                }
            }
        }
        None
    }
    /// Get trait by name
    pub fn get_trait(&self, name: &str) -> Option<&Trait> {
        self.traits.get(name)
    }
    /// Count traits
    pub fn trait_count(&self) -> usize {
        self.traits.len()
    }
    /// Count implementations
    pub fn impl_count(&self) -> usize {
        self.impls.len()
    }
}
impl Default for TraitRegistry {
    fn default() -> Self {
        Self::new()
    }
}
/// Trait Errors (Arabic messages — Principle 5: البيان)
#[derive(Debug)]
pub enum TraitError {
    DuplicateTrait { name: String },
    DuplicateMethod { trait_name: String, method_name: String },
    UnknownTrait { name: String },
    IncompleteImpl { trait_name: String, type_name: String, missing: Vec<String> },
    CoherenceViolation { trait_name: String, type_name: String },
}
impl std::fmt::Display for TraitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraitError::DuplicateTrait { name } => {
                write!(f, "واجهة مكررة: {}", name)
            }
            TraitError::DuplicateMethod { trait_name, method_name } => {
                write!(f, "دالة مكررة في {}: {}", trait_name, method_name)
            }
            TraitError::UnknownTrait { name } => {
                write!(f, "واجهة غير معروفة: {}", name)
            }
            TraitError::IncompleteImpl { trait_name, type_name, missing } => {
                write!(f, "تطبيق ناقص لـ {} على {}: {:?}", trait_name, type_name, missing)
            }
            TraitError::CoherenceViolation { trait_name, type_name } => {
                write!(f, "انتهاك التماسك: تطبيقان لـ {} على {}", trait_name, type_name)
            }
        }
    }
}
impl std::error::Error for TraitError {}
#[cfg(test)]
mod tests {
    use super::*;
    /// Test 1: Simple trait definition (interface)
    #[test]
    fn test_simple_trait_definition() {
        let mut drawable = Trait::new("قابل_للرسم".to_string());
        drawable.add_method(
            Method::new("ارسم".to_string(), SelfKind::Borrowed)
                .with_return("فراغ".to_string())
        ).unwrap();
        drawable.add_method(
            Method::new("مساحة".to_string(), SelfKind::Borrowed)
                .with_return("عدد_عشري".to_string())
        ).unwrap();
        assert_eq!(drawable.method_count(), 2);
        assert!(drawable.find_method("ارسم").is_some());
        assert!(drawable.find_method("مساحة").is_some());
        assert!(drawable.find_method("غير_موجود").is_none());
        let sigma = drawable.to_sigma();
        assert!(sigma.contains("قابل_للرسم"));
        assert!(sigma.contains("ارسم"));
        assert!(sigma.contains("مساحة"));
    }
    /// Test 2: Impl for type (implementation)
    #[test]
    fn test_impl_for_type() {
        let mut registry = TraitRegistry::new();
        // Define trait
        let mut printable = Trait::new("قابل_للطباعة".to_string());
        printable.add_method(
            Method::new("اطبع".to_string(), SelfKind::Borrowed)
                .with_return("نص".to_string())
        ).unwrap();
        registry.register_trait(printable).unwrap();
        // Implement for type "نقطة"
        let mut impl_block = Impl::new("قابل_للطباعة".to_string(), "نقطة".to_string());
        impl_block.add_method("اطبع".to_string());
        registry.register_impl(impl_block).unwrap();
        assert_eq!(registry.trait_count(), 1);
        assert_eq!(registry.impl_count(), 1);
    }
    /// Test 3: Method resolution
    #[test]
    fn test_method_resolution() {
        let mut registry = TraitRegistry::new();
        // Define trait
        let mut comparable = Trait::new("قابل_للمقارنة".to_string());
        comparable.add_method(
            Method::new("قارن".to_string(), SelfKind::Borrowed)
                .with_params(vec!["نفس_النوع".to_string()])
                .with_return("عدد_صحيح".to_string())
        ).unwrap();
        registry.register_trait(comparable).unwrap();
        // Implement for "عدد_صحيح"
        let mut impl_block = Impl::new("قابل_للمقارنة".to_string(), "عدد_صحيح".to_string());
        impl_block.add_method("قارن".to_string());
        registry.register_impl(impl_block).unwrap();
        // Resolve: (عدد_صحيح, قارن) should find the impl
        let resolved = registry.resolve_method("عدد_صحيح", "قارن");
        assert!(resolved.is_some());
        let (trait_def, impl_block, method) = resolved.unwrap();
        assert_eq!(trait_def.name, "قابل_للمقارنة");
        assert_eq!(impl_block.for_type, "عدد_صحيح");
        assert_eq!(method.name, "قارن");
        assert_eq!(method.self_kind, SelfKind::Borrowed);
        // Non-existent method should return None
        assert!(registry.resolve_method("عدد_صحيح", "غير_موجود").is_none());
        assert!(registry.resolve_method("نوع_آخر", "قارن").is_none());
    }
    /// Test 4: Coherence check (Phi proof obligation)
    #[test]
    fn test_coherence_check() {
        let mut registry = TraitRegistry::new();
        let mut t = Trait::new("T".to_string());
        t.add_method(Method::new("m".to_string(), SelfKind::Consumed)).unwrap();
        registry.register_trait(t).unwrap();
        // First impl OK
        let mut impl1 = Impl::new("T".to_string(), "A".to_string());
        impl1.add_method("m".to_string());
        registry.register_impl(impl1).unwrap();
        // Second impl for same (trait, type) should fail (coherence)
        let mut impl2 = Impl::new("T".to_string(), "A".to_string());
        impl2.add_method("m".to_string());
        let result = registry.register_impl(impl2);
        assert!(matches!(result, Err(TraitError::CoherenceViolation { .. })));
    }
    /// Test 5: Incomplete impl detection
    #[test]
    fn test_incomplete_impl_detection() {
        let mut registry = TraitRegistry::new();
        let mut t = Trait::new("T".to_string());
        t.add_method(Method::new("m1".to_string(), SelfKind::Borrowed)).unwrap();
        t.add_method(Method::new("m2".to_string(), SelfKind::Borrowed)).unwrap();
        registry.register_trait(t).unwrap();
        // Impl only provides m1, missing m2
        let mut incomplete = Impl::new("T".to_string(), "A".to_string());
        incomplete.add_method("m1".to_string());
        let result = registry.register_impl(incomplete);
        assert!(matches!(result, Err(TraitError::IncompleteImpl { .. })));
        if let Err(TraitError::IncompleteImpl { missing, .. }) = result {
            assert_eq!(missing, vec!["m2".to_string()]);
        }
    }
}
