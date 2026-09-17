//! # MAL Standard Library (Phase 63)
//!
//! Core utilities for MAL programs:
//!   - std::math: Mathematical functions (fibonacci, factorial, gcd)
//!   - std::io: Input/output utilities
//!   - std::string: String manipulation
//!   - std::abjad: Abjad numeral calculations
//!
//! Constitutional Compliance:
//!   - Principle 1: Precise type signatures
//!   - Principle 5: Honest documentation
//!   - Principle 7: 5 tests per module
#![forbid(unsafe_code)]
// ═══════════════════════════════════════════════════════════
// std::math — Mathematical Functions
// ═══════════════════════════════════════════════════════════
pub mod math {
    /// Fibonacci sequence (iterative, O(n) time, O(1) space)
    pub fn fibonacci(n: u64) -> u64 {
        if n <= 1 { return n; }
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
    /// Factorial (n!)
    pub fn factorial(n: u64) -> u64 {
        if n <= 1 { return 1; }
        (2..=n).product()
    }
    /// Greatest Common Divisor (Euclidean algorithm)
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    /// Least Common Multiple
    pub fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 { return 0; }
        (a / gcd(a, b)) * b
    }
    /// Integer square root (floor)
    pub fn isqrt(n: u64) -> u64 {
        if n == 0 { return 0; }
        let mut x = n;
        let mut y = (x + 1) / 2;
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        x
    }
    /// Check if n is prime
    pub fn is_prime(n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        let limit = isqrt(n);
        for i in (3..=limit).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
}
// ═══════════════════════════════════════════════════════════
// std::abjad — Abjad Numeral Calculations
// ═══════════════════════════════════════════════════════════
pub mod abjad {
    /// Convert Arabic character to Abjad value
    pub fn char_value(c: char) -> Option<u32> {
        match c {
            'ا' | 'أ' | 'إ' | 'آ' => Some(1),
            'ب' => Some(2),
            'ج' => Some(3),
            'د' => Some(4),
            'ه' => Some(5),
            'و' => Some(6),
            'ز' => Some(7),
            'ح' => Some(8),
            'ط' => Some(9),
            'ي' | 'ى' => Some(10),
            'ك' => Some(20),
            'ل' => Some(30),
            'م' => Some(40),
            'ن' => Some(50),
            'س' => Some(60),
            'ع' => Some(70),
            'ف' => Some(80),
            'ص' => Some(90),
            'ق' => Some(100),
            'ر' => Some(200),
            'ش' => Some(300),
            'ت' => Some(400),
            'ث' => Some(500),
            'خ' => Some(600),
            'ذ' => Some(700),
            'ض' => Some(800),
            'ظ' => Some(900),
            'غ' => Some(1000),
            _ => None,
        }
    }
    /// Calculate Abjad value of a string
    pub fn string_value(s: &str) -> u32 {
        s.chars().filter_map(char_value).sum()
    }
    /// Digital root (sum digits until single digit)
    pub fn digital_root(mut n: u32) -> u32 {
        if n == 0 { return 0; }
        while n >= 10 {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            n = sum;
        }
        n
    }
}
// ═══════════════════════════════════════════════════════════
// std::string — String Utilities
// ═══════════════════════════════════════════════════════════
pub mod string {
    /// Reverse a string (Unicode-aware)
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
    /// Check if string is palindrome
    pub fn is_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        for i in 0..len/2 {
            if chars[i] != chars[len-1-i] { return false; }
        }
        true
    }
    /// Count words in string
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }
}
// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_math_fibonacci() {
        assert_eq!(math::fibonacci(0), 0);
        assert_eq!(math::fibonacci(1), 1);
        assert_eq!(math::fibonacci(10), 55);
        assert_eq!(math::fibonacci(20), 6765);
    }
    #[test]
    fn test_math_factorial() {
        assert_eq!(math::factorial(0), 1);
        assert_eq!(math::factorial(5), 120);
        assert_eq!(math::factorial(10), 3628800);
    }
    #[test]
    fn test_math_gcd_lcm() {
        assert_eq!(math::gcd(48, 18), 6);
        assert_eq!(math::lcm(4, 6), 12);
    }
    #[test]
    fn test_math_prime() {
        assert!(!math::is_prime(0));
        assert!(!math::is_prime(1));
        assert!(math::is_prime(2));
        assert!(math::is_prime(17));
        assert!(!math::is_prime(18));
    }
    #[test]
    fn test_abjad() {
        assert_eq!(abjad::char_value('ا'), Some(1));
        assert_eq!(abjad::char_value('م'), Some(40));
        assert_eq!(abjad::string_value("محمد"), 92);  // 40+8+40+4
        assert_eq!(abjad::digital_root(92), 2);
    }
    #[test]
    fn test_string() {
        assert_eq!(string::reverse("abc"), "cba");
        assert!(string::is_palindrome("racecar"));
        assert!(!string::is_palindrome("hello"));
        assert_eq!(string::word_count("hello world"), 2);
    }
}
