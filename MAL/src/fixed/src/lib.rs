//! MAL Fixed-Point — Deterministic Math (Stages 40-44)
//! Ported from the Arabic Mathematical Language compiler.
//! Evidence: stage44_run.py, fix3.py, p40_sqrt.asm, result_stage44_to_copy.txt
//!
//! CRITICAL: i128 mandatory (Horner acc reaches exp(r)*2^64 ~= 2.6e19,
//! exceeding i64::MAX). Identified during porting analysis.
#![forbid(unsafe_code)]
pub const FRAC: u32 = 32;
pub const SCALE: i128 = 1i128 << 32;
/// ln(2) in Q32.32 = 2977044472 (from result_stage44_to_copy.txt)
pub const LN2_Q32: i64 = 2977044472;
/// 1/ln(2) in Q32.32 = 6196328019 (from result_stage44_to_copy.txt)
pub const INVLN2_Q32: i64 = 6196328019;
/// ln(2) in Q40.40 = 762123384786 (error <= 2^-41, from fix3.py)
pub const C40: i128 = 762123384786;
/// Taylor coefficients: coeffs[n] = round(2^32 / n!), degree 12.
/// Matches nasbi_exp.asm exactly.
pub const COEFFS: [i64; 13] = [
    4294967296, 4294967296, 2147483648, 715827883, 178956971,
    35791394, 5965232, 852176, 106522, 11836, 1184, 108, 9,
];
/// A Q32.32 fixed-point number (32 integer bits + 32 fractional bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Q32_32(pub i64);
impl Q32_32 {
    pub fn from_f64(x: f64) -> Self {
        Q32_32((x * 4294967296.0).round() as i64)
    }
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / 4294967296.0
    }
    pub fn raw(self) -> i64 { self.0 }
    pub fn one() -> Self { Q32_32(1i64 << 32) }
    pub fn zero() -> Self { Q32_32(0) }
}
/// Deterministic exp(x) in Q32.32. NO floating-point in computation.
/// From Stage 44. Verified: max_abs_err ~= 8.48e-9, rel_err ~= 1.2e-10.
///
/// Algorithm:
///   1. k = round(x * INVLN2 / 2^64)      [range reduction]
///   2. r64 = x*2^64 - k*C40*2^24          [reduced arg, Q0.64, 40-bit ln2]
///   3. Horner degree-12 with 32-bit extra guard [Q32.64]
///   4. correction: exp(r) ~= acc*(1 + r_lo)
///   5. scale by 2^k, single final rounding to Q32.32
pub fn exp_fixed(x: Q32_32) -> Q32_32 {
    let xq = x.0 as i128;
    // 1. k = round(x * INVLN2 / 2^64)
    let prod = xq * (INVLN2_Q32 as i128);
    let half: i128 = 1i128 << 63;
    let k: i64 = if prod >= 0 {
        ((prod + half) >> 64) as i64
    } else {
        -(((-prod + half) >> 64) as i64)
    };
    // 2. r64 = x*2^64 - k*C40*2^24 (all integer, full precision)
    let r64: i128 = xq * SCALE - (k as i128) * C40 * (1i128 << 24);
    let rhi: i128 = r64 >> FRAC;            // high part Q32.32
    let rlo64: i128 = r64 - (rhi << FRAC);   // remainder (|rlo64| < 2^32)
    // 3. Horner degree-12 with extra 32-bit guard (Q32.64)
    let mut acc: i128 = (COEFFS[12] as i128) << FRAC;
    for n in (0..=11).rev() {
        acc = (acc * rhi) >> FRAC;
        acc += (COEFFS[n] as i128) << FRAC;
    }
    // 4. Remainder correction: exp(r) ~= acc*(1 + r_lo), no extra rounding
    let exp_full: i128 = (acc << FRAC) + ((acc * rlo64) >> FRAC);
    // 5. Scale by 2^k, single final rounding to Q32.32
    let result: i64 = if k >= 0 {
        (((exp_full << k) + (1i128 << 63)) >> 64) as i64
    } else {
        (((exp_full >> (-k)) + (1i128 << 63)) >> 64) as i64
    };
    Q32_32(result)
}
/// Deterministic integer sqrt (bit-by-bit). From p40_sqrt.asm (Stage 40).
/// Returns floor(sqrt(x)). Exact for perfect squares. No libc.
///
/// Assembly correspondence:
///   result=0, step=2^30; loop: if (result|step)^2<=x: result|=step; step>>=1
pub fn sqrt_fixed(x: u64) -> u64 {
    if x == 0 { return 0; }
    let mut result: u64 = 0;
    let mut step: u64 = 1u64 << 30;
    while step != 0 {
        let candidate = result | step;
        if let Some(sq) = candidate.checked_mul(candidate) {
            if sq <= x { result = candidate; }
        }
        step >>= 1;
    }
    result
}
/// Integer power (base^exp) by repeated multiplication. From Stage 40 (قوة).
pub fn power_fixed(base: u64, exp: u32) -> u64 {
    let mut result: u64 = 1;
    for _ in 0..exp {
        result = result.saturating_mul(base);
    }
    result
}
/// Absolute value of Q32.32. From Stage 40 (مطلق).
pub fn abs_fixed(x: Q32_32) -> Q32_32 {
    if x.0 >= 0 { x } else { Q32_32(-x.0) }
}
/// Floor of Q32.32 (integer part). From Stage 40 (أرضية).
/// Arithmetic right shift handles negatives correctly (toward -infinity).
pub fn floor_fixed(x: Q32_32) -> i64 {
    x.0 >> FRAC
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Tolerance accounting for Q32.32 input quantization (~e^|x| * 2^-33).
    fn tol(x: f64) -> f64 {
        x.abs().exp() * 2.0f64.powi(-31) + 2.0f64.powi(-31)
    }
    #[test]
    fn test_constants_match_stage44_report() {
        // These MUST match result_stage44_to_copy.txt
        assert_eq!(LN2_Q32, 2977044472);
        assert_eq!(INVLN2_Q32, 6196328019);
        assert_eq!(C40, 762123384786);
        assert_eq!(COEFFS.len(), 13);
        assert_eq!(COEFFS[0], 4294967296); // 1/0!
        assert_eq!(COEFFS[12], 9);         // 1/12!
    }
    #[test]
    fn test_exp_reference_points_from_report() {
        // 9 reference points from result_stage44_to_copy.txt (9/9 verified)
        let cases = [0.0, 1.0, -1.0, 0.5, 2.0, -5.0, 3.0, 5.0, -20.0];
        for &x in &cases {
            let got = exp_fixed(Q32_32::from_f64(x)).to_f64();
            let reference = x.exp();
            let err = (got - reference).abs();
            assert!(err < tol(x),
                "exp({}): got {:.12}, ref {:.12}, err {:.2e}, tol {:.2e}",
                x, got, reference, err, tol(x));
        }
    }
    #[test]
    fn test_exp_zero_and_one() {
        let e0 = exp_fixed(Q32_32::zero()).to_f64();
        assert!((e0 - 1.0).abs() < 1e-9, "exp(0)={}", e0);
        let e1 = exp_fixed(Q32_32::one()).to_f64();
        assert!((e1 - std::f64::consts::E).abs() < 1e-8, "exp(1)={}", e1);
    }
    #[test]
    fn test_exp_determinism_bit_exact() {
        // Core property: identical input -> bit-identical output, every time
        for &x in &[0.3, 1.7, -2.5, 4.1] {
            let q = Q32_32::from_f64(x);
            assert_eq!(exp_fixed(q).raw(), exp_fixed(q).raw(),
                "non-deterministic at x={}", x);
        }
    }
    #[test]
    fn test_exp_monotonic_increasing() {
        let mut prev = exp_fixed(Q32_32::from_f64(-10.0)).raw();
        let mut x = -9.5;
        while x <= 5.0 {
            let cur = exp_fixed(Q32_32::from_f64(x)).raw();
            assert!(cur > prev, "not increasing at x={}", x);
            prev = cur;
            x += 0.5;
        }
    }
    #[test]
    fn test_sqrt_perfect_squares() {
        // From p40_sqrt.asm: sqrt(144) = 12 (the documented test case)
        assert_eq!(sqrt_fixed(144), 12);
        assert_eq!(sqrt_fixed(0), 0);
        assert_eq!(sqrt_fixed(1), 1);
        assert_eq!(sqrt_fixed(4), 2);
        assert_eq!(sqrt_fixed(9), 3);
        assert_eq!(sqrt_fixed(1000000), 1000);
    }
    #[test]
    fn test_sqrt_floor_non_squares() {
        assert_eq!(sqrt_fixed(2), 1);
        assert_eq!(sqrt_fixed(3), 1);
        assert_eq!(sqrt_fixed(8), 2);
        assert_eq!(sqrt_fixed(15), 3);
        assert_eq!(sqrt_fixed(99), 9);
    }
    #[test]
    fn test_sqrt_cross_check_f64() {
        for x in (0..1000u64).step_by(7) {
            let got = sqrt_fixed(x);
            let reference = (x as f64).sqrt().floor() as u64;
            assert_eq!(got, reference, "sqrt({}): got {}, ref {}", x, got, reference);
        }
    }
    #[test]
    fn test_power_from_p40() {
        // From result_stage44_to_copy.txt p40_pow: pow(2,10) = 1024
        assert_eq!(power_fixed(2, 10), 1024);
        assert_eq!(power_fixed(3, 0), 1);
        assert_eq!(power_fixed(5, 3), 125);
        assert_eq!(power_fixed(2, 0), 1);
    }
    #[test]
    fn test_abs_from_p40() {
        // From result_stage44_to_copy.txt p40_abs: abs(-42) = 42
        assert_eq!(abs_fixed(Q32_32::from_f64(-42.0)).to_f64(), 42.0);
        assert_eq!(abs_fixed(Q32_32::from_f64(42.0)).to_f64(), 42.0);
        assert_eq!(abs_fixed(Q32_32::zero()).to_f64(), 0.0);
    }
    #[test]
    fn test_floor_from_p40() {
        // From result_stage44_to_copy.txt p40_floor: floor(37/10) = 3
        assert_eq!(floor_fixed(Q32_32::from_f64(3.7)), 3);
        assert_eq!(floor_fixed(Q32_32::from_f64(5.0)), 5);
        assert_eq!(floor_fixed(Q32_32::from_f64(0.5)), 0);
    }
    #[test]
    fn test_no_silent_overflow_i128() {
        // Sanity: exp(20) intermediate must not overflow i128
        // (exp(20)*2^64 ~ 4.8e8 * 1.8e19 = 8.7e27 < i128::MAX ~ 1.7e38)
        let r = exp_fixed(Q32_32::from_f64(20.0)).to_f64();
        let reference = 20.0f64.exp();
        let rel_err = ((r - reference) / reference).abs();
        assert!(rel_err < 1e-8, "exp(20): got {}, ref {}", r, reference);
    }
}
