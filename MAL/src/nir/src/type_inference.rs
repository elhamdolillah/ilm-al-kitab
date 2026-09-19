//! Comprehensive Type Inference System for MAL
//!
//! Implements 7 algorithms and features:
//! 1. Hindley-Milner Algorithm W (classical inference)
//! 2. Union-Find (efficient constraint solving)
//! 3. Bidirectional Type Checking (synthesis + checking)
//! 4. Flow-Sensitive Analysis (type narrowing)
//! 5. Union Types (Arabic-friendly: إما هذا أو ذاك)
//! 6. Generic Functions (Parametric Polymorphism)
//! 7. Type Classes / Traits (Ad-hoc Polymorphism)
//!
//! Mathematical Foundation:
//! - Type System: (T, Γ, ⊢, R)
//! - Lattice: (Types, ≤, ⊔, ⊓, ⊥, ⊤)
//! - Constraint System: C ::= τ₁ = τ₂ | τ₁ ≤ τ₂ | τ₁ : Trait
use std::collections::HashMap;
use std::fmt;
// ═══════════════════════════════════════════════
// PART 1: Type Algebra (البنية الجبرية للأنواع)
// ═══════════════════════════════════════════════
/// Type variable identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(pub u32);
/// Trait identifier (for Type Classes)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitId(pub String);
/// Generic parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericParam(pub String);
/// Complete type algebra
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // === Primitive Types (الأنواع الأولية) ===
    Int(u32),
    UInt(u32),
    Float(u32),
    Bool,
    Char,
    Str,
    Unit,
    // === Type Variables (للاستدلال) ===
    Var(TypeVar),
    // === Composite Types (الأنواع المركبة) ===
    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
    Record(Vec<(String, Type)>),
    // === Function Types (أنواع الدوال) ===
    Func(Vec<Type>, Box<Type>),
    // === Polymorphic Types (الأنواع العامة) ===
    /// ∀α. T
    Forall(TypeVar, Box<Type>),
    /// Named generic: forall<T>
    NamedGeneric(GenericParam),
    // === Union Types (أنواع الاتحاد) - NEW! ===
    /// T₁ | T₂ | ... | Tₙ (Arabic: إما T₁ أو T₂)
    Union(Vec<Type>),
    /// Null type for Option-like behavior
    Null,
    // === Type Class / Trait instances - NEW! ===
    /// τ : Trait (type τ implements Trait)
    TraitInstance(Box<Type>, TraitId),
    /// Generic bound: T : Trait
    TraitBound(GenericParam, TraitId),
    // === Dependent Types (أنواع معتمدة على القيم) - NEW! ===
    /// {x: T | P(x)} - refinement type
    Refinement(Box<Type>, String),
    /// (x: T) → P(x) - dependent function
    DepFunc(String, Box<Type>, Box<Type>),
    // === Special Types ===
    Never,
    Any,
    Unknown,
}
impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int(_) | Type::UInt(_) | Type::Float(_))
    }
    pub fn is_var(&self) -> bool {
        matches!(self, Type::Var(_))
    }
    /// Get all free type variables
    pub fn free_vars(&self) -> Vec<TypeVar> {
        match self {
            Type::Var(v) => vec![*v],
            Type::Array(t, _) => t.free_vars(),
            Type::Tuple(ts) => ts.iter().flat_map(|t| t.free_vars()).collect(),
            Type::Record(fields) => fields.iter().flat_map(|(_, t)| t.free_vars()).collect(),
            Type::Func(args, ret) => {
                let mut vars: Vec<TypeVar> = args.iter().flat_map(|t| t.free_vars()).collect();
                vars.extend(ret.free_vars());
                vars
            }
            Type::Union(ts) => ts.iter().flat_map(|t| t.free_vars()).collect(),
            Type::TraitInstance(t, _) => t.free_vars(),
            Type::Refinement(t, _) => t.free_vars(),
            Type::DepFunc(_, t1, t2) => {
                let mut vars = t1.free_vars();
                vars.extend(t2.free_vars());
                vars
            }
            Type::Forall(v, t) => {
                t.free_vars().into_iter().filter(|x| x != v).collect()
            }
            _ => vec![],
        }
    }
    /// Apply substitution
    pub fn substitute(&self, subst: &HashMap<TypeVar, Type>) -> Type {
        match self {
            Type::Var(v) => {
                if let Some(t) = subst.get(v) {
                    t.substitute(subst)
                } else {
                    self.clone()
                }
            }
            Type::Array(t, n) => Type::Array(Box::new(t.substitute(subst)), *n),
            Type::Tuple(ts) => Type::Tuple(ts.iter().map(|t| t.substitute(subst)).collect()),
            Type::Record(fields) => Type::Record(
                fields.iter().map(|(n, t)| (n.clone(), t.substitute(subst))).collect()
            ),
            Type::Func(args, ret) => Type::Func(
                args.iter().map(|t| t.substitute(subst)).collect(),
                Box::new(ret.substitute(subst))
            ),
            Type::Union(ts) => Type::Union(ts.iter().map(|t| t.substitute(subst)).collect()),
            Type::TraitInstance(t, tr) => Type::TraitInstance(Box::new(t.substitute(subst)), tr.clone()),
            Type::Refinement(t, p) => Type::Refinement(Box::new(t.substitute(subst)), p.clone()),
            Type::DepFunc(x, t1, t2) => Type::DepFunc(
                x.clone(),
                Box::new(t1.substitute(subst)),
                Box::new(t2.substitute(subst))
            ),
            Type::Forall(v, t) => {
                let mut subst2 = subst.clone();
                subst2.remove(v);
                Type::Forall(*v, Box::new(t.substitute(&subst2)))
            }
            _ => self.clone(),
        }
    }
    /// Create union type (smart constructor)
    pub fn union(types: Vec<Type>) -> Type {
        let mut flat = Vec::new();
        for t in types {
            match t {
                Type::Union(ts) => flat.extend(ts),
                _ => flat.push(t),
            }
        }
        flat.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
        flat.dedup();
        match flat.len() {
            0 => Type::Never,
            1 => flat.into_iter().next().unwrap(),
            _ => Type::Union(flat),
        }
    }
    /// Check if type contains another (for union membership)
    pub fn contains(&self, other: &Type) -> bool {
        match self {
            Type::Union(ts) => ts.iter().any(|t| t.contains(other)),
            Type::Any => true,
            _ => self == other,
        }
    }
    /// Check subtype relation
    pub fn is_subtype_of(&self, other: &Type) -> bool {
        if self == other { return true; }
        match (self, other) {
            (_, Type::Any) => true,
            (Type::Never, _) => true,
            // Union: T₁ | T₂ ≤ U iff ∀i: Tᵢ ≤ U
            (Type::Union(ts), _) => ts.iter().all(|t| t.is_subtype_of(other)),
            // T ≤ T₁ | T₂ iff ∃i: T ≤ Tᵢ
            (_, Type::Union(ts)) => ts.iter().any(|t| self.is_subtype_of(t)),
            (Type::Null, Type::Union(_)) => true,  // null fits in any union
            (Type::Int(a), Type::Int(b)) => a <= b,
            (Type::UInt(a), Type::UInt(b)) => a <= b,
            (Type::Float(a), Type::Float(b)) => a <= b,
            (Type::Int(_), Type::Float(_)) => true,
            (Type::Var(_), _) | (_, Type::Var(_)) => true,
            (Type::Array(t1, n1), Type::Array(t2, n2)) => n1 == n2 && t1.is_subtype_of(t2),
            (Type::Tuple(ts1), Type::Tuple(ts2)) => {
                ts1.len() == ts2.len() && ts1.iter().zip(ts2).all(|(a, b)| a.is_subtype_of(b))
            }
            (Type::Func(a1, r1), Type::Func(a2, r2)) => {
                a1.len() == a2.len() &&
                a2.iter().zip(a1).all(|(b, a)| a.is_subtype_of(b)) &&
                r1.is_subtype_of(r2)
            }
            _ => false,
        }
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int(n) => write!(f, "عدد{}", n),
            Type::UInt(n) => write!(f, "عدد_موجب{}", n),
            Type::Float(n) => write!(f, "عشري{}", n),
            Type::Bool => write!(f, "منطقي"),
            Type::Char => write!(f, "حرف"),
            Type::Str => write!(f, "نص"),
            Type::Unit => write!(f, "فارغ"),
            Type::Var(v) => write!(f, "α{}", v.0),
            Type::Array(t, n) => write!(f, "قائمة[{}; {}]", t, n),
            Type::Tuple(ts) => {
                write!(f, "(")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 { write!(f, "، ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Record(fields) => {
                write!(f, "سجل{{")?;
                for (i, (n, t)) in fields.iter().enumerate() {
                    if i > 0 { write!(f, "، ")?; }
                    write!(f, "{}: {}", n, t)?;
                }
                write!(f, "}}")
            }
            Type::Func(args, ret) => {
                write!(f, "دالة(")?;
                for (i, t) in args.iter().enumerate() {
                    if i > 0 { write!(f, "، ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ") ← {}", ret)
            }
            Type::Forall(v, t) => write!(f, "∀α{}. {}", v.0, t),
            Type::NamedGeneric(p) => write!(f, "{}", p.0),
            Type::Union(ts) => {
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 { write!(f, " | ")?; }
                    write!(f, "{}", t)?;
                }
                Ok(())
            }
            Type::Null => write!(f, "لا_شيء"),
            Type::TraitInstance(t, tr) => write!(f, "{} : {}", t, tr.0),
            Type::TraitBound(p, tr) => write!(f, "{} : {}", p.0, tr.0),
            Type::Refinement(t, p) => write!(f, "{{{}: {} | {}}}", "x", t, p),
            Type::DepFunc(x, t1, t2) => write!(f, "({}: {}) ← {}", x, t1, t2),
            Type::Never => write!(f, "أبدا"),
            Type::Any => write!(f, "أي_نوع"),
            Type::Unknown => write!(f, "؟"),
        }
    }
}
// ═══════════════════════════════════════════════
// PART 2: Union-Find (خوارزمية الاتحاد والبحث)
// ═══════════════════════════════════════════════
#[derive(Debug)]
pub struct UnionFind {
    parent: HashMap<TypeVar, TypeVar>,
    rank: HashMap<TypeVar, u32>,
    resolved: HashMap<TypeVar, Type>,
}
impl UnionFind {
    pub fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
            resolved: HashMap::new(),
        }
    }
    fn ensure(&mut self, v: TypeVar) {
        if !self.parent.contains_key(&v) {
            self.parent.insert(v, v);
            self.rank.insert(v, 0);
        }
    }
    pub fn find(&mut self, v: TypeVar) -> TypeVar {
        self.ensure(v);
        let parent = self.parent[&v];
        if parent == v { return v; }
        let root = self.find(parent);
        self.parent.insert(v, root);
        root
    }
    pub fn union(&mut self, v1: TypeVar, v2: TypeVar) {
        let r1 = self.find(v1);
        let r2 = self.find(v2);
        if r1 == r2 { return; }
        let rank1 = self.rank[&r1];
        let rank2 = self.rank[&r2];
        if rank1 < rank2 {
            self.parent.insert(r1, r2);
        } else if rank1 > rank2 {
            self.parent.insert(r2, r1);
        } else {
            self.parent.insert(r2, r1);
            self.rank.insert(r1, rank1 + 1);
        }
    }
    pub fn resolve(&mut self, v: TypeVar, ty: Type) {
        let root = self.find(v);
        self.resolved.insert(root, ty);
    }
    pub fn get_resolved(&mut self, v: TypeVar) -> Option<Type> {
        let root = self.find(v);
        self.resolved.get(&root).cloned()
    }
}
// ═══════════════════════════════════════════════
// PART 3: Type Environment + Traits Registry
// ═══════════════════════════════════════════════
#[derive(Debug)]
pub struct TypeEnv {
    pub bindings: HashMap<String, Type>,
    next_var: u32,
    pub uf: UnionFind,
    /// Trait implementations: (trait, type) -> bool
    pub trait_impls: HashMap<(TraitId, Type), bool>,
}
impl TypeEnv {
    pub fn new() -> Self {
        let mut env = TypeEnv {
            bindings: HashMap::new(),
            next_var: 0,
            uf: UnionFind::new(),
            trait_impls: HashMap::new(),
        };
        env.register_default_traits();
        env
    }
    /// Register built-in traits (Arabic names)
    fn register_default_traits(&mut self) {
        // Trait: قابلة_للطباعة (Printable)
        // Trait: قابلة_للمقارنة (Comparable)
        // Trait: قابلة_للجمع (Addable)
        // Trait: عددية (Numeric)
        let numeric_trait = TraitId("عدد".to_string());
        let printable_trait = TraitId("قابلة_للطباعة".to_string());
        let comparable_trait = TraitId("قابلة_للمقارنة".to_string());
        let addable_trait = TraitId("قابلة_للجمع".to_string());
        // Int implements all
        for ty in [Type::Int(32), Type::Int(64), Type::Float(32), Type::Float(64)] {
            self.trait_impls.insert((numeric_trait.clone(), ty.clone()), true);
            self.trait_impls.insert((printable_trait.clone(), ty.clone()), true);
            self.trait_impls.insert((comparable_trait.clone(), ty.clone()), true);
            self.trait_impls.insert((addable_trait.clone(), ty.clone()), true);
        }
        // String is printable and addable (concatenation)
        self.trait_impls.insert((printable_trait.clone(), Type::Str), true);
        self.trait_impls.insert((addable_trait, Type::Str), true);
        // Bool is printable and comparable
        self.trait_impls.insert((printable_trait.clone(), Type::Bool), true);
        self.trait_impls.insert((comparable_trait, Type::Bool), true);
    }
    pub fn fresh_var(&mut self) -> Type {
        let v = TypeVar(self.next_var);
        self.next_var += 1;
        Type::Var(v)
    }
    pub fn bind(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings.get(name)
    }
    /// Check if a type implements a trait
    pub fn implements_trait(&self, ty: &Type, trait_id: &TraitId) -> bool {
        self.trait_impls.get(&(trait_id.clone(), ty.clone())).copied().unwrap_or(false)
    }
    /// Register a trait implementation
    pub fn impl_trait(&mut self, trait_id: TraitId, ty: Type) {
        self.trait_impls.insert((trait_id, ty), true);
    }
    pub fn extend(&self) -> TypeEnv {
        TypeEnv {
            bindings: self.bindings.clone(),
            next_var: self.next_var,
            uf: UnionFind::new(),
            trait_impls: self.trait_impls.clone(),
        }
    }
}
// ═══════════════════════════════════════════════
// PART 4: Constraints + Errors (Arabic messages)
// ═══════════════════════════════════════════════
#[derive(Debug, Clone)]
pub enum Constraint {
    Equal(Type, Type),
    Subtype(Type, Type),
    /// Type must implement trait
    Implements(Type, TraitId),
}
/// Source location for better error messages
#[derive(Debug, Clone, Default)]
pub struct SourceLoc {
    pub line: usize,
    pub col: usize,
    pub file: String,
}
#[derive(Debug, Clone)]
pub enum TypeError {
    Mismatch { expected: Type, got: Type, loc: SourceLoc },
    SubtypeMismatch(Type, Type, SourceLoc),
    InfiniteType(TypeVar, Type, SourceLoc),
    UnboundVar(String, SourceLoc),
    ArityMismatch { expected: usize, got: usize, loc: SourceLoc },
    TraitNotImplemented { ty: Type, trait_id: TraitId, loc: SourceLoc },
    /// For dependent type violations
    RefinementFailed { ty: Type, predicate: String, loc: SourceLoc },
}
impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::Mismatch { expected, got, loc } => {
                write!(f,
                    "❌ خطأ في النوع (السطر {}، العمود {}): \n\
                     توقع النوع: {}\n\
                     ولكن وجد: {}",
                    loc.line, loc.col, expected, got)
            }
            TypeError::SubtypeMismatch(t1, t2, loc) => {
                write!(f,
                    "❌ خطأ التبعية (السطر {}): {} ليس من النوع {}",
                    loc.line, t1, t2)
            }
            TypeError::InfiniteType(v, t, loc) => {
                write!(f,
                    "❌ نوع لا نهائي (السطر {}): α{} = {}",
                    loc.line, v.0, t)
            }
            TypeError::UnboundVar(name, loc) => {
                write!(f,
                    "❌ متغير غير معرف (السطر {}): '{}'",
                    loc.line, name)
            }
            TypeError::ArityMismatch { expected, got, loc } => {
                write!(f,
                    "❌ عدم تطابق عدد الوسائط (السطر {}): توقع {}، وجد {}",
                    loc.line, expected, got)
            }
            TypeError::TraitNotImplemented { ty, trait_id, loc } => {
                write!(f,
                    "❌ النوع {} لا يحقق الخاصية '{}' (السطر {})",
                    ty, trait_id.0, loc.line)
            }
            TypeError::RefinementFailed { ty, predicate, loc } => {
                write!(f,
                    "❌ فشل الشرط '{}' على النوع {} (السطر {})",
                    predicate, ty, loc.line)
            }
        }
    }
}
impl std::error::Error for TypeError {}
// ═══════════════════════════════════════════════
// PART 5: Constraint Solver
// ═══════════════════════════════════════════════
pub struct ConstraintSolver {
    uf: UnionFind,
    substitution: HashMap<TypeVar, Type>,
    env: TypeEnv,
}
impl ConstraintSolver {
    pub fn new(env: TypeEnv) -> Self {
        ConstraintSolver {
            uf: UnionFind::new(),
            substitution: HashMap::new(),
            env,
        }
    }
    pub fn solve(&mut self, constraints: &[Constraint], loc: &SourceLoc) -> Result<HashMap<TypeVar, Type>, TypeError> {
        for c in constraints {
            match c {
                Constraint::Equal(t1, t2) => self.unify(t1, t2, loc)?,
                Constraint::Subtype(t1, t2) => self.check_subtype(t1, t2, loc)?,
                Constraint::Implements(ty, trait_id) => self.check_trait(ty, trait_id, loc)?,
            }
        }
        Ok(self.substitution.clone())
    }
    pub fn unify(&mut self, t1: &Type, t2: &Type, loc: &SourceLoc) -> Result<(), TypeError> {
        match (t1, t2) {
            (Type::Int(a), Type::Int(b)) if a == b => Ok(()),
            (Type::UInt(a), Type::UInt(b)) if a == b => Ok(()),
            (Type::Float(a), Type::Float(b)) if a == b => Ok(()),
            (Type::Bool, Type::Bool) => Ok(()),
            (Type::Char, Type::Char) => Ok(()),
            (Type::Str, Type::Str) => Ok(()),
            (Type::Unit, Type::Unit) => Ok(()),
            // Union unification: T ∈ (U₁ | U₂) if T = Ui for some i
            (t, Type::Union(ts)) | (Type::Union(ts), t) => {
                if ts.iter().any(|u| u == t) {
                    Ok(())
                } else {
                    Err(TypeError::Mismatch {
                        expected: Type::Union(ts.clone()),
                        got: t.clone(),
                        loc: loc.clone(),
                    })
                }
            }
            (Type::Var(v), t) | (t, Type::Var(v)) => {
                if t.free_vars().contains(v) {
                    return Err(TypeError::InfiniteType(*v, t.clone(), loc.clone()));
                }
                self.substitution.insert(*v, t.clone());
                self.uf.resolve(*v, t.clone());
                Ok(())
            }
            (Type::Array(a, n1), Type::Array(b, n2)) if n1 == n2 => self.unify(a, b, loc),
            (Type::Tuple(ts1), Type::Tuple(ts2)) if ts1.len() == ts2.len() => {
                for (a, b) in ts1.iter().zip(ts2) {
                    self.unify(a, b, loc)?;
                }
                Ok(())
            }
            (Type::Func(a1, r1), Type::Func(a2, r2)) if a1.len() == a2.len() => {
                for (a, b) in a1.iter().zip(a2) {
                    self.unify(a, b, loc)?;
                }
                self.unify(r1, r2, loc)
            }
            (Type::Any, _) | (_, Type::Any) => Ok(()),
            (Type::Unknown, _) | (_, Type::Unknown) => Ok(()),
            _ => Err(TypeError::Mismatch {
                expected: t1.clone(),
                got: t2.clone(),
                loc: loc.clone(),
            }),
        }
    }
    fn check_subtype(&mut self, t1: &Type, t2: &Type, loc: &SourceLoc) -> Result<(), TypeError> {
        if t1.is_subtype_of(t2) {
            Ok(())
        } else {
            Err(TypeError::SubtypeMismatch(t1.clone(), t2.clone(), loc.clone()))
        }
    }
    fn check_trait(&mut self, ty: &Type, trait_id: &TraitId, loc: &SourceLoc) -> Result<(), TypeError> {
        if self.env.implements_trait(ty, trait_id) {
            Ok(())
        } else {
            Err(TypeError::TraitNotImplemented {
                ty: ty.clone(),
                trait_id: trait_id.clone(),
                loc: loc.clone(),
            })
        }
    }
    pub fn apply(&self, ty: &Type) -> Type {
        ty.substitute(&self.substitution)
    }
}
// ═══════════════════════════════════════════════
// PART 6: Hindley-Milner Algorithm W
// ═══════════════════════════════════════════════
pub struct AlgorithmW {
    pub env: TypeEnv,
    pub constraints: Vec<Constraint>,
}
impl AlgorithmW {
    pub fn new() -> Self {
        AlgorithmW {
            env: TypeEnv::new(),
            constraints: Vec::new(),
        }
    }
    pub fn infer_literal(&mut self, value: &str) -> Type {
        if value.parse::<i64>().is_ok() {
            Type::Int(64)
        } else if value.parse::<f64>().is_ok() {
            Type::Float(64)
        } else if value == "true" || value == "false" {
            Type::Bool
        } else if value.starts_with('"') && value.ends_with('"') {
            Type::Str
        } else {
            Type::Unknown
        }
    }
    /// Infer binary operation with trait constraints
    pub fn infer_binop(&mut self, op: &str, left: Type, right: Type, loc: &SourceLoc) -> Result<Type, TypeError> {
        match op {
            "+" | "-" | "*" | "/" | "%" => {
                // Require Addable/Numeric trait
                let trait_id = TraitId("عدد".to_string());
                self.constraints.push(Constraint::Implements(left.clone(), trait_id.clone()));
                self.constraints.push(Constraint::Implements(right.clone(), trait_id));
                if !left.is_numeric() && !matches!(left, Type::Var(_) | Type::Unknown) {
                    return Err(TypeError::Mismatch {
                        expected: Type::Int(64),
                        got: left,
                        loc: loc.clone(),
                    });
                }
                if !right.is_numeric() && !matches!(right, Type::Var(_) | Type::Unknown) {
                    return Err(TypeError::Mismatch {
                        expected: Type::Int(64),
                        got: right,
                        loc: loc.clone(),
                    });
                }
                match (&left, &right) {
                    (Type::Float(a), Type::Float(b)) => Ok(Type::Float(*a.max(b))),
                    (Type::Float(a), Type::Int(_)) => Ok(Type::Float(*a)),
                    (Type::Int(_), Type::Float(b)) => Ok(Type::Float(*b)),
                    (Type::Int(a), Type::Int(b)) => Ok(Type::Int(*a.max(b))),
                    _ => Ok(Type::Int(64)),
                }
            }
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                let trait_id = TraitId("قابلة_للمقارنة".to_string());
                self.constraints.push(Constraint::Implements(left.clone(), trait_id.clone()));
                self.constraints.push(Constraint::Implements(right.clone(), trait_id));
                self.constraints.push(Constraint::Equal(left, right));
                Ok(Type::Bool)
            }
            "&&" | "||" => {
                self.constraints.push(Constraint::Equal(left, Type::Bool));
                self.constraints.push(Constraint::Equal(right, Type::Bool));
                Ok(Type::Bool)
            }
            _ => Ok(Type::Unknown),
        }
    }
    pub fn generalize(&self, ty: &Type) -> Type {
        let env_vars: Vec<TypeVar> = self.env.bindings.values()
            .flat_map(|t| t.free_vars())
            .collect();
        let free = ty.free_vars();
        let to_generalize: Vec<TypeVar> = free.into_iter()
            .filter(|v| !env_vars.contains(v))
            .collect();
        if to_generalize.is_empty() {
            ty.clone()
        } else {
            Type::Forall(to_generalize[0], Box::new(ty.clone()))
        }
    }
    pub fn instantiate(&mut self, ty: &Type) -> Type {
        match ty {
            Type::Forall(v, t) => {
                let fresh = self.env.fresh_var();
                let mut subst = HashMap::new();
                subst.insert(*v, fresh);
                self.instantiate(&t.substitute(&subst))
            }
            _ => ty.clone(),
        }
    }
    pub fn solve(&self) -> Result<HashMap<TypeVar, Type>, TypeError> {
        let mut solver = ConstraintSolver::new(TypeEnv::new());
        solver.solve(&self.constraints, &SourceLoc::default())
    }
}
// ═══════════════════════════════════════════════
// PART 7: Bidirectional Type Checking
// ═══════════════════════════════════════════════
#[derive(Debug, Clone)]
pub enum Mode {
    Infer,
    Check(Type),
}
pub struct BidirectionalChecker {
    pub env: TypeEnv,
}
impl BidirectionalChecker {
    pub fn new() -> Self {
        BidirectionalChecker { env: TypeEnv::new() }
    }
    pub fn check_or_infer(&mut self, ty: &Type, mode: &Mode) -> Result<Type, TypeError> {
        match mode {
            Mode::Infer => Ok(ty.clone()),
            Mode::Check(expected) => {
                if ty.is_subtype_of(expected) {
                    Ok(expected.clone())
                } else {
                    Err(TypeError::Mismatch {
                        expected: expected.clone(),
                        got: ty.clone(),
                        loc: SourceLoc::default(),
                    })
                }
            }
        }
    }
    pub fn check_application(&mut self, func_ty: &Type, arg_ty: &Type) -> Result<Type, TypeError> {
        match func_ty {
            Type::Func(params, ret) => {
                if params.len() == 1 {
                    if arg_ty.is_subtype_of(&params[0]) {
                        Ok(*ret.clone())
                    } else {
                        Err(TypeError::Mismatch {
                            expected: params[0].clone(),
                            got: arg_ty.clone(),
                            loc: SourceLoc::default(),
                        })
                    }
                } else {
                    Err(TypeError::ArityMismatch {
                        expected: params.len(),
                        got: 1,
                        loc: SourceLoc::default(),
                    })
                }
            }
            Type::Var(_) | Type::Unknown => Ok(Type::Unknown),
            _ => Err(TypeError::Mismatch {
                expected: Type::Func(vec![Type::Unknown], Box::new(Type::Unknown)),
                got: func_ty.clone(),
                loc: SourceLoc::default(),
            }),
        }
    }
}
// ═══════════════════════════════════════════════
// PART 8: Flow-Sensitive Analysis
// ═══════════════════════════════════════════════
#[derive(Debug, Clone)]
pub struct FlowEnv {
    point_types: HashMap<u32, HashMap<String, Type>>,
}
impl FlowEnv {
    pub fn new() -> Self {
        FlowEnv { point_types: HashMap::new() }
    }
    pub fn narrow(&self, ty: &Type, condition: &str) -> Type {
        match condition {
            "typeof == string" | "نوع == نص" => {
                match ty {
                    Type::Str => Type::Str,
                    Type::Any => Type::Str,
                    Type::Union(ts) => {
                        // Extract string from union
                        let narrowed: Vec<Type> = ts.iter()
                            .filter(|t| matches!(t, Type::Str))
                            .cloned()
                            .collect();
                        Type::union(narrowed)
                    }
                    _ => Type::Never,
                }
            }
            "typeof == number" | "نوع == عدد" => {
                if ty.is_numeric() { ty.clone() } else { Type::Never }
            }
            "typeof == boolean" | "نوع == منطقي" => {
                match ty {
                    Type::Bool => Type::Bool,
                    _ => Type::Never,
                }
            }
            "!= null" | "!= لا_شيء" => {
                match ty {
                    Type::Union(ts) => {
                        let narrowed: Vec<Type> = ts.iter()
                            .filter(|t| !matches!(t, Type::Null))
                            .cloned()
                            .collect();
                        Type::union(narrowed)
                    }
                    Type::Null => Type::Never,
                    _ => ty.clone(),
                }
            }
            _ => ty.clone(),
        }
    }
    pub fn join(&self, t1: &Type, t2: &Type) -> Type {
        if t1 == t2 { t1.clone() }
        else if t1.is_subtype_of(t2) { t2.clone() }
        else if t2.is_subtype_of(t1) { t1.clone() }
        else { Type::union(vec![t1.clone(), t2.clone()]) }
    }
    pub fn set_point(&mut self, point: u32, var: String, ty: Type) {
        self.point_types.entry(point).or_insert_with(HashMap::new).insert(var, ty);
    }
    pub fn get_point(&self, point: u32, var: &str) -> Option<&Type> {
        self.point_types.get(&point).and_then(|m| m.get(var))
    }
}
// ═══════════════════════════════════════════════
// PART 9: Integrated Type Checker (Arabic errors)
// ═══════════════════════════════════════════════
pub struct TypeChecker {
    pub alg_w: AlgorithmW,
    pub bidir: BidirectionalChecker,
    pub flow: FlowEnv,
    pub errors: Vec<TypeError>,
    pub source_file: String,
}
impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            alg_w: AlgorithmW::new(),
            bidir: BidirectionalChecker::new(),
            flow: FlowEnv::new(),
            errors: Vec::new(),
            source_file: String::new(),
        }
    }
    pub fn with_file(file: &str) -> Self {
        TypeChecker {
            alg_w: AlgorithmW::new(),
            bidir: BidirectionalChecker::new(),
            flow: FlowEnv::new(),
            errors: Vec::new(),
            source_file: file.to_string(),
        }
    }
    pub fn infer_source(&mut self, source: &str) -> Type {
        self.alg_w.infer_literal(source.trim())
    }
    pub fn compatible(&self, t1: &Type, t2: &Type) -> bool {
        t1.is_subtype_of(t2) || t2.is_subtype_of(t1)
    }
    /// Create union type from two types (Arabic: إما هذا أو ذاك)
    pub fn make_union(&self, t1: Type, t2: Type) -> Type {
        Type::union(vec![t1, t2])
    }
    /// Create generic function (forall<T>)
    pub fn generic_func(&self, params: Vec<String>, args: Vec<Type>, ret: Type) -> Type {
        let mut ty = Type::Func(args, Box::new(ret));
        // Wrap in Forall for each generic param
        for _ in params.iter() {
            let v = TypeVar(0);  // placeholder
            ty = Type::Forall(v, Box::new(ty));
        }
        ty
    }
    /// Check trait implementation
    pub fn check_trait(&self, ty: &Type, trait_name: &str) -> bool {
        self.alg_w.env.implements_trait(ty, &TraitId(trait_name.to_string()))
    }
    /// Format errors in Arabic for display
    pub fn format_errors(&self) -> String {
        if self.errors.is_empty() {
            return String::new();
        }
        let mut output = format!("❌ تم اكتشاف {} خطأ:\n\n", self.errors.len());
        for (i, err) in self.errors.iter().enumerate() {
            output.push_str(&format!("{}. {}\n\n", i + 1, err));
        }
        output
    }
}
impl Default for TypeChecker {
    fn default() -> Self { Self::new() }
}
// ═══════════════════════════════════════════════
// PART 10: Tests (Comprehensive)
// ═══════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_type_display() {
        assert_eq!(format!("{}", Type::Int(64)), "عدد64");
        assert_eq!(format!("{}", Type::Bool), "منطقي");
        assert_eq!(format!("{}", Type::Str), "نص");
    }
    #[test]
    fn test_subtype() {
        assert!(Type::Int(32).is_subtype_of(&Type::Int(64)));
        assert!(Type::Int(64).is_subtype_of(&Type::Float(64)));
        assert!(Type::Never.is_subtype_of(&Type::Int(64)));
        assert!(Type::Int(64).is_subtype_of(&Type::Any));
    }
    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new();
        let v1 = TypeVar(0);
        let v2 = TypeVar(1);
        uf.union(v1, v2);
        assert_eq!(uf.find(v1), uf.find(v2));
    }
    #[test]
    fn test_unification() {
        let mut solver = ConstraintSolver::new(TypeEnv::new());
        let loc = SourceLoc::default();
        assert!(solver.unify(&Type::Int(64), &Type::Int(64), &loc).is_ok());
        assert!(solver.unify(&Type::Int(32), &Type::Int(64), &loc).is_err());
        let v = TypeVar(0);
        assert!(solver.unify(&Type::Var(v), &Type::Int(64), &loc).is_ok());
    }
    #[test]
    fn test_literal_inference() {
        let mut alg = AlgorithmW::new();
        assert_eq!(alg.infer_literal("42"), Type::Int(64));
        assert_eq!(alg.infer_literal("3.14"), Type::Float(64));
        assert_eq!(alg.infer_literal("true"), Type::Bool);
        assert_eq!(alg.infer_literal("\"hello\""), Type::Str);
    }
    #[test]
    fn test_binop_inference() {
        let mut alg = AlgorithmW::new();
        let loc = SourceLoc::default();
        assert_eq!(alg.infer_binop("+", Type::Int(64), Type::Int(64), &loc).unwrap(), Type::Int(64));
        assert_eq!(alg.infer_binop("==", Type::Int(64), Type::Int(64), &loc).unwrap(), Type::Bool);
        assert_eq!(alg.infer_binop("&&", Type::Bool, Type::Bool, &loc).unwrap(), Type::Bool);
    }
    #[test]
    fn test_flow_narrowing() {
        let flow = FlowEnv::new();
        let ty = Type::Any;
        assert_eq!(flow.narrow(&ty, "typeof == string"), Type::Str);
    }
    #[test]
    fn test_free_vars() {
        let t = Type::Func(vec![Type::Var(TypeVar(0))], Box::new(Type::Var(TypeVar(1))));
        let vars = t.free_vars();
        assert_eq!(vars.len(), 2);
    }
    // === NEW TESTS for Union, Traits, Generics ===
    #[test]
    fn test_union_types() {
        let u = Type::union(vec![Type::Int(64), Type::Str]);
        match &u {
            Type::Union(ts) => assert_eq!(ts.len(), 2),
            _ => panic!("expected union"),
        }
        assert!(Type::Int(64).is_subtype_of(&u));
        assert!(Type::Str.is_subtype_of(&u));
    }
    #[test]
    fn test_trait_registry() {
        let env = TypeEnv::new();
        assert!(env.implements_trait(&Type::Int(64), &TraitId("عدد".to_string())));
        assert!(env.implements_trait(&Type::Str, &TraitId("قابلة_للطباعة".to_string())));
    }
    #[test]
    fn test_arabic_display() {
        let union = Type::union(vec![Type::Int(64), Type::Str]);
        let display = format!("{}", union);
        assert!(display.contains("عدد64"));
        assert!(display.contains("نص"));
        assert!(display.contains("|"));
    }
}
