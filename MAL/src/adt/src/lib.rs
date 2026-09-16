//! # MAL Algebraic Data Types (Phase 44)
//!
//! Mathematical foundation:
//!   T = C_1 | C_2 | ... | C_n     (Sum type / Coproduct)
//!   P = A × B × C                 (Product type)
//!
//! Constitutional Compliance:
//! - Principle 1 (الإحكام): Unique Constructor names
//! - Principle 2 (التيسير): Uses existing symbols (|, ⟨⟩, ×)
//! - Principle 4 (الأمانة): Linear types preserved
//! - Principle 7 (التفكر): 5 tests verify correctness
//! - Principle 9 (الوحدة الدلالية): One ADT definition
//! - Principle 11 (الأولوية الرياضية): Sigma formulation
#![forbid(unsafe_code)]
use mal_arena::{Arena, NodeID};
use std::collections::HashMap;
/// Constructor name (unique within a type)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructorName(pub String);
/// A single constructor: C(field_1: Type_1, ..., field_k: Type_k)
#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: ConstructorName,
    pub fields: Vec<FieldType>,
}
/// Field in a constructor
#[derive(Debug, Clone)]
pub struct FieldType {
    pub name: Option<String>,  // None = positional
    pub type_tag: String,      // Type name (Int, Text, etc.)
}
/// Algebraic Data Type: Sum of Products
#[derive(Debug, Clone)]
pub struct AlgebraicDataType {
    pub name: String,
    pub constructors: Vec<Constructor>,
}
impl AlgebraicDataType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            constructors: Vec::new(),
        }
    }
    /// Add a constructor (sum variant)
    pub fn add_constructor(&mut self, constructor: Constructor) -> Result<(), ADTError> {
        // Check for duplicate constructor names (Principle 1: الإحكام)
        if self.constructors.iter().any(|c| c.name == constructor.name) {
            return Err(ADTError::DuplicateConstructor {
                type_name: self.name.clone(),
                constructor_name: constructor.name.0.clone(),
            });
        }
        self.constructors.push(constructor);
        Ok(())
    }
    /// Get number of variants (for exhaustiveness check - Φ)
    pub fn variant_count(&self) -> usize {
        self.constructors.len()
    }
    /// Find constructor by name
    pub fn find_constructor(&self, name: &str) -> Option<&Constructor> {
        self.constructors.iter().find(|c| c.name.0 == name)
    }
    /// Check if type is a sum type (multiple constructors)
    pub fn is_sum_type(&self) -> bool {
        self.constructors.len() > 1
    }
    /// Check if type is a product type (single constructor with multiple fields)
    pub fn is_product_type(&self) -> bool {
        self.constructors.len() == 1 && self.constructors[0].fields.len() > 1
    }
    /// Sigma notation: T = C_1 | C_2 | ... | C_n
    pub fn to_sigma(&self) -> String {
        if self.constructors.is_empty() {
            return format!("{} = ∅", self.name);
        }
        let variants: Vec<String> = self.constructors
            .iter()
            .map(|c| {
                if c.fields.is_empty() {
                    c.name.0.clone()
                } else {
                    let fields: Vec<String> = c.fields.iter()
                        .map(|f| f.type_tag.clone())
                        .collect();
                    format!("{}({})", c.name.0, fields.join(" × "))
                }
            })
            .collect();
        format!("{} = {}", self.name, variants.join(" | "))
    }
}
/// ADT Registry — stores all defined ADTs
pub struct ADTRegistry {
    types: HashMap<String, AlgebraicDataType>,
}
impl ADTRegistry {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }
    /// Register a new ADT (Principle 9: no duplicates)
    pub fn register(&mut self, adt: AlgebraicDataType) -> Result<(), ADTError> {
        if self.types.contains_key(&adt.name) {
            return Err(ADTError::DuplicateType {
                name: adt.name.clone(),
            });
        }
        self.types.insert(adt.name.clone(), adt);
        Ok(())
    }
    /// Lookup a type by name
    pub fn lookup(&self, name: &str) -> Option<&AlgebraicDataType> {
        self.types.get(name)
    }
    /// Count registered types
    pub fn len(&self) -> usize {
        self.types.len()
    }
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
    /// List all type names
    pub fn type_names(&self) -> Vec<&String> {
        self.types.keys().collect()
    }
}
impl Default for ADTRegistry {
    fn default() -> Self {
        Self::new()
    }
}
/// ADT Errors
#[derive(Debug)]
pub enum ADTError {
    DuplicateType { name: String },
    DuplicateConstructor { type_name: String, constructor_name: String },
    UnknownConstructor { type_name: String, constructor_name: String },
    TypeMismatch { expected: String, got: String },
}
impl std::fmt::Display for ADTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ADTError::DuplicateType { name } => {
                write!(f, "نوع مكرر: {}", name)
            }
            ADTError::DuplicateConstructor { type_name, constructor_name } => {
                write!(f, "بناء مكرر في {}: {}", type_name, constructor_name)
            }
            ADTError::UnknownConstructor { type_name, constructor_name } => {
                write!(f, "بناء غير معروف: {} في {}", constructor_name, type_name)
            }
            ADTError::TypeMismatch { expected, got } => {
                write!(f, "عدم تطابق: متوقع {}، حصل {}", expected, got)
            }
        }
    }
}
impl std::error::Error for ADTError {}
#[cfg(test)]
mod tests {
    use super::*;
    /// Test 1: Simple sum type (Color = Red | Green | Blue)
    #[test]
    fn test_simple_sum_type() {
        let mut registry = ADTRegistry::new();
        let mut color = AlgebraicDataType::new("لون".to_string());
        color.add_constructor(Constructor {
            name: ConstructorName("أحمر".to_string()),
            fields: vec![],
        }).unwrap();
        color.add_constructor(Constructor {
            name: ConstructorName("أخضر".to_string()),
            fields: vec![],
        }).unwrap();
        color.add_constructor(Constructor {
            name: ConstructorName("أزرق".to_string()),
            fields: vec![],
        }).unwrap();
        registry.register(color).unwrap();
        let retrieved = registry.lookup("لون").unwrap();
        assert_eq!(retrieved.variant_count(), 3);
        assert!(retrieved.is_sum_type());
        assert!(!retrieved.is_product_type());
        assert_eq!(retrieved.to_sigma(), "لون = أحمر | أخضر | أزرق");
    }
    /// Test 2: Product type (Point = ⟨س: Int × ص: Int⟩)
    #[test]
    fn test_product_type() {
        let mut point = AlgebraicDataType::new("نقطة".to_string());
        point.add_constructor(Constructor {
            name: ConstructorName("نقطة".to_string()),
            fields: vec![
                FieldType { name: Some("س".to_string()), type_tag: "Int".to_string() },
                FieldType { name: Some("ص".to_string()), type_tag: "Int".to_string() },
            ],
        }).unwrap();
        assert!(point.is_product_type());
        assert!(!point.is_sum_type());
        assert_eq!(point.variant_count(), 1);
    }
    /// Test 3: Complex ADT (Shape = Circle | Square | Triangle)
    #[test]
    fn test_complex_adt() {
        let mut shape = AlgebraicDataType::new("شكل".to_string());
        shape.add_constructor(Constructor {
            name: ConstructorName("دائرة".to_string()),
            fields: vec![
                FieldType { name: Some("مركز".to_string()), type_tag: "نقطة".to_string() },
                FieldType { name: Some("نق".to_string()), type_tag: "عدد_عشري".to_string() },
            ],
        }).unwrap();
        shape.add_constructor(Constructor {
            name: ConstructorName("مربع".to_string()),
            fields: vec![
                FieldType { name: Some("زاوية".to_string()), type_tag: "نقطة".to_string() },
                FieldType { name: Some("طول".to_string()), type_tag: "عدد_عشري".to_string() },
            ],
        }).unwrap();
        shape.add_constructor(Constructor {
            name: ConstructorName("مثلث".to_string()),
            fields: vec![
                FieldType { name: Some("ر1".to_string()), type_tag: "نقطة".to_string() },
                FieldType { name: Some("ر2".to_string()), type_tag: "نقطة".to_string() },
                FieldType { name: Some("ر3".to_string()), type_tag: "نقطة".to_string() },
            ],
        }).unwrap();
        assert_eq!(shape.variant_count(), 3);
        assert!(shape.is_sum_type());
        let circle = shape.find_constructor("دائرة").unwrap();
        assert_eq!(circle.fields.len(), 2);
        assert_eq!(circle.fields[0].type_tag, "نقطة");
        let sigma = shape.to_sigma();
        assert!(sigma.contains("دائرة"));
        assert!(sigma.contains("مربع"));
        assert!(sigma.contains("مثلث"));
    }
    /// Test 4: Duplicate detection (Principle 1: الإحكام)
    #[test]
    fn test_duplicate_detection() {
        let mut registry = ADTRegistry::new();
        let mut t1 = AlgebraicDataType::new("نوع1".to_string());
        t1.add_constructor(Constructor {
            name: ConstructorName("A".to_string()),
            fields: vec![],
        }).unwrap();
        registry.register(t1).unwrap();
        // Try to register duplicate type
        let t2 = AlgebraicDataType::new("نوع1".to_string());
        let result = registry.register(t2);
        assert!(matches!(result, Err(ADTError::DuplicateType { .. })));
    }
    /// Test 5: Duplicate constructor detection
    #[test]
    fn test_duplicate_constructor_detection() {
        let mut t = AlgebraicDataType::new("نوع".to_string());
        t.add_constructor(Constructor {
            name: ConstructorName("A".to_string()),
            fields: vec![],
        }).unwrap();
        // Try to add duplicate constructor
        let result = t.add_constructor(Constructor {
            name: ConstructorName("A".to_string()),
            fields: vec![],
        });
        assert!(matches!(result, Err(ADTError::DuplicateConstructor { .. })));
    }
}
