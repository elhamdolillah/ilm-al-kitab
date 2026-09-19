//! # MAL Types — Mathematical Foundation Types
//!
//! Pure mathematical types for the Mathematical Arabic Language (MAL).
//! Zero dependencies — this is the foundation layer.
//!
//! ## Constitutional Compliance
//! - `#![forbid(unsafe_code)]` — no raw pointers
//! - All types are `Clone`, `Debug`, `PartialEq`, `Eq` where possible
//! - Mathematical rigor: types follow formal relational algebra semantics
//! - No semantic confusion: NULL ≠ 0, NULL ≠ false, NULL ≠ ∅
#![forbid(unsafe_code)]
#![deny(missing_docs)]
// ═══════════════════════════════════════════════════════════════
// DATA TYPES (τ)
// ═══════════════════════════════════════════════════════════════
/// Core data types for MAL (mathematical foundation).
///
/// Follows the relational model with SQL-like extensions.
///
/// # Mathematical Notation
/// - `Bool` ↔ 𝔹 (Boolean values)
/// - `Int64` ↔ ℤ₆₄ (64-bit signed integers)
/// - `Text` ↔ Text (Unicode strings)
/// - `Null` is NOT a type — it's a state within nullable columns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Boolean values (true/false) — 𝔹
    Bool,
    /// 64-bit signed integers — ℤ₆₄
    Int64,
    /// Unicode text strings — Text
    Text,
    /// Decimal with precision and scale — 𝔻_{p,s}
    Decimal {
        /// Total number of digits (1-38).
        precision: u8,
        /// Digits after decimal point (0-38).
        scale: u8,
    },
    /// 64-bit IEEE 754 floating point — 𝔽₆₄
    Float64,
    /// Byte sequences — Bytes
    Bytes,
    /// Calendar date — Date
    Date,
    /// Timestamp with optional timezone — Timestamp
    Timestamp {
        /// Whether timezone is included.
        with_timezone: bool,
    },
    /// Universally unique identifier — UUID
    Uuid,
    /// JSON document — JSON
    Json,
    // ── Composite types (future phases) ──
    // Array(Box<DataType>),
    // Set(Box<DataType>),
    // Tuple(Vec<DataType>),
    // Relation(Box<Schema>),
}
impl DataType {
    /// Check if this type is numeric (supports arithmetic).
    pub fn is_numeric(&self) -> bool {
        matches!(self, DataType::Int64 | DataType::Decimal { .. } | DataType::Float64)
    }
    /// Check if this type supports comparison operators (<, >, ≤, ≥).
    pub fn is_comparable(&self) -> bool {
        matches!(
            self,
            DataType::Int64 | DataType::Decimal { .. } | DataType::Float64 | DataType::Text | DataType::Date | DataType::Timestamp { .. }
        )
    }
}
// ═══════════════════════════════════════════════════════════════
// THREE-VALUED LOGIC (𝔹₃)
// ═══════════════════════════════════════════════════════════════
/// Three-valued logic for SQL NULL semantics.
///
/// In SQL, comparisons with NULL produce UNKNOWN (not false).
/// WHERE clause keeps only TRUE rows.
///
/// # Mathematical Notation
/// 𝔹₃ = {T, F, U}
///
/// # Truth Tables
///
/// AND: T∧T=T, T∧F=F, T∧U=U, F∧x=F, U∧U=U
/// OR:  T∨x=T, F∨F=F, F∨U=U, U∨U=U
/// NOT: ¬T=F, ¬F=T, ¬U=U
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Truth3 {
    /// True — row passes WHERE filter.
    True,
    /// False — row fails WHERE filter.
    False,
    /// Unknown — typically from NULL comparison; row fails WHERE filter.
    Unknown,
}
impl Truth3 {
    /// Logical AND (three-valued).
    ///
    /// | ∧ | T | F | U |
    /// |---|---|---|---|
    /// | T | T | F | U |
    /// | F | F | F | F |
    /// | U | U | F | U |
    pub fn and(self, other: Truth3) -> Truth3 {
        match (self, other) {
            (Truth3::False, _) | (_, Truth3::False) => Truth3::False,
            (Truth3::True, Truth3::True) => Truth3::True,
            _ => Truth3::Unknown,
        }
    }
    /// Logical OR (three-valued).
    ///
    /// | ∨ | T | F | U |
    /// |---|---|---|---|
    /// | T | T | T | T |
    /// | F | T | F | U |
    /// | U | T | U | U |
    pub fn or(self, other: Truth3) -> Truth3 {
        match (self, other) {
            (Truth3::True, _) | (_, Truth3::True) => Truth3::True,
            (Truth3::False, Truth3::False) => Truth3::False,
            _ => Truth3::Unknown,
        }
    }
    /// Logical NOT (three-valued).
    ///
    /// ¬T = F, ¬F = T, ¬U = U
    pub fn not(self) -> Truth3 {
        match self {
            Truth3::True => Truth3::False,
            Truth3::False => Truth3::True,
            Truth3::Unknown => Truth3::Unknown,
        }
    }
    /// Check if this value passes a WHERE filter.
    ///
    /// WHERE keeps only TRUE rows. FALSE and UNKNOWN are excluded.
    pub fn passes_where(self) -> bool {
        self == Truth3::True
    }
    /// Convert from two-valued bool to Truth3.
    pub fn from_bool(b: bool) -> Truth3 {
        if b { Truth3::True } else { Truth3::False }
    }
}
// ═══════════════════════════════════════════════════════════════
// SCALAR VALUES
// ═══════════════════════════════════════════════════════════════
/// Runtime scalar value with NULL support.
///
/// NULL is distinct from any other value:
/// - NULL ≠ 0
/// - NULL ≠ ""
/// - NULL ≠ false
/// - NULL ≠ ∅
#[derive(Debug, Clone, PartialEq)]
pub enum ScalarValue {
    /// Missing/unknown value.
    Null,
    /// Boolean value.
    Bool(bool),
    /// 64-bit signed integer.
    Int64(i64),
    /// Unicode text.
    Text(String),
    /// 64-bit float.
    Float64(f64),
    // Future: Decimal(DecimalValue), Date, Timestamp, Bytes, Uuid, Json
}
impl ScalarValue {
    /// Check if this value is NULL.
    pub fn is_null(&self) -> bool {
        matches!(self, ScalarValue::Null)
    }
    /// Get the DataType of this value.
    /// Returns None for Null (since Null has no specific type).
    pub fn data_type(&self) -> Option<DataType> {
        match self {
            ScalarValue::Null => None,
            ScalarValue::Bool(_) => Some(DataType::Bool),
            ScalarValue::Int64(_) => Some(DataType::Int64),
            ScalarValue::Text(_) => Some(DataType::Text),
            ScalarValue::Float64(_) => Some(DataType::Float64),
        }
    }
}
// ═══════════════════════════════════════════════════════════════
// OPERATORS
// ═══════════════════════════════════════════════════════════════
/// Binary operator (type-safe, no magic numbers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    // ── Arithmetic ──
    /// Addition (+).
    Add,
    /// Subtraction (-).
    Sub,
    /// Multiplication (·, *, ×).
    Mul,
    /// Division (÷, /).
    Div,
    /// Modulo (%).
    Mod,
    /// Power (^).
    Pow,
    /// String/list concatenation (⊕).
    Concat,
    // ── Comparison ──
    /// Less than (<).
    Lt,
    /// Greater than (>).
    Gt,
    /// Less than or equal (≤).
    Le,
    /// Greater than or equal (≥).
    Ge,
    /// Equality (=).
    Eq,
    /// Not equal (≠).
    Neq,
    // ── Logical ──
    /// Logical AND (∧).
    And,
    /// Logical OR (∨).
    Or,
    // ── Assignment ──
    /// Assignment (≔).
    Assign,
}
/// Unary operator (type-safe, no magic numbers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    /// Arithmetic negation (-x).
    Neg,
    /// Logical NOT (¬P).
    Not,
}
/// Relational algebra operator (for SQL-like queries).
///
/// # Mathematical Notation
/// - σ (sigma) = Selection/Filter
/// - π (pi) = Projection
/// - ρ (rho) = Rename
/// - ⋈ (bowtie) = Join
/// - × = Cartesian Product
/// - ∪ = Union, ∩ = Intersect, ∖ = Difference
/// - ÷ = Division
/// - γ (gamma) = Grouping/Aggregation
/// - τ (tau) = Sorting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelOp {
    /// σ — Selection/Filter (WHERE).
    Select,
    /// π — Projection (SELECT columns).
    Project,
    /// ρ — Rename (AS).
    Rename,
    /// ⋈ — Join (JOIN ... ON).
    Join,
    /// × — Cartesian product (CROSS JOIN).
    CrossProduct,
    /// ∪ — Union (UNION).
    Union,
    /// ∩ — Intersection (INTERSECT).
    Intersect,
    /// ∖ — Difference (EXCEPT).
    Difference,
    /// ÷ — Relational division.
    Divide,
    /// γ — Grouping/aggregation (GROUP BY).
    Group,
    /// τ — Sorting (ORDER BY).
    Sort,
}
// ═══════════════════════════════════════════════════════════════
// IDENTITIES
// ═══════════════════════════════════════════════════════════════
/// Unique column identifier (not name-based).
///
/// Critical for disambiguation after JOINs where multiple
/// columns may share the same name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnId(pub u32);
/// Unique relation/table identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelationId(pub u32);
/// Identifier (column name, relation name, alias).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);
impl Identifier {
    /// Create a new identifier.
    pub fn new(name: impl Into<String>) -> Self {
        Identifier(name.into())
    }
    /// Get the string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
// ═══════════════════════════════════════════════════════════════
// NULLABILITY
// ═══════════════════════════════════════════════════════════════
/// Whether a column allows NULL values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nullability {
    /// Column does not allow NULL (NOT NULL constraint).
    NotNull,
    /// Column allows NULL.
    Nullable,
}
impl Nullability {
    /// Check if nullable.
    pub fn is_nullable(self) -> bool {
        self == Nullability::Nullable
    }
    /// Merge two nullabilities (for UNION, JOIN, etc.).
    /// Result is nullable if either input is nullable.
    pub fn merge(self, other: Nullability) -> Nullability {
        if self == Nullability::Nullable || other == Nullability::Nullable {
            Nullability::Nullable
        } else {
            Nullability::NotNull
        }
    }
}
// ═══════════════════════════════════════════════════════════════
// COLUMN
// ═══════════════════════════════════════════════════════════════
/// A single column in a schema.
///
/// Mathematical definition:
/// c = (id, name, τ, ν) where:
/// - id ∈ ColId (unique identity)
/// - name ∈ Name (human-readable)
/// - τ ∈ Type (data type)
/// - ν ∈ {NOT NULL, NULLABLE}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    /// Unique column identifier.
    pub id: ColumnId,
    /// Optional parent relation (for disambiguation).
    pub relation_id: Option<RelationId>,
    /// Human-readable name.
    pub name: Identifier,
    /// Data type.
    pub data_type: DataType,
    /// Whether NULL is allowed.
    pub nullable: Nullability,
}
// ═══════════════════════════════════════════════════════════════
// SCHEMA
// ═══════════════════════════════════════════════════════════════
/// Ordered sequence of columns (Σ).
///
/// Mathematical definition:
/// Σ = ⟨c₁, c₂, ..., cₙ⟩
///
/// Order matters for:
/// - SELECT column order
/// - UNION compatibility (position-based)
/// - Row value alignment
///
/// Uniqueness constraints:
/// - ∀i≠j: id(cᵢ) ≠ id(cⱼ)  (always)
/// - ∀i≠j: name(cᵢ) ≠ name(cⱼ)  (within single relation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    /// Optional parent relation identifier.
    pub relation_id: Option<RelationId>,
    /// Ordered columns.
    pub columns: Vec<Column>,
}
impl Schema {
    /// Create an empty schema.
    pub fn empty() -> Self {
        Schema {
            relation_id: None,
            columns: Vec::new(),
        }
    }
    /// Get the number of columns (arity).
    pub fn arity(&self) -> usize {
        self.columns.len()
    }
    /// Check if schema is empty.
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }
    /// Find a column by name.
    pub fn find_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name.0 == name)
    }
    /// Find a column by qualified name (relation.column).
    pub fn find_qualified_column(&self, relation: &str, column: &str) -> Option<&Column> {
        self.columns.iter().find(|c| {
            c.name.0 == column
                && c.relation_id.is_some()
        })
    }
    /// Concatenate two schemas (for JOIN).
    /// Combines columns from both, maintaining order.
    pub fn concat(&self, other: &Schema) -> Schema {
        let mut columns = self.columns.clone();
        columns.extend(other.columns.iter().cloned());
        Schema {
            relation_id: None, // JOIN result has no single parent
            columns,
        }
    }
}
// ═══════════════════════════════════════════════════════════════
// ROW
// ═══════════════════════════════════════════════════════════════
/// A single row of values (tuple).
///
/// Mathematical definition:
/// r ∈ Row(Σ) = Dom(c₁) × Dom(c₂) × ... × Dom(cₙ)
///
/// Values must match schema arity and types.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    /// Values in schema column order.
    pub values: Vec<ScalarValue>,
}
impl Row {
    /// Create a new row.
    pub fn new(values: Vec<ScalarValue>) -> Self {
        Row { values }
    }
    /// Get the number of values.
    pub fn arity(&self) -> usize {
        self.values.len()
    }
    /// Get value at position (0-indexed).
    pub fn get(&self, index: usize) -> Option<&ScalarValue> {
        self.values.get(index)
    }
}
// ═══════════════════════════════════════════════════════════════
// SOURCE LOCATION
// ═══════════════════════════════════════════════════════════════
/// Source location for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    /// Line number (1-indexed).
    pub line: u32,
    /// Column number (1-indexed).
    pub col: u32,
    /// Length in characters.
    pub len: u16,
}
// ═══════════════════════════════════════════════════════════════
// SUPPORT STATUS
// ═══════════════════════════════════════════════════════════════
/// Feature support status for documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportStatus {
    /// Fully implemented and tested.
    Implemented,
    /// Syntax accepted but not executed.
    ParseOnly,
    /// Token reserved for future use.
    Reserved,
}
// ═══════════════════════════════════════════════════════════════
// TYPE ERRORS
// ═══════════════════════════════════════════════════════════════
/// Type and schema errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    /// Schema arity mismatch (e.g., UNION with different column counts).
    ArityMismatch {
        /// Expected number of columns.
        expected: usize,
        /// Found number of columns.
        found: usize,
    },
    /// Incompatible types in operation.
    IncompatibleTypes {
        /// Left operand type.
        left: DataType,
        /// Right operand type.
        right: DataType,
    },
    /// Column not found.
    UnknownColumn {
        /// Column name.
        name: Identifier,
    },
    /// Ambiguous column reference (exists in multiple relations).
    AmbiguousColumn {
        /// Column name.
        name: Identifier,
    },
    /// Invalid NULL in NOT NULL column.
    InvalidNull {
        /// Column name.
        column: Identifier,
    },
    /// Predicate must be boolean/Truth3.
    InvalidPredicate {
        /// Found type.
        found: DataType,
    },
}
// ═══════════════════════════════════════════════════════════════
// COERCION RULES
// ═══════════════════════════════════════════════════════════════
/// Result of type coercion check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coercion {
    /// Types are identical.
    Exact,
    /// Implicit coercion allowed (e.g., Int64 → Decimal).
    Implicit,
    /// Explicit CAST required.
    ExplicitCastRequired,
    /// Coercion impossible.
    Impossible,
}
/// Check if type τ₁ can be coerced to τ₂.
pub fn check_coercion(from: &DataType, to: &DataType) -> Coercion {
    if from == to {
        return Coercion::Exact;
    }
    match (from, to) {
        // Int64 → Decimal: implicit
        (DataType::Int64, DataType::Decimal { .. }) => Coercion::Implicit,
        // Int64 → Float64: implicit (with precision loss warning)
        (DataType::Int64, DataType::Float64) => Coercion::Implicit,
        // Everything else: explicit or impossible
        _ => Coercion::ExplicitCastRequired,
    }
}
// ═══════════════════════════════════════════════════════════════
// VALIDATION
// ═══════════════════════════════════════════════════════════════
/// Validate a scalar value against a column definition.
pub fn validate_value(
    value: &ScalarValue,
    data_type: &DataType,
    nullable: Nullability,
) -> Result<(), TypeError> {
    // NULL check
    if value.is_null() {
        if nullable == Nullability::NotNull {
            return Err(TypeError::InvalidNull {
                column: Identifier::new("(unknown)"),
            });
        }
        return Ok(());
    }
    // Type check
    let value_type = value.data_type();
    if let Some(vt) = value_type {
        if check_coercion(&vt, data_type) == Coercion::Impossible {
            return Err(TypeError::IncompatibleTypes {
                left: vt,
                right: data_type.clone(),
            });
        }
    }
    Ok(())
}
/// Validate a row against a schema.
pub fn validate_row(row: &Row, schema: &Schema) -> Result<(), TypeError> {
    // Arity check
    if row.arity() != schema.arity() {
        return Err(TypeError::ArityMismatch {
            expected: schema.arity(),
            found: row.arity(),
        });
    }
    // Per-column validation
    for (value, column) in row.values.iter().zip(&schema.columns) {
        validate_value(value, &column.data_type, column.nullable).map_err(|e| {
            if let TypeError::InvalidNull { .. } = e {
                TypeError::InvalidNull {
                    column: column.name.clone(),
                }
            } else {
                e
            }
        })?;
    }
    Ok(())
}
// ═══════════════════════════════════════════════════════════════
// QUERY NODE (placeholder for future IR)
// ═══════════════════════════════════════════════════════════════
/// Placeholder for future query IR nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum QueryNode {
    /// Placeholder — will be populated in Q1.
    Placeholder,
}
// ═══════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    // ── Truth3 tests ──
    #[test]
    fn test_truth3_and_table() {
        // T ∧ T = T
        assert_eq!(Truth3::True.and(Truth3::True), Truth3::True);
        // T ∧ F = F
        assert_eq!(Truth3::True.and(Truth3::False), Truth3::False);
        // T ∧ U = U
        assert_eq!(Truth3::True.and(Truth3::Unknown), Truth3::Unknown);
        // F ∧ x = F (for all x)
        assert_eq!(Truth3::False.and(Truth3::True), Truth3::False);
        assert_eq!(Truth3::False.and(Truth3::False), Truth3::False);
        assert_eq!(Truth3::False.and(Truth3::Unknown), Truth3::False);
        // U ∧ U = U
        assert_eq!(Truth3::Unknown.and(Truth3::Unknown), Truth3::Unknown);
    }
    #[test]
    fn test_truth3_or_table() {
        // T ∨ x = T (for all x)
        assert_eq!(Truth3::True.or(Truth3::True), Truth3::True);
        assert_eq!(Truth3::True.or(Truth3::False), Truth3::True);
        assert_eq!(Truth3::True.or(Truth3::Unknown), Truth3::True);
        // F ∨ F = F
        assert_eq!(Truth3::False.or(Truth3::False), Truth3::False);
        // F ∨ U = U
        assert_eq!(Truth3::False.or(Truth3::Unknown), Truth3::Unknown);
        // U ∨ U = U
        assert_eq!(Truth3::Unknown.or(Truth3::Unknown), Truth3::Unknown);
    }
    #[test]
    fn test_truth3_not() {
        assert_eq!(Truth3::True.not(), Truth3::False);
        assert_eq!(Truth3::False.not(), Truth3::True);
        assert_eq!(Truth3::Unknown.not(), Truth3::Unknown);
    }
    #[test]
    fn test_truth3_where_filter() {
        // WHERE keeps only TRUE
        assert!(Truth3::True.passes_where());
        assert!(!Truth3::False.passes_where());
        assert!(!Truth3::Unknown.passes_where());
    }
    // ── DataType tests ──
    #[test]
    fn test_data_type_numeric() {
        assert!(DataType::Int64.is_numeric());
        assert!(DataType::Float64.is_numeric());
        assert!(DataType::Decimal { precision: 10, scale: 2 }.is_numeric());
        assert!(!DataType::Bool.is_numeric());
        assert!(!DataType::Text.is_numeric());
    }
    #[test]
    fn test_data_type_comparable() {
        assert!(DataType::Int64.is_comparable());
        assert!(DataType::Text.is_comparable());
        assert!(DataType::Date.is_comparable());
        assert!(!DataType::Bool.is_comparable());
        assert!(!DataType::Json.is_comparable());
    }
    // ── Nullability tests ──
    #[test]
    fn test_nullability_merge() {
        assert_eq!(
            Nullability::NotNull.merge(Nullability::NotNull),
            Nullability::NotNull
        );
        assert_eq!(
            Nullability::NotNull.merge(Nullability::Nullable),
            Nullability::Nullable
        );
        assert_eq!(
            Nullability::Nullable.merge(Nullability::NotNull),
            Nullability::Nullable
        );
        assert_eq!(
            Nullability::Nullable.merge(Nullability::Nullable),
            Nullability::Nullable
        );
    }
    // ── Schema tests ──
    #[test]
    fn test_schema_concat() {
        let s1 = Schema {
            relation_id: Some(RelationId(1)),
            columns: vec![Column {
                id: ColumnId(1),
                relation_id: Some(RelationId(1)),
                name: Identifier::new("id"),
                data_type: DataType::Int64,
                nullable: Nullability::NotNull,
            }],
        };
        let s2 = Schema {
            relation_id: Some(RelationId(2)),
            columns: vec![Column {
                id: ColumnId(2),
                relation_id: Some(RelationId(2)),
                name: Identifier::new("name"),
                data_type: DataType::Text,
                nullable: Nullability::Nullable,
            }],
        };
        let joined = s1.concat(&s2);
        assert_eq!(joined.arity(), 2);
        assert_eq!(joined.columns[0].name.0, "id");
        assert_eq!(joined.columns[1].name.0, "name");
    }
    // ── Coercion tests ──
    #[test]
    fn test_coercion_exact() {
        assert_eq!(check_coercion(&DataType::Int64, &DataType::Int64), Coercion::Exact);
        assert_eq!(check_coercion(&DataType::Text, &DataType::Text), Coercion::Exact);
    }
    #[test]
    fn test_coercion_implicit() {
        assert_eq!(
            check_coercion(&DataType::Int64, &DataType::Decimal { precision: 10, scale: 2 }),
            Coercion::Implicit
        );
        assert_eq!(
            check_coercion(&DataType::Int64, &DataType::Float64),
            Coercion::Implicit
        );
    }
    // ── Row validation tests ──
    #[test]
    fn test_validate_row_ok() {
        let schema = Schema {
            relation_id: None,
            columns: vec![
                Column {
                    id: ColumnId(1),
                    relation_id: None,
                    name: Identifier::new("id"),
                    data_type: DataType::Int64,
                    nullable: Nullability::NotNull,
                },
                Column {
                    id: ColumnId(2),
                    relation_id: None,
                    name: Identifier::new("name"),
                    data_type: DataType::Text,
                    nullable: Nullability::Nullable,
                },
            ],
        };
        let row = Row::new(vec![
            ScalarValue::Int64(1),
            ScalarValue::Text("Ada".to_string()),
        ]);
        assert!(validate_row(&row, &schema).is_ok());
    }
    #[test]
    fn test_validate_row_arity_mismatch() {
        let schema = Schema {
            relation_id: None,
            columns: vec![Column {
                id: ColumnId(1),
                relation_id: None,
                name: Identifier::new("id"),
                data_type: DataType::Int64,
                nullable: Nullability::NotNull,
            }],
        };
        let row = Row::new(vec![
            ScalarValue::Int64(1),
            ScalarValue::Int64(2),
        ]);
        assert!(matches!(
            validate_row(&row, &schema),
            Err(TypeError::ArityMismatch { .. })
        ));
    }
    #[test]
    fn test_validate_row_null_in_not_null() {
        let schema = Schema {
            relation_id: None,
            columns: vec![Column {
                id: ColumnId(1),
                relation_id: None,
                name: Identifier::new("id"),
                data_type: DataType::Int64,
                nullable: Nullability::NotNull,
            }],
        };
        let row = Row::new(vec![ScalarValue::Null]);
        assert!(matches!(
            validate_row(&row, &schema),
            Err(TypeError::InvalidNull { .. })
        ));
    }
}
