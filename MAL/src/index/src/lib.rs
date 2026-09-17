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
    /// Code generation and compilation backends
    Backend,
}