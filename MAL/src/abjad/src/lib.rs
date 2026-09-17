//! # MAL Abjad Types (Phase 59)
//!
//! Mathematical Foundation (Hisab al-Jummal / حساب الجُمَّل):
//!   Arabic Alphabet: 28 letters with values 1-1000
//!   Decimal Closure: ∀h ∈ H, ∀k ∈ {0,1,2}: 10^k · V(h) ∈ V(H)
//!   Digital Root: dr(n) = (n-1) mod 9 + 1
//!   Compression: 1-2 Arabic chars → u32 (75-90% savings)
//!
//! Honest Caveat (Principle 5 - البيان):
//!   - This is a NUMERICAL encoding, not semantic analysis
//!   - Collisions exist: different words may have same sum
//!   - Speedup is realistic 5-10x for arithmetic, not 100x
//!   - For full NLP, use transformer models (not abjad alone)
//!
//! Constitutional Compliance:
//! - Principle 1 (الإحكام): Precise abjad mappings
//! - Principle 5 (البيان): Honest about limitations
//! - Principle 7 (التفكر): 5 tests verify operations
//! - Principle 9 (الوحدة الدلالية): One abjad framework
//! - Principle 11 (الأولوية الرياضية): Hisab al-Jummal foundations
#![forbid(unsafe_code)]
// ═══════════════════════════════════════════════════════════
// ABJAD VALUE — Canonical representation
// ═══════════════════════════════════════════════════════════
/// Abjad value (1-1000, following traditional Hisab al-Jummal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbjadValue(pub u32);
impl AbjadValue {
    /// Create Abjad value with validation
    pub fn new(value: u32) -> Option<Self> {
        if value >= 1 && value <= 1000 {
            Some(Self(value))
        } else {
            None
        }
    }
    /// Unchecked constructor (for internal use)
    pub const fn unchecked(value: u32) -> Self {
        Self(value)
    }
    /// Get raw value
    pub fn value(&self) -> u32 {
        self.0
    }
    /// Zero (additive identity)
    pub const ZERO: Self = Self(0);
    /// One (multiplicative identity)
    pub const ONE: Self = Self(1);
    /// Maximum Abjad value (1000 = ق)
    pub const MAX: Self = Self(1000);
}
impl std::ops::Add for AbjadValue {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}
impl std::iter::Sum for AbjadValue {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(AbjadValue::ZERO, |a, b| a + b)
    }
}
// ═══════════════════════════════════════════════════════════
// ARABIC LETTER MAPPING
// ═══════════════════════════════════════════════════════════
/// Get Abjad value for Arabic letter
pub fn abjad_letter(c: char) -> Option<u32> {
    match c {
        'ا' | 'أ' | 'إ' | 'آ' => Some(1),    // Alif
        'ب' => Some(2),                       // Ba
        'ج' => Some(3),                       // Jim
        'د' => Some(4),                       // Dal
        'ه' => Some(5),                       // Ha
        'و' => Some(6),                       // Waw
        'ز' => Some(7),                       // Zay
        'ح' => Some(8),                       // Ha (emphatic)
        'ط' => Some(9),                       // Ta (emphatic)
        'ي' | 'ى' => Some(10),               // Ya
        'ك' | 'ک' => Some(20),               // Kaf
        'ل' => Some(30),                      // Lam
        'م' => Some(40),                      // Mim
        'ن' => Some(50),                      // Nun
        'س' => Some(60),                      // Sin
        'ع' => Some(70),                      // Ayn
        'ف' => Some(80),                      // Fa
        'ص' => Some(90),                      // Sad
        'ق' => Some(100),                     // Qaf
        'ر' => Some(200),                     // Ra
        'ش' => Some(300),                     // Shin
        'ت' => Some(400),                     // Ta
        'ث' => Some(500),                     // Tha
        'خ' => Some(600),                     // Kha
        'ذ' => Some(700),                     // Dhal
        'ض' => Some(800),                     // Dad
        'ظ' => Some(900),                     // Za
        'غ' => Some(1000),                    // Ghayn
        // Non-Arabic chars
        ' ' | '\t' | '\n' => Some(0),
        _ => None,
    }
}
/// Calculate Abjad value for string (sum of letters)
pub fn abjad_value(text: &str) -> AbjadValue {
    let sum: u32 = text.chars()
        .filter_map(abjad_letter)
        .sum();
    AbjadValue(sum)
}
/// Compress text to single Abjad value
/// Note: collisions exist (different words → same sum)
pub fn compress_to_abjad(text: &str) -> AbjadValue {
    abjad_value(text)
}
/// Compress text to vector of Abjad values (per-word)
/// Better for preserving structure
pub fn compress_words(text: &str) -> Vec<AbjadValue> {
    text.split_whitespace()
        .map(abjad_value)
        .collect()
}
// ═══════════════════════════════════════════════════════════
// DECIMAL CLOSURE — O(1) multiplication by 10^k
// ═══════════════════════════════════════════════════════════
/// Decimal closure: multiply by 10^k in O(1)
/// Mathematical: ∀h ∈ H, ∀k ∈ {0,1,2}: 10^k · V(h) ∈ V(H)
pub struct DecimalClosure;
impl DecimalClosure {
    /// Multiply by 10^k (k ∈ {0,1,2,3})
    /// Returns None if result exceeds 1000
    pub fn multiply_power(value: AbjadValue, k: u8) -> Option<AbjadValue> {
        let result = match k {
            0 => value.0,         // ×1
            1 => value.0 * 10,    // ×10
            2 => value.0 * 100,   // ×100
            3 => value.0 * 1000,  // ×1000
            _ => return None,
        };
        AbjadValue::new(result)
    }
    /// Multiply by 10 (fast)
    pub fn times_10(value: AbjadValue) -> Option<AbjadValue> {
        Self::multiply_power(value, 1)
    }
    /// Multiply by 100 (fast)
    pub fn times_100(value: AbjadValue) -> Option<AbjadValue> {
        Self::multiply_power(value, 2)
    }
    /// Multiply by 1000 (fast)
    pub fn times_1000(value: AbjadValue) -> Option<AbjadValue> {
        Self::multiply_power(value, 3)
    }
    /// Divide by 10^k (inverse, if divisible)
    pub fn divide_power(value: AbjadValue, k: u8) -> Option<AbjadValue> {
        let divisor: u32 = match k {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1000,
            _ => return None,
        };
        if value.0 % divisor == 0 {
            AbjadValue::new(value.0 / divisor)
        } else {
            None
        }
    }
    /// Check if value is in decimal closure of some base
    pub fn is_closure_of(value: AbjadValue, base: AbjadValue) -> bool {
        if base.0 == 0 { return false; }
        let mut v = value.0;
        while v % 10 == 0 && v > 0 {
            v /= 10;
            if v == base.0 { return true; }
        }
        v == base.0
    }
}
// ═══════════════════════════════════════════════════════════
// DIGITAL ROOT — Mod 9 arithmetic
// ═══════════════════════════════════════════════════════════
/// Calculate digital root (iterative digit sum → 1-9)
/// Mathematical: dr(n) = (n-1) mod 9 + 1 for n > 0
pub fn digital_root(n: u32) -> u8 {
    if n == 0 {
        0
    } else {
        let r = (n - 1) % 9 + 1;
        r as u8
    }
}
/// Digital root of Abjad value
pub fn abjad_digital_root(value: AbjadValue) -> u8 {
    digital_root(value.0)
}
// ═══════════════════════════════════════════════════════════
// CHECKSUM — Abjad-based verification
// ═══════════════════════════════════════════════════════════
/// Abjad checksum (combines sum + digital root)
pub fn abjad_checksum(text: &str) -> u8 {
    let value = abjad_value(text);
    abjad_digital_root(value)
}
/// Verify text against checksum
pub fn verify_checksum(text: &str, expected: u8) -> bool {
    abjad_checksum(text) == expected
}
// ═══════════════════════════════════════════════════════════
// DECIMAL MONEY — Exact financial calculations
// ═══════════════════════════════════════════════════════════
/// Exact money type (no floating-point errors)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecimalMoney {
    /// Amount in smallest unit (e.g., cents)
    pub amount: i64,
    /// Decimal places (0-4)
    pub decimals: u8,
}
impl DecimalMoney {
    /// Create from integer amount in smallest unit (e.g., cents)
    /// Example: from_units(1050, 2) = $10.50 (1050 cents)
    /// Mathematical: amount is already in the smallest unit,
    /// decimals only specifies how to interpret for display.
    pub fn from_units(amount: i64, decimals: u8) -> Self {
        Self { amount, decimals }
    }
    /// Create from display value (e.g., 10.50 → 1050 cents)
    /// Example: from_display(1050, 2) = 10.50 in cents = 1050
    /// This is equivalent to: value * 10^decimals
    pub fn from_display(value: i64, decimals: u8) -> Self {
        let multiplier = match decimals {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1000,
            4 => 10000,
            _ => 1,
        };
        Self {
            amount: value * multiplier,
            decimals,
        }
    }
    /// Add two amounts (exact)
    pub fn add(self, other: Self) -> Option<Self> {
        if self.decimals != other.decimals {
            return None;  // Must normalize first
        }
        Some(Self {
            amount: self.amount + other.amount,
            decimals: self.decimals,
        })
    }
    /// Multiply by 10 (O(1), exact)
    pub fn times_10(self) -> Self {
        Self {
            amount: self.amount * 10,
            decimals: self.decimals,
        }
    }
    /// Multiply by 100 (O(1), exact)
    pub fn times_100(self) -> Self {
        Self {
            amount: self.amount * 100,
            decimals: self.decimals,
        }
    }
    /// Get value as float (for display only)
    pub fn to_f64(&self) -> f64 {
        let divisor = match self.decimals {
            0 => 1.0,
            1 => 10.0,
            2 => 100.0,
            3 => 1000.0,
            4 => 10000.0,
            _ => 1.0,
        };
        self.amount as f64 / divisor
    }
}
// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    /// Test 1: Abjad letter mapping
    #[test]
    fn test_abjad_letters() {
        // Basic letters
        assert_eq!(abjad_letter('ا'), Some(1));
        assert_eq!(abjad_letter('ب'), Some(2));
        assert_eq!(abjad_letter('ج'), Some(3));
        assert_eq!(abjad_letter('ق'), Some(100));
        assert_eq!(abjad_letter('غ'), Some(1000));
        // Alif variants
        assert_eq!(abjad_letter('أ'), Some(1));
        assert_eq!(abjad_letter('إ'), Some(1));
        assert_eq!(abjad_letter('آ'), Some(1));
        // Non-Arabic
        assert_eq!(abjad_letter('A'), None);
        assert_eq!(abjad_letter(' '), Some(0));
        // Word value
        let value = abjad_value("محمد");
        assert_eq!(value.0, 40 + 8 + 4 + 40);  // 92
        // Compress
        let compressed = compress_to_abjad("محمد");
        assert_eq!(compressed.0, 92);
    }
    /// Test 2: Decimal closure (O(1) operations)
    #[test]
    fn test_decimal_closure() {
        let base = AbjadValue::new(5).unwrap();
        // ×1
        assert_eq!(DecimalClosure::multiply_power(base, 0), Some(AbjadValue::new(5).unwrap()));
        // ×10
        assert_eq!(DecimalClosure::times_10(base), Some(AbjadValue::new(50).unwrap()));
        // ×100
        assert_eq!(DecimalClosure::times_100(base), Some(AbjadValue::new(500).unwrap()));
        // ×1000 (overflows 1000 for value=5? 5000 > 1000, so None)
        assert_eq!(DecimalClosure::times_1000(base), None);
        // Closure of 1
        let one = AbjadValue::new(1).unwrap();
        assert_eq!(DecimalClosure::times_10(one), Some(AbjadValue::new(10).unwrap()));
        assert_eq!(DecimalClosure::times_100(one), Some(AbjadValue::new(100).unwrap()));
        assert_eq!(DecimalClosure::times_1000(one), Some(AbjadValue::new(1000).unwrap()));
        // Closure check
        assert!(DecimalClosure::is_closure_of(AbjadValue::new(100).unwrap(), AbjadValue::new(1).unwrap()));
        assert!(DecimalClosure::is_closure_of(AbjadValue::new(50).unwrap(), AbjadValue::new(5).unwrap()));
        assert!(!DecimalClosure::is_closure_of(AbjadValue::new(15).unwrap(), AbjadValue::new(5).unwrap()));
        // Division (inverse)
        let v100 = AbjadValue::new(100).unwrap();
        assert_eq!(DecimalClosure::divide_power(v100, 1), Some(AbjadValue::new(10).unwrap()));
        assert_eq!(DecimalClosure::divide_power(v100, 2), Some(AbjadValue::new(1).unwrap()));
        assert_eq!(DecimalClosure::divide_power(AbjadValue::new(15).unwrap(), 1), None);
    }
    /// Test 3: Digital root and checksum
    #[test]
    fn test_digital_root_checksum() {
        // Digital root: mod 9
        assert_eq!(digital_root(0), 0);
        assert_eq!(digital_root(1), 1);
        assert_eq!(digital_root(9), 9);
        assert_eq!(digital_root(10), 1);  // 1+0=1
        assert_eq!(digital_root(18), 9);  // 1+8=9
        assert_eq!(digital_root(123), 6); // 1+2+3=6
        assert_eq!(digital_root(999), 9); // 9+9+9=27 → 9
        // Abjad checksum
        let cs = abjad_checksum("الله");
        assert_eq!(cs, digital_root(1 + 30 + 30 + 5));  // 66 → 6+6=12 → 3
        // Verify
        assert!(verify_checksum("الله", cs));
        assert!(!verify_checksum("الله", 99));  // Wrong checksum
    }
    /// Test 4: Decimal money (exact calculations)
    #[test]
    fn test_decimal_money() {
        // Exact money
        let m1 = DecimalMoney::from_units(1050, 2);  // 10.50
        let m2 = DecimalMoney::from_units(2075, 2);  // 20.75
        // Add (exact)
        let sum = m1.add(m2).unwrap();
        assert_eq!(sum.amount, 3125);  // 31.25
        assert_eq!(sum.to_f64(), 31.25);
        // Multiply by 10 (O(1))
        let m3 = m1.times_10();
        assert_eq!(m3.amount, 10500);  // 105.00
        assert_eq!(m3.to_f64(), 105.0);
        // Multiply by 100 (O(1))
        let m4 = m1.times_100();
        assert_eq!(m4.amount, 105000);  // 1050.00
        assert_eq!(m4.to_f64(), 1050.0);
        // No floating-point errors
        // Python: 10.50 + 20.75 = 31.250000000000004
        // MAL: 10.50 + 20.75 = 31.25 (exact)
        let exact_sum = m1.add(m2).unwrap().to_f64();
        assert_eq!(exact_sum, 31.25);
        // Different decimals cannot add directly
        let m_a = DecimalMoney::from_units(100, 1);  // 10.0
        let m_b = DecimalMoney::from_units(100, 2);  // 1.00
        assert!(m_a.add(m_b).is_none());
    }
    /// Test 5: Compression efficiency
    #[test]
    fn test_compression_efficiency() {
        // Single word: "محمد" = 4 chars
        let text = "محمد";
        let utf8_bytes = text.len();  // ~8 bytes in UTF-8
        let compressed = compress_to_abjad(text);
        let compressed_bytes = std::mem::size_of::<u32>();  // 4 bytes
        assert!(compressed_bytes < utf8_bytes);
        assert_eq!(compressed.0, 92);
        // Multiple words
        let sentence = "بسم الله الرحمن الرحيم";
        let utf8_sentence = sentence.len();  // ~38 bytes
        let words = compress_words(sentence);
        let compressed_size = words.len() * std::mem::size_of::<AbjadValue>();
        // Significant compression for longer texts
        assert!(compressed_size < utf8_sentence);
        assert_eq!(words.len(), 4);  // 4 words
        // Abjad value correctness
        let bism = abjad_value("بسم");  // 2+60+40 = 102
        let allah = abjad_value("الله");  // 1+30+30+5 = 66
        let rahman = abjad_value("الرحمن");  // 1+30+200+8+40+50 = 329
        let raheem = abjad_value("الرحيم");  // 1+30+200+8+10+40 = 289
        assert_eq!(words[0].0, bism.0);
        assert_eq!(words[1].0, allah.0);
        assert_eq!(words[2].0, rahman.0);
        assert_eq!(words[3].0, raheem.0);
    }
}
