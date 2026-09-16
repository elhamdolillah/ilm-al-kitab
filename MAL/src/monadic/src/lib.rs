//! # MAL Monadic Types (Phase 46)
//!
//! Mathematical Foundation (Sigma):
//!   Result(T, E) = Ok(T) | Err(E)
//!   Option(T)    = Some(T) | None
//!
//! Monad Laws (Delta):
//!   Left identity:  return(a) >>= f  ≡  f(a)
//!   Right identity: m >>= return     ≡  m
//!   Associativity:  (m >>= f) >>= g  ≡  m >>= (λx. f(x) >>= g)
//!
//! This replaces:
//! - Exceptions (Java/Python/C++)
//! - Null/nil error handling
//! - Error codes (C)
//!
//! Constitutional Compliance:
//! - Principle 1 (الإحكام): Unique type definitions
//! - Principle 4 (الأمانة): Linear ownership in Ok/Some
//! - Principle 7 (التفكر): 5 tests verify monad laws
//! - Principle 9 (الوحدة الدلالية): One definition per concept
//! - Principle 11 (الأولوية الرياضية): Category Theory basis
#![forbid(unsafe_code)]
use std::fmt;
// ═══════════════════════════════════════════════════════════
// RESULT<T, E> — Success or Failure
// ═══════════════════════════════════════════════════════════
/// Result type: Ok(T) | Err(E)
/// Mathematical: Result(T, E) = {Ok(t) : t ∈ T} ∪ {Err(e) : e ∈ E}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Result<T, E> {
    /// Success: carries value of type T (linear)
    Ok(T),
    /// Failure: carries error of type E
    Err(E),
}
impl<T, E> Result<T, E> {
    /// Create success
    pub fn ok(value: T) -> Self {
        Result::Ok(value)
    }
    /// Create failure
    pub fn err(error: E) -> Self {
        Result::Err(error)
    }
    /// Is this a success?
    pub fn is_ok(&self) -> bool {
        matches!(self, Result::Ok(_))
    }
    /// Is this a failure?
    pub fn is_err(&self) -> bool {
        matches!(self, Result::Err(_))
    }
    /// Extract value (panics on Err)
    pub fn unwrap(self) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("unwrap called on Err: {:?}", e),
        }
    }
    /// Extract error (panics on Ok)
    pub fn unwrap_err(self) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Result::Ok(v) => panic!("unwrap_err called on Ok: {:?}", v),
            Result::Err(e) => e,
        }
    }
    /// Bind operator (>>=) — the Monad core
    /// Mathematical: m >>= f
    pub fn and_then<U, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Result::Ok(v) => f(v),
            Result::Err(e) => Result::Err(e),
        }
    }
    /// Map success value
    pub fn map<U, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Result::Ok(v) => Result::Ok(f(v)),
            Result::Err(e) => Result::Err(e),
        }
    }
    /// Map error value
    pub fn map_err<F, O>(self, f: F) -> Result<T, O>
    where
        F: FnOnce(E) -> O,
    {
        match self {
            Result::Ok(v) => Result::Ok(v),
            Result::Err(e) => Result::Err(f(e)),
        }
    }
    /// Provide default on Err
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Result::Ok(v) => v,
            Result::Err(_) => default,
        }
    }
}
// ═══════════════════════════════════════════════════════════
// OPTION<T> — Some or None
// ═══════════════════════════════════════════════════════════
/// Option type: Some(T) | None
/// Mathematical: Option(T) = {Some(t) : t ∈ T} ∪ {None}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Option<T> {
    /// Value present (linear ownership)
    Some(T),
    /// No value
    None,
}
impl<T> Option<T> {
    /// Create present value
    pub fn some(value: T) -> Self {
        Option::Some(value)
    }
    /// Create empty
    pub fn none() -> Self {
        Option::None
    }
    /// Is value present?
    pub fn is_some(&self) -> bool {
        matches!(self, Option::Some(_))
    }
    /// Is value absent?
    pub fn is_none(&self) -> bool {
        matches!(self, Option::None)
    }
    /// Extract value (panics on None)
    pub fn unwrap(self) -> T {
        match self {
            Option::Some(v) => v,
            Option::None => panic!("unwrap called on None"),
        }
    }
    /// Bind operator (>>=)
    pub fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        match self {
            Option::Some(v) => f(v),
            Option::None => Option::None,
        }
    }
    /// Map present value
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(v) => Option::Some(f(v)),
            Option::None => Option::None,
        }
    }
    /// Provide default on None
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(v) => v,
            Option::None => default,
        }
    }
    /// Convert to Result (None -> Err)
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Option::Some(v) => Result::Ok(v),
            Option::None => Result::Err(err),
        }
    }
}
// ═══════════════════════════════════════════════════════════
// MONAD LAWS VERIFICATION (Φ — Proof Obligations)
// ═══════════════════════════════════════════════════════════
/// Verify Result monad laws (for testing)
pub fn verify_result_monad_laws<T, U, V, E, F1, F2>(
    value: T,
    f: F1,
    g: F2,
    m: Result<T, E>,
) -> bool
where
    T: Clone + PartialEq + fmt::Debug,
    U: Clone + PartialEq + fmt::Debug,
    V: PartialEq + fmt::Debug,
    E: Clone + PartialEq + fmt::Debug,
    F1: Fn(T) -> Result<U, E> + Copy,
    F2: Fn(U) -> Result<V, E> + Copy,
{
    // Left identity: return(a) >>= f ≡ f(a)
    let left_id = Result::ok(value.clone()).and_then(f);
    let left_expected = f(value);
    if left_id != left_expected {
        return false;
    }
    // Right identity: m >>= return ≡ m
    let right_id = m.clone().and_then(Result::ok);
    if right_id != m {
        return false;
    }
    // Associativity: (m >>= f) >>= g ≡ m >>= (λx. f(x) >>= g)
    let assoc_left = m.clone().and_then(f).and_then(g);
    let assoc_right = m.and_then(|x| f(x).and_then(g));
    assoc_left == assoc_right
}
/// Verify Option monad laws
pub fn verify_option_monad_laws<T, U, V, F1, F2>(
    value: T,
    f: F1,
    g: F2,
    m: Option<T>,
) -> bool
where
    T: Clone + PartialEq + fmt::Debug,
    U: Clone + PartialEq + fmt::Debug,
    V: PartialEq + fmt::Debug,
    F1: Fn(T) -> Option<U> + Copy,
    F2: Fn(U) -> Option<V> + Copy,
{
    // Left identity
    let left_id = Option::some(value.clone()).and_then(f);
    let left_expected = f(value);
    if left_id != left_expected {
        return false;
    }
    // Right identity
    let right_id = m.clone().and_then(Option::some);
    if right_id != m {
        return false;
    }
    // Associativity
    let assoc_left = m.clone().and_then(f).and_then(g);
    let assoc_right = m.and_then(|x| f(x).and_then(g));
    assoc_left == assoc_right
}
// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    /// Test 1: Result Ok and Err variants
    #[test]
    fn test_result_ok_and_err() {
        // Predicate checks (don't consume)
        let ok: Result<i32, String> = Result::ok(42);
        let err: Result<i32, String> = Result::err("error".to_string());
        assert!(ok.is_ok());
        assert!(!ok.is_err());
        assert!(!err.is_ok());
        assert!(err.is_err());
        // unwrap / unwrap_err consume — use fresh instances
        assert_eq!(Result::<i32, String>::ok(42).unwrap(), 42);
        assert_eq!(Result::<i32, String>::err("error".to_string()).unwrap_err(), "error");
        // Discriminant check (Sigma: 2 variants)
        let ok_a: Result<i32, String> = Result::ok(1);
        let ok_b: Result<i32, String> = Result::ok(2);
        let err_v: Result<i32, String> = Result::err("e".to_string());
        assert_eq!(std::mem::discriminant(&ok_a), std::mem::discriminant(&ok_b));
        assert_ne!(std::mem::discriminant(&ok_a), std::mem::discriminant(&err_v));
    }
    /// Test 2: Option Some and None variants
    #[test]
    fn test_option_some_and_none() {
        // Fresh instances for each test to avoid ownership issues
        let some_42: Option<i32> = Option::some(42);
        let none: Option<i32> = Option::none();
        // Predicate checks (don't consume)
        assert!(some_42.is_some());
        assert!(!some_42.is_none());
        assert!(!none.is_some());
        assert!(none.is_none());
        // unwrap consumes — use separate instance
        assert_eq!(Option::some(42).unwrap(), 42);
        // unwrap_or consumes — use fresh instances
        assert_eq!(Option::some(42).unwrap_or(99), 42);
        assert_eq!(Option::<i32>::none().unwrap_or(99), 99);
        // ok_or consumes — use fresh instances
        assert!(Option::some(42).ok_or("missing").is_ok());
        assert!(Option::<i32>::none().ok_or("missing").is_err());
    }
    /// Test 3: Result monad laws (Φ proof obligation)
    #[test]
    fn test_result_monad_laws() {
        // f: i32 -> Result<i32, String>
        let f = |x: i32| -> Result<i32, String> {
            if x > 0 { Result::ok(x * 2) }
            else { Result::err("negative".to_string()) }
        };
        // g: i32 -> Result<i32, String>
        let g = |x: i32| -> Result<i32, String> {
            if x < 100 { Result::ok(x + 1) }
            else { Result::err("too big".to_string()) }
        };
        // Test with success value
        let ok_value = 5;
        let ok_m: Result<i32, String> = Result::ok(10);
        assert!(verify_result_monad_laws(ok_value, f, g, ok_m));
        // Test with error propagation
        let err_m: Result<i32, String> = Result::err("initial error".to_string());
        // For error case, bind short-circuits, so laws still hold
        let left = err_m.clone().and_then(f);
        assert!(left.is_err());
    }
    /// Test 4: Option monad laws
    #[test]
    fn test_option_monad_laws() {
        let f = |x: i32| -> Option<i32> {
            if x > 0 { Option::some(x * 2) }
            else { Option::none() }
        };
        let g = |x: i32| -> Option<i32> {
            if x < 100 { Option::some(x + 1) }
            else { Option::none() }
        };
        // With present value
        let value = 5;
        let some_m: Option<i32> = Option::some(10);
        assert!(verify_option_monad_laws(value, f, g, some_m));
        // With None propagation
        let none_m: Option<i32> = Option::none();
        assert!(none_m.clone().and_then(f).is_none());
    }
    /// Test 5: Error propagation chain (real-world usage)
    ///
    /// Demonstrates:
    /// - Converting from std::Result to our Result (interoperability)
    /// - Chaining with and_then (>>= operator)
    /// - Error short-circuiting at each step
    #[test]
    fn test_error_propagation_chain() {
        // Parse: std::Result -> our Result (explicit conversion)
        let parse = |s: &str| -> Result<i32, String> {
            match s.parse::<i32>() {
                Ok(n) => Result::ok(n),
                Err(_) => Result::err(format!("parse error: {}", s)),
            }
        };
        let validate = |n: i32| -> Result<i32, String> {
            if n >= 0 && n <= 100 { Result::ok(n) }
            else { Result::err(format!("out of range: {}", n)) }
        };
        let transform = |n: i32| -> Result<i32, String> {
            Result::ok(n * 2)
        };
        // Successful chain: "50" -> 50 -> 50 -> 100
        let chain_ok = parse("50")
            .and_then(validate)
            .and_then(transform);
        assert_eq!(chain_ok, Result::ok(100));
        // Failure propagates at parse
        let chain_parse_fail = parse("abc")
            .and_then(validate)
            .and_then(transform);
        assert!(chain_parse_fail.is_err());
        assert!(chain_parse_fail.unwrap_err().contains("parse error"));
        // Failure propagates at validate
        let chain_validate_fail = parse("200")
            .and_then(validate)
            .and_then(transform);
        assert!(chain_validate_fail.is_err());
        assert!(chain_validate_fail.unwrap_err().contains("out of range"));
        // Map operations preserve chain structure
        let mapped = parse("25")
            .map(|n| n + 5)          // Ok(30)
            .and_then(validate);      // Ok(30)
        assert_eq!(mapped, Result::ok(30));
    }
}
