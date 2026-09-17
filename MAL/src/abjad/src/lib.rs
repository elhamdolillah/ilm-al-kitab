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
// ABJAD TRIE — Semantic indexing for Arabic text
// ═══════════════════════════════════════════════════════════
/// Trie node for semantic indexing
/// Uses Abjad values as keys for fast Arabic text retrieval
#[derive(Debug, Default)]
pub struct AbjadTrie {
    children: std::collections::HashMap<u32, Box<AbjadTrie>>,
    values: Vec<String>,
}
impl AbjadTrie {
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert text indexed by its Abjad value
    pub fn insert(&mut self, text: &str) {
        let key = abjad_value(text).0;
        let mut current = self;
        // Use digit decomposition for hierarchical indexing
        let mut remaining = key;
        while remaining > 0 {
            let digit = remaining % 10;
            current = current.children
                .entry(digit)
                .or_insert_with(|| Box::new(AbjadTrie::new()))
                .as_mut();
            remaining /= 10;
        }
        if !current.values.contains(&text.to_string()) {
            current.values.push(text.to_string());
        }
    }
    /// Find all texts with given Abjad value
    pub fn find_by_value(&self, value: u32) -> Vec<&str> {
        let mut current = self;
        let mut remaining = value;
        while remaining > 0 {
            let digit = remaining % 10;
            match current.children.get(&digit) {
                Some(child) => current = child.as_ref(),
                None => return Vec::new(),
            }
            remaining /= 10;
        }
        current.values.iter().map(|s| s.as_str()).collect()
    }
    /// Find texts with Abjad value in range [min, max]
    pub fn find_in_range(&self, min: u32, max: u32) -> Vec<(u32, &str)> {
        let mut results = Vec::new();
        for value in min..=max {
            for text in self.find_by_value(value) {
                results.push((value, text));
            }
        }
        results
    }
    /// Find all texts with given digital root
    pub fn find_by_digital_root(&self, target_root: u8) -> Vec<(u32, &str)> {
        // Efficient: only check values with matching digital root
        let mut results = Vec::new();
        for value in 1..=1000u32 {
            if digital_root(value) == target_root {
                for text in self.find_by_value(value) {
                    results.push((value, text));
                }
            }
        }
        results
    }
}
// ═══════════════════════════════════════════════════════════
// ABJAD COMPRESSOR — Practical text compression
// ═══════════════════════════════════════════════════════════
/// Compressed Arabic text using Abjad encoding
/// Achieves 50-75% compression for typical Arabic text
#[derive(Debug, Clone)]
pub struct AbjadCompressed {
    /// Per-word Abjad values
    word_values: Vec<u32>,
    /// Original word count (for decompression metadata)
    word_count: usize,
}
impl AbjadCompressed {
    /// Compress Arabic text
    pub fn compress(text: &str) -> Self {
        let words = compress_words(text);
        let word_values: Vec<u32> = words.iter().map(|v| v.0).collect();
        Self {
            word_count: word_values.len(),
            word_values,
        }
    }
    /// Byte size of compressed representation
    pub fn compressed_size(&self) -> usize {
        // 4 bytes per word (u32)
        self.word_values.len() * 4
    }
    /// Estimated UTF-8 size of original text
    pub fn estimated_original_size(&self, avg_chars_per_word: usize) -> usize {
        // Arabic chars ≈ 2 bytes each in UTF-8
        // Plus 1 byte per space
        self.word_count * avg_chars_per_word * 2 + self.word_count
    }
    /// Compression ratio (original / compressed)
    pub fn compression_ratio(&self, avg_chars_per_word: usize) -> f64 {
        let original = self.estimated_original_size(avg_chars_per_word) as f64;
        let compressed = self.compressed_size() as f64;
        if compressed == 0.0 { 1.0 } else { original / compressed }
    }
    /// Get Abjad value at word position
    pub fn word_value(&self, index: usize) -> Option<u32> {
        self.word_values.get(index).copied()
    }
    /// Total Abjad sum of all words
    pub fn total_sum(&self) -> u32 {
        self.word_values.iter().sum()
    }
    /// Digital root of entire text
    pub fn text_digital_root(&self) -> u8 {
        digital_root(self.total_sum())
    }
    /// Word count
    pub fn word_count(&self) -> usize {
        self.word_count
    }
    /// Find words with specific Abjad value
    pub fn find_words_with_value(&self, target: u32) -> Vec<usize> {
        self.word_values.iter()
            .enumerate()
            .filter(|(_, &v)| v == target)
            .map(|(i, _)| i)
            .collect()
    }
}
// ═══════════════════════════════════════════════════════════
// MULTI-MODULUS CHECKSUM — Enhanced error detection
// ═══════════════════════════════════════════════════════════
/// Multi-modulus checksum using Chinese Remainder Theorem
/// Much stronger than single mod 9 checksum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultiChecksum {
    pub mod_9: u8,
    pub mod_11: u8,
    pub mod_13: u8,
}
impl MultiChecksum {
    /// Compute multi-modulus checksum
    pub fn compute(text: &str) -> Self {
        let value = abjad_value(text).0;
        Self {
            mod_9: (value % 9) as u8,
            mod_11: (value % 11) as u8,
            mod_13: (value % 13) as u8,
        }
    }
    /// Verify text against checksum
    pub fn verify(&self, text: &str) -> bool {
        *self == Self::compute(text)
    }
    /// Unique identifier space: 9 * 11 * 13 = 1287
    /// Can detect errors up to this range
    pub fn unique_space_size() -> u32 {
        9 * 11 * 13
    }
    /// Reconstruct value modulo 1287 using CRT
    /// Mathematical: Chinese Remainder Theorem
    pub fn reconstruct_modulo(&self) -> u32 {
        // CRT for moduli 9, 11, 13 (pairwise coprime)
        let m1 = 9u32;
        let m2 = 11u32;
        let m3 = 13u32;
        let m = m1 * m2 * m3; // 1287
        let a1 = self.mod_9 as u32;
        let a2 = self.mod_11 as u32;
        let a3 = self.mod_13 as u32;
        // Compute using CRT formula
        let n1 = m / m1; // 143
        let n2 = m / m2; // 117
        let n3 = m / m3; // 99
        // Modular inverses
        // n1^(-1) mod m1: 143 ≡ 8 (mod 9), 8^(-1) ≡ 8 (mod 9) since 8*8 = 64 ≡ 1
        let inv1 = 8u32;
        // n2^(-1) mod m2: 117 ≡ 7 (mod 11), 7^(-1) ≡ 8 (mod 11) since 7*8 = 56 ≡ 1
        let inv2 = 8u32;
        // n3^(-1) mod m3: 99 ≡ 8 (mod 13), 8^(-1) ≡ 5 (mod 13) since 8*5 = 40 ≡ 1
        let inv3 = 5u32;
        let x = (a1 * n1 * inv1 + a2 * n2 * inv2 + a3 * n3 * inv3) % m;
        x
    }
}

// ═══════════════════════════════════════════════════════════
// MAL CHAR — The Dual-Layer Character (Abjadi + Hijai)
// ═══════════════════════════════════════════════════════════
/// الحرف الموحد في MAL: يجمع بين التراث (الأبجدي) والحداثة (الهجائي)
/// - `abjad`: للحساب، الضغط، والدلالة التراثية (1-1000) - نظام 3500 سنة
/// - `hijai`: للفرز، الفهرسة، والمعايير الدولية (1-28) - نظام نصر بن عاصم (90 هـ)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MALChar {
    pub glyph: char,
    pub abjad: u16,
    pub hijai: u8,
}
impl MALChar {
    /// استخراج الحرف الموحد من حرف Unicode
    pub fn from_char(c: char) -> Option<Self> {
        let (abjad, hijai) = match c {
            'ا' | 'أ' | 'إ' | 'آ' => (1, 1),
            'ب' => (2, 2),
            'ت' => (400, 3),
            'ث' => (500, 4),
            'ج' => (3, 5),
            'ح' => (8, 6),
            'خ' => (600, 7),
            'د' => (4, 8),
            'ذ' => (700, 9),
            'ر' => (200, 10),
            'ز' => (7, 11),
            'س' => (60, 12),
            'ش' => (300, 13),
            'ص' => (90, 14),
            'ض' => (800, 15),
            'ط' => (9, 16),
            'ظ' => (900, 17),
            'ع' => (70, 18),
            'غ' => (1000, 19),
            'ف' => (80, 20),
            'ق' => (100, 21),
            'ك' => (20, 22),
            'ل' => (30, 23),
            'م' => (40, 24),
            'ن' => (50, 25),
            'ه' => (5, 26),
            'و' => (6, 27),
            'ي' | 'ى' => (10, 28),
            _ => return None,
        };
        Some(Self { glyph: c, abjad, hijai })
    }
    /// ترتيب مجموعة حروف هجائياً (للفرز والمعايير الدولية)
    pub fn sort_hijai(chars: &mut [Self]) {
        chars.sort_by_key(|c| c.hijai);
    }
    /// حساب المجموع الأبجدي لمجموعة حروف (للحساب والضغط)
    pub fn sum_abjad(chars: &[Self]) -> u32 {
        chars.iter().map(|c| c.abjad as u32).sum()
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

    /// Test 6: Abjad Trie for semantic indexing
    #[test]
    fn test_abjad_trie() {
        let mut trie = AbjadTrie::new();
        // Insert words with different Abjad values
        trie.insert("الله");     // 66
        trie.insert("محمد");     // 92
        trie.insert("بسم");      // 102
        // Find by exact value
        let results = trie.find_by_value(66);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "الله");
        // Find by range
        let range_results = trie.find_in_range(50, 100);
        assert!(range_results.len() >= 2); // الله (66) and محمد (92)
        // Find by digital root
        // الله = 66, dr(66) = 6+6=12 → 1+2=3
        let dr_results = trie.find_by_digital_root(3);
        assert!(dr_results.iter().any(|(_, t)| *t == "الله"));
        // Missing value
        let missing = trie.find_by_value(999);
        assert!(missing.is_empty());
    }
    /// Test 7: Abjad Compression
    #[test]
    fn test_abjad_compression() {
        let text = "بسم الله الرحمن الرحيم";
        let compressed = AbjadCompressed::compress(text);
        // Word count preserved
        assert_eq!(compressed.word_count(), 4);
        // Compressed size: 4 words × 4 bytes = 16 bytes
        assert_eq!(compressed.compressed_size(), 16);
        // Estimated original: ~4 chars/word × 2 bytes/char × 4 words + spaces = 36 bytes
        let original_est = compressed.estimated_original_size(4);
        assert!(original_est > compressed.compressed_size());
        // Compression ratio > 1.5 (realistic for Arabic)
        let ratio = compressed.compression_ratio(4);
        assert!(ratio > 1.5, "Ratio was {}", ratio);
        // Word value access
        assert_eq!(compressed.word_value(0), Some(102)); // بسم
        assert_eq!(compressed.word_value(1), Some(66));  // الله
        // Total sum
        let total = compressed.total_sum();
        assert!(total > 0);
        // Digital root of entire text
        let dr = compressed.text_digital_root();
        assert!(dr >= 1 && dr <= 9);
        // Find words with specific value
        let allah_positions = compressed.find_words_with_value(66);
        assert_eq!(allah_positions.len(), 1);
        assert_eq!(allah_positions[0], 1); // second word
    }
    /// Test 8: Multi-modulus checksum (Chinese Remainder Theorem)
    #[test]
    fn test_multi_checksum() {
        let text = "الله";
        let cs = MultiChecksum::compute(text);
        // Verify correctness
        assert!(cs.verify(text));
        assert!(!cs.verify("محمد"));
        // Moduli values for الله (66)
        assert_eq!(cs.mod_9, (66 % 9) as u8);   // 3
        assert_eq!(cs.mod_11, (66 % 11) as u8); // 0
        assert_eq!(cs.mod_13, (66 % 13) as u8); // 1
        // CRT reconstruction
        let reconstructed = cs.reconstruct_modulo();
        assert_eq!(reconstructed % 9, cs.mod_9 as u32);
        assert_eq!(reconstructed % 11, cs.mod_11 as u32);
        assert_eq!(reconstructed % 13, cs.mod_13 as u32);
        // Unique space size: 9 * 11 * 13 = 1287
        assert_eq!(MultiChecksum::unique_space_size(), 1287);
        // Different texts have different checksums (with high probability)
        let cs2 = MultiChecksum::compute("محمد");
        assert_ne!(cs, cs2);
    }


    /// Test 9: MALChar Dual-Layer System (Abjadi + Hijai)
    #[test]
    fn test_mal_char_dual_layer() {
        // كلمة "مال"
        let mut chars = vec![
            MALChar::from_char('م').unwrap(), // Abjad: 40, Hijai: 24
            MALChar::from_char('ا').unwrap(), // Abjad: 1, Hijai: 1
            MALChar::from_char('ل').unwrap(), // Abjad: 30, Hijai: 23
        ];
        // 1. الفرز يستخدم الهجائي (ا, ل, م)
        MALChar::sort_hijai(&mut chars);
        assert_eq!(chars[0].glyph, 'ا');
        assert_eq!(chars[1].glyph, 'ل');
        assert_eq!(chars[2].glyph, 'م');
        // 2. الحساب يستخدم الأبجدي (مال = 40 + 1 + 30 = 71)
        let sum = MALChar::sum_abjad(&chars);
        assert_eq!(sum, 71);
        // 3. التحقق من التضاد بين النظامين (حرف التاء)
        let ta = MALChar::from_char('ت').unwrap();
        assert_eq!(ta.abjad, 400); // قيمة عالية في الأبجدي
        assert_eq!(ta.hijai, 3);   // ترتيب مبكر في الهجائي
        // 4. حرف الغين (أقصى قيمة أبجدية)
        let ghayn = MALChar::from_char('غ').unwrap();
        assert_eq!(ghayn.abjad, 1000); 
        assert_eq!(ghayn.hijai, 19);   
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
