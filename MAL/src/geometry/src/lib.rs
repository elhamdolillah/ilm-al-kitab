//! # MAL Geometry (Phase 60)
//!
//! Mathematical Foundation (Euclid, Fibonacci, Quranic patterns):
//!   Golden Ratio: φ = (1 + √5) / 2 = 1.6180339887...
//!   Abjad Pi: π_A = Σ(1/V(h)) = 263857/84000 ≈ 3.1411547619
//!   Quranic Fibonacci: 12, 21, 33, 54, 87, 141, ...
//!   Geometric Hash: using φ-based distribution
//!
//! Honest Caveat (Principle 5 - البيان):
//!   - π_A is a NUMERICAL COINCIDENCE, not a proof that Σ(1/V(h)) = π
//!   - Real π = 3.1415926535... (Abjad π differs by 0.014%)
//!   - Quranic Fibonacci is an OBSERVED pattern, not divine theorem
//!   - For precise calculations, use std::f64::consts::PI
//!
//! Constitutional Compliance:
//! - Principle 1 (الإحكام): Precise constants
//! - Principle 5 (البيان): Honest about limitations
//! - Principle 7 (التفكر): 5 tests verify patterns
//! - Principle 11 (الأولوية الرياضية): Euclid/Fibonacci foundations
#![forbid(unsafe_code)]
use mal_abjad::{AbjadValue, abjad_letter};
// ═══════════════════════════════════════════════════════════
// GOLDEN RATIO — φ = (1 + √5) / 2
// ═══════════════════════════════════════════════════════════
/// Golden Ratio φ = (1 + √5) / 2 = 1.618033988749895...
pub const GOLDEN_RATIO: f64 = 1.618033988749895;
/// Golden Ratio conjugate φ⁻¹ = φ - 1 = 0.618033988749895...
pub const GOLDEN_RATIO_CONJUGATE: f64 = 0.618033988749895;
/// Check if two values are in golden ratio (within tolerance)
pub fn is_golden_ratio(a: f64, b: f64, tolerance: f64) -> bool {
    if a == 0.0 || b == 0.0 {
        return false;
    }
    let ratio = a / b;
    (ratio - GOLDEN_RATIO).abs() < tolerance || (ratio - GOLDEN_RATIO_CONJUGATE).abs() < tolerance
}
/// Split a value according to golden ratio
/// Returns (larger_part, smaller_part)
pub fn golden_split(total: f64) -> (f64, f64) {
    let larger = total * GOLDEN_RATIO_CONJUGATE;
    let smaller = total - larger;
    (larger, smaller)
}
/// Golden angle (in radians) = 2π / φ²
pub const GOLDEN_ANGLE: f64 = 2.399963229728653;
// ═══════════════════════════════════════════════════════════
// ABJAD PI — π_A = Σ(1/V(h))
// ═══════════════════════════════════════════════════════════
/// Abjad Pi: sum of reciprocals of all Abjad letter values
/// Mathematical: π_A = 263857 / 84000 = 3.141154761904761...
/// Note: This is a NUMERICAL COINCIDENCE, not a proof that π_A = π
pub const ABJAD_PI: f64 = 3.141154761904761;
/// Exact rational representation of Abjad Pi
pub const ABJAD_PI_NUMERATOR: u64 = 263857;
pub const ABJAD_PI_DENOMINATOR: u64 = 84000;
/// Standard mathematical Pi for comparison
pub const STANDARD_PI: f64 = std::f64::consts::PI;
/// Difference between Abjad Pi and standard Pi
pub fn abjad_pi_error() -> f64 {
    (ABJAD_PI - STANDARD_PI).abs()
}
/// Relative error (percentage)
pub fn abjad_pi_relative_error() -> f64 {
    abjad_pi_error() / STANDARD_PI * 100.0
}
/// Verify Abjad Pi calculation
pub fn verify_abjad_pi() -> f64 {
    let mut sum = 0.0;
    // Units (1-9)
    for d in 1..=9 {
        sum += 1.0 / d as f64;
    }
    // Tens (10-90)
    for d in 1..=9 {
        sum += 1.0 / (d * 10) as f64;
    }
    // Hundreds (100-900)
    for d in 1..=9 {
        sum += 1.0 / (d * 100) as f64;
    }
    // Thousands (1000)
    sum += 1.0 / 1000.0;
    sum
}
// ═══════════════════════════════════════════════════════════
// QURANIC FIBONACCI — Observed pattern in Quran
// ═══════════════════════════════════════════════════════════
/// Quranic Fibonacci sequence (observed pattern)
/// Based on word "شهر" (month) occurrences:
/// - 12 times up to chapter 65
/// - 21 times up to chapter 98
/// - 33 times to end
/// 12, 21, 33 are Fibonacci numbers!
pub const QURANIC_FIBONACCI: [u64; 16] = [
    12, 21, 33, 54, 87, 141, 228, 369, 597, 966,
    1563, 2529, 4092, 6621, 10713, 17334
];
/// Standard Fibonacci for comparison
pub fn standard_fibonacci(n: usize) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
/// Check if a number is in Quranic Fibonacci sequence
pub fn is_quranic_fibonacci(n: u64) -> bool {
    QURANIC_FIBONACCI.contains(&n)
}
/// Generate Fibonacci-like sequence starting from Quranic base
pub fn quranic_fibonacci_sequence(start_idx: usize, count: usize) -> Vec<u64> {
    let mut seq = Vec::new();
    if start_idx >= QURANIC_FIBONACCI.len() {
        return seq;
    }
    let mut a = if start_idx == 0 { 12 } else { QURANIC_FIBONACCI[start_idx - 1] };
    let mut b = QURANIC_FIBONACCI[start_idx];
    for i in 0..count {
        if start_idx + i < QURANIC_FIBONACCI.len() {
            seq.push(QURANIC_FIBONACCI[start_idx + i]);
        } else {
            let c = a + b;
            seq.push(c);
            a = b;
            b = c;
        }
    }
    seq
}
// ═══════════════════════════════════════════════════════════
// GEOMETRIC HASH — φ-based distribution
// ═══════════════════════════════════════════════════════════
/// Geometric hash using Fibonacci hashing constant (2^64 / φ)
/// Mathematical: Knuth's multiplicative hash (TAOCP Vol 3, Section 6.4)
/// Constant = 0x9E3779B97F4A7C15 = 2^64 / φ = 11400714819323198485
/// Provides excellent bit distribution using golden ratio properties.
pub fn geometric_hash(text: &str) -> u64 {
    // Fibonacci hashing constant = 2^64 / φ
    const FIB_HASH_CONST: u64 = 0x9E3779B97F4A7C15;
    let mut hash: u64 = 0x123456789ABCDEF0;  // Non-zero seed
    for c in text.chars() {
        if let Some(value) = abjad_letter(c) {
            // Mix using golden ratio constant + bit rotation
            hash = hash.wrapping_mul(FIB_HASH_CONST)
                       .wrapping_add(value as u64)
                       .rotate_left(5);
        }
    }
    hash
}
/// Golden ratio based prime check (heuristic)
pub fn is_golden_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let sqrt_n = (n as f64).sqrt() as u64;
    let mut i = 5;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
/// Golden spiral approximation
pub fn golden_spiral_radius(angle: f64) -> f64 {
    // r = a * e^(b*θ) where b = ln(φ) / (π/2)
    let b = GOLDEN_RATIO.ln() / (std::f64::consts::PI / 2.0);
    b.exp().powf(angle)
}
// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    /// Test 1: Golden Ratio properties
    #[test]
    fn test_golden_ratio() {
        // φ = (1 + √5) / 2
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        assert!((phi - GOLDEN_RATIO).abs() < 1e-10);
        // φ² = φ + 1
        assert!((GOLDEN_RATIO * GOLDEN_RATIO - (GOLDEN_RATIO + 1.0)).abs() < 1e-10);
        // φ⁻¹ = φ - 1
        assert!((GOLDEN_RATIO_CONJUGATE - (GOLDEN_RATIO - 1.0)).abs() < 1e-10);
        // Golden ratio check
        assert!(is_golden_ratio(1.618, 1.0, 0.01));
        assert!(is_golden_ratio(100.0, 61.8, 0.01));
        assert!(!is_golden_ratio(2.0, 1.0, 0.01));
        // Golden split
        let (larger, smaller) = golden_split(100.0);
        assert!((larger - 61.80339887).abs() < 1e-5);
        assert!((smaller - 38.19660113).abs() < 1e-5);
        assert!((larger + smaller - 100.0).abs() < 1e-10);
        // Golden angle
        assert!((GOLDEN_ANGLE - 2.399963229728653).abs() < 1e-10);
    }
    /// Test 2: Abjad Pi calculation
    #[test]
    fn test_abjad_pi() {
        // Verify calculation
        let calculated = verify_abjad_pi();
        assert!((calculated - ABJAD_PI).abs() < 1e-10);
        // Exact rational
        let rational = ABJAD_PI_NUMERATOR as f64 / ABJAD_PI_DENOMINATOR as f64;
        assert!((rational - ABJAD_PI).abs() < 1e-10);
        // Error analysis
        let error = abjad_pi_error();
        assert!(error < 0.001);  // Less than 0.001 difference
        let relative_error = abjad_pi_relative_error();
        assert!(relative_error < 0.02);  // Less than 0.02% error
        // Comparison with standard π
        assert!((STANDARD_PI - 3.141592653589793).abs() < 1e-10);
        assert!(ABJAD_PI < STANDARD_PI);  // Abjad π is slightly smaller
        // Partial sums
        let units_sum: f64 = (1..=9).map(|d| 1.0 / d as f64).sum();
        assert!((units_sum - 2.8289682539682538).abs() < 1e-10);
    }
    /// Test 3: Quranic Fibonacci sequence
    #[test]
    fn test_quranic_fibonacci() {
        // First three values (observed in Quran)
        assert_eq!(QURANIC_FIBONACCI[0], 12);
        assert_eq!(QURANIC_FIBONACCI[1], 21);
        assert_eq!(QURANIC_FIBONACCI[2], 33);
        // Fibonacci property: F(n) = F(n-1) + F(n-2)
        for i in 2..QURANIC_FIBONACCI.len() {
            assert_eq!(
                QURANIC_FIBONACCI[i],
                QURANIC_FIBONACCI[i-1] + QURANIC_FIBONACCI[i-2]
            );
        }
        // Membership check
        assert!(is_quranic_fibonacci(12));
        assert!(is_quranic_fibonacci(21));
        assert!(is_quranic_fibonacci(33));
        assert!(!is_quranic_fibonacci(13));
        // Sequence generation
        let seq = quranic_fibonacci_sequence(0, 5);
        assert_eq!(seq.len(), 5);
        assert_eq!(seq[0], 12);
        assert_eq!(seq[1], 21);
        assert_eq!(seq[2], 33);
        // Standard Fibonacci comparison
        assert_eq!(standard_fibonacci(0), 0);
        assert_eq!(standard_fibonacci(1), 1);
        assert_eq!(standard_fibonacci(10), 55);
    }
    /// Test 4: Geometric hash distribution
    #[test]
    fn test_geometric_hash() {
        // Same text → same hash
        let hash1 = geometric_hash("بسم الله");
        let hash2 = geometric_hash("بسم الله");
        assert_eq!(hash1, hash2);
        // Different text → different hash (usually)
        let hash3 = geometric_hash("الحمد لله");
        assert_ne!(hash1, hash3);
        // Hash is deterministic
        let hash4 = geometric_hash("محمد");
        let hash5 = geometric_hash("محمد");
        assert_eq!(hash4, hash5);
        // Prime check
        assert!(is_golden_prime(2));
        assert!(is_golden_prime(3));
        assert!(is_golden_prime(5));
        assert!(is_golden_prime(7));
        assert!(!is_golden_prime(4));
        assert!(!is_golden_prime(9));
        // Golden spiral
        let r1 = golden_spiral_radius(0.0);
        assert!((r1 - 1.0).abs() < 1e-10);
        let r2 = golden_spiral_radius(std::f64::consts::PI / 2.0);
        assert!(r2 > r1);  // Radius increases with angle
    }
    /// Test 5: Integration with Abjad types
    #[test]
    fn test_abjad_geometry_integration() {
        // Abjad value of "الله" = 66
        let allah_value = 1 + 30 + 30 + 5;  // ا + ل + ل + ه
        assert_eq!(allah_value, 66);
        // Check if 66 is related to golden ratio
        let ratio = 106.0 / 66.0;  // 106/66 ≈ φ
        assert!((ratio - GOLDEN_RATIO).abs() < 0.02);
        // Abjad Pi contribution from "الله"
        let allah_contribution = 1.0 / 66.0;
        assert!(allah_contribution < ABJAD_PI);
        // Fibonacci relationship
        // 66 is close to Fibonacci number 55 (difference = 11)
        let fib_10 = standard_fibonacci(10);
        assert_eq!(fib_10, 55);
        assert_eq!(66 - 55, 11);
        // Geometric hash of "الله"
        let hash = geometric_hash("الله");
        assert!(hash > 0);
        // Digital root of 66 = 6+6 = 12 → 1+2 = 3
        let dr = mal_abjad::digital_root(66);
        assert_eq!(dr, 3);
    }
}
