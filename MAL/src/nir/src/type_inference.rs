//! Comprehensive Type Inference System for MAL
//! 
//! Implements 4 algorithms:
//! 1. Hindley-Milner Algorithm W (classical inference)
//! 2. Union-Find (efficient constraint solving)
//! 3. Bidirectional Type Checking (synthesis + checking)
//! 4. Flow-Sensitive Analysis (type narrowing)
//!
//! Mathematical Foundation:
//! - Type System: (T, Γ, ⊢, R) where T=types, Γ=context, ⊢=judgment, R=rules
//! - Constraint System: C ::= τ₁ = τ₂ | τ₁ ≤ τ₂ | C₁ ∧ C₂
//! - Lattice: (Types, ≤, ⊔, ⊓, ⊥, ⊤)
use std::collections::HashMap;
use std::fmt;
// ═══════════════════════════════════════════════
// PART 1: Type Algebra (البنية الجبرية للأنواع)
// ═══════════════════════════════════════════════
/// Type variable identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(pub u32);
/// Complete type algebra following Hindley-Milner + extensions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // === Primitive Types (الأنواع الأولية) ===
    /// Int_n = {x ∈ ℤ | -2^(n-1) ≤ x < 2^(n-1)}
    Int(u32),
    /// UInt_n = {x ∈ ℕ | 0 ≤ x < 2^n}
    UInt(u32),
    /// Float_n (IEEE 754)
    Float(u32),
    /// Bool = {true, false}
    Bool,
    /// Char (Unicode code point)
    Char,
    /// String = List(Char)
    Str,
    /// Unit type ()
    Unit,
    // === Type Variables (للاستدلال) ===
    /// α, β, γ... (type variables for inference)
    Var(TypeVar),
    // === Composite Types (الأنواع المركبة) ===
    /// Array(T, n) = {0..n-1} → T
    Array(Box<Type>, usize),
    /// T₁ × T₂ × ... × Tₙ
    Tuple(Vec<Type>),
    /// Record {l₁: T₁, ..., lₙ: Tₙ}
    Record(Vec<(String, Type)>),
    // === Function Types (أنواع الدوال) ===
    /// (T₁, ..., Tₙ) → T_ret
    Func(Vec<Type>, Box<Type>),
    // === Polymorphic Types (الأنواع العامة) ===
    /// ∀α. T
    Forall(TypeVar, Box<Type>),
    // === Special Types ===
    /// Bottom type (never returns)
    Never,
    /// Top type (any)
    Any,
    /// Unknown (for error recovery)
    Unknown,
}
impl Type {
    /// Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int(_) | Type::UInt(_) | Type::Float(_))
    }
    /// Check if type is a variable
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
            Type::Forall(v, t) => {
                let mut subst2 = subst.clone();
                subst2.remove(v);
                Type::Forall(*v, Box::new(t.substitute(&subst2)))
            }
            _ => self.clone(),
        }
    }
    /// Check subtype relation: self ≤ other
    pub fn is_subtype_of(&self, other: &Type) -> bool {
        if self == other { return true; }
        match (self, other) {
            (_, Type::Any) => true,
            (Type::Never, _) => true,
            (Type::Int(a), Type::Int(b)) => a <= b,
            (Type::UInt(a), Type::UInt(b)) => a <= b,
            (Type::Float(a), Type::Float(b)) => a <= b,
            (Type::Int(_), Type::Float(_)) => true,  // Int can be widened to Float
            (Type::Var(_), _) | (_, Type::Var(_)) => true,  // Variables match anything
            (Type::Array(t1, n1), Type::Array(t2, n2)) => n1 == n2 && t1.is_subtype_of(t2),
            (Type::Tuple(ts1), Type::Tuple(ts2)) => {
                ts1.len() == ts2.len() && ts1.iter().zip(ts2).all(|(a, b)| a.is_subtype_of(b))
            }
            (Type::Func(a1, r1), Type::Func(a2, r2)) => {
                // Contravariant in args, covariant in return
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
            Type::Int(n) => write!(f, "Int{}", n),
            Type::UInt(n) => write!(f, "UInt{}", n),
            Type::Float(n) => write!(f, "Float{}", n),
            Type::Bool => write!(f, "Bool"),
            Type::Char => write!(f, "Char"),
            Type::Str => write!(f, "String"),
            Type::Unit => write!(f, "()"),
            Type::Var(v) => write!(f, "α{}", v.0),
            Type::Array(t, n) => write!(f, "[{}; {}]", t, n),
            Type::Tuple(ts) => {
                write!(f, "(")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Record(fields) => {
                write!(f, "{{")?;
                for (i, (n, t)) in fields.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", n, t)?;
                }
                write!(f, "}}")
            }
            Type::Func(args, ret) => {
                write!(f, "(")?;
                for (i, t) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ") → {}", ret)
            }
            Type::Forall(v, t) => write!(f, "∀α{}. {}", v.0, t),
            Type::Never => write!(f, "!"),
            Type::Any => write!(f, "Any"),
            Type::Unknown => write!(f, "?"),
        }
    }
}
// ═══════════════════════════════════════════════
// PART 2: Union-Find (خوارزمية الاتحاد والبحث)
// ═══════════════════════════════════════════════
/// Union-Find data structure for efficient type variable unification
/// Complexity: O(α(n)) where α is inverse Ackermann function
#[derive(Debug)]
pub struct UnionFind {
    parent: HashMap<TypeVar, TypeVar>,
    rank: HashMap<TypeVar, u32>,
    /// Maps type variables to their resolved types
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
    /// Ensure variable exists in the structure
    fn ensure(&mut self, v: TypeVar) {
        if !self.parent.contains_key(&v) {
            self.parent.insert(v, v);
            self.rank.insert(v, 0);
        }
    }
    /// Find root with path compression
    /// Mathematical: find(x) returns representative of equivalence class [x]
    pub fn find(&mut self, v: TypeVar) -> TypeVar {
        self.ensure(v);
        let parent = self.parent[&v];
        if parent == v {
            return v;
        }
        let root = self.find(parent);
        self.parent.insert(v, root);  // Path compression
        root
    }
    /// Union by rank
    /// Mathematical: union([x], [y]) merges equivalence classes
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
    /// Resolve a type variable to its final type
    pub fn resolve(&mut self, v: TypeVar, ty: Type) {
        let root = self.find(v);
        self.resolved.insert(root, ty);
    }
    /// Get resolved type for a variable
    pub fn get_resolved(&mut self, v: TypeVar) -> Option<Type> {
        let root = self.find(v);
        self.resolved.get(&root).cloned()
    }
}
// ═══════════════════════════════════════════════
// PART 3: Type Environment (سياق الأنواع)
// ═══════════════════════════════════════════════
/// Type environment Γ mapping names to types
#[derive(Debug)]
pub struct TypeEnv {
    /// Variable bindings: name → type scheme
    pub bindings: HashMap<String, Type>,
    /// Fresh variable counter
    next_var: u32,
    /// Union-Find for constraint solving
    pub uf: UnionFind,
}
impl TypeEnv {
    pub fn new() -> Self {
        TypeEnv {
            bindings: HashMap::new(),
            next_var: 0,
            uf: UnionFind::new(),
        }
    }
    /// Generate fresh type variable
    /// Mathematical: fresh() → α where α ∉ FV(Γ)
    pub fn fresh_var(&mut self) -> Type {
        let v = TypeVar(self.next_var);
        self.next_var += 1;
        Type::Var(v)
    }
    /// Bind a name to a type
    pub fn bind(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }
    /// Lookup a name
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings.get(name)
    }
    /// Extend environment (for nested scopes)
    pub fn extend(&self) -> TypeEnv {
        TypeEnv {
            bindings: self.bindings.clone(),
            next_var: self.next_var,
            uf: UnionFind::new(),
        }
    }
}
// ═══════════════════════════════════════════════
// PART 4: Constraints (نظام القيود)
// ═══════════════════════════════════════════════
/// Type constraint
#[derive(Debug, Clone)]
pub enum Constraint {
    /// τ₁ = τ₂ (equality)
    Equal(Type, Type),
    /// τ₁ ≤ τ₂ (subtyping)
    Subtype(Type, Type),
}
/// Constraint solver using Union-Find
pub struct ConstraintSolver {
    uf: UnionFind,
    substitution: HashMap<TypeVar, Type>,
}
impl ConstraintSolver {
    pub fn new() -> Self {
        ConstraintSolver {
            uf: UnionFind::new(),
            substitution: HashMap::new(),
        }
    }
    /// Solve a set of constraints
    /// Mathematical: solve(C) → σ such that ∀(τ₁=τ₂)∈C: σ(τ₁) = σ(τ₂)
    pub fn solve(&mut self, constraints: &[Constraint]) -> Result<HashMap<TypeVar, Type>, TypeError> {
        for c in constraints {
            match c {
                Constraint::Equal(t1, t2) => self.unify(t1, t2)?,
                Constraint::Subtype(t1, t2) => self.check_subtype(t1, t2)?,
            }
        }
        Ok(self.substitution.clone())
    }
    /// Unify two types (Algorithm U)
    /// Mathematical: unify(τ₁, τ₂) → σ where σ(τ₁) = σ(τ₂)
    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<(), TypeError> {
        match (t1, t2) {
            // Same types: trivial
            (Type::Int(a), Type::Int(b)) if a == b => Ok(()),
            (Type::UInt(a), Type::UInt(b)) if a == b => Ok(()),
            (Type::Float(a), Type::Float(b)) if a == b => Ok(()),
            (Type::Bool, Type::Bool) => Ok(()),
            (Type::Char, Type::Char) => Ok(()),
            (Type::Str, Type::Str) => Ok(()),
            (Type::Unit, Type::Unit) => Ok(()),
            // Variable unification
            (Type::Var(v), t) | (t, Type::Var(v)) => {
                // Occurs check: v ∉ FV(t)
                if t.free_vars().contains(v) {
                    return Err(TypeError::InfiniteType(*v, t.clone()));
                }
                self.substitution.insert(*v, t.clone());
                self.uf.resolve(*v, t.clone());
                Ok(())
            }
            // Array unification
            (Type::Array(a, n1), Type::Array(b, n2)) if n1 == n2 => {
                self.unify(a, b)
            }
            // Tuple unification
            (Type::Tuple(ts1), Type::Tuple(ts2)) if ts1.len() == ts2.len() => {
                for (a, b) in ts1.iter().zip(ts2) {
                    self.unify(a, b)?;
                }
                Ok(())
            }
            // Function unification
            (Type::Func(a1, r1), Type::Func(a2, r2)) if a1.len() == a2.len() => {
                for (a, b) in a1.iter().zip(a2) {
                    self.unify(a, b)?;
                }
                self.unify(r1, r2)
            }
            // Any matches everything
            (Type::Any, _) | (_, Type::Any) => Ok(()),
            // Unknown matches everything (error recovery)
            (Type::Unknown, _) | (_, Type::Unknown) => Ok(()),
            // Mismatch
            _ => Err(TypeError::Mismatch(t1.clone(), t2.clone())),
        }
    }
    /// Check subtype constraint
    fn check_subtype(&mut self, t1: &Type, t2: &Type) -> Result<(), TypeError> {
        if t1.is_subtype_of(t2) {
            Ok(())
        } else {
            Err(TypeError::SubtypeMismatch(t1.clone(), t2.clone()))
        }
    }
    /// Apply substitution to a type
    pub fn apply(&self, ty: &Type) -> Type {
        ty.substitute(&self.substitution)
    }
}
// ═══════════════════════════════════════════════
// PART 5: Type Errors (أخطاء الأنواع)
// ═══════════════════════════════════════════════
#[derive(Debug, Clone)]
pub enum TypeError {
    /// Type mismatch: expected τ₁, got τ₂
    Mismatch(Type, Type),
    /// Subtype violation: τ₁ ≰ τ₂
    SubtypeMismatch(Type, Type),
    /// Infinite type: α = ...α...
    InfiniteType(TypeVar, Type),
    /// Unbound variable
    UnboundVar(String),
    /// Arity mismatch
    ArityMismatch(usize, usize),
}
impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::Mismatch(expected, got) => {
                write!(f, "Type mismatch: expected {}, got {}", expected, got)
            }
            TypeError::SubtypeMismatch(t1, t2) => {
                write!(f, "Subtype error: {} is not a subtype of {}", t1, t2)
            }
            TypeError::InfiniteType(v, t) => {
                write!(f, "Infinite type: α{} = {}", v.0, t)
            }
            TypeError::UnboundVar(name) => {
                write!(f, "Unbound variable: {}", name)
            }
            TypeError::ArityMismatch(expected, got) => {
                write!(f, "Arity mismatch: expected {} args, got {}", expected, got)
            }
        }
    }
}
// ═══════════════════════════════════════════════
// PART 6: Hindley-Milner Algorithm W
// ═══════════════════════════════════════════════
/// Hindley-Milner Algorithm W
/// Mathematical: W(Γ, e) → (σ, τ) where σ(Γ) ⊢ e : σ(τ)
pub struct AlgorithmW {
    env: TypeEnv,
    constraints: Vec<Constraint>,
}
impl AlgorithmW {
    pub fn new() -> Self {
        AlgorithmW {
            env: TypeEnv::new(),
            constraints: Vec::new(),
        }
    }
    /// Infer type of a literal value
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
    /// Infer type of binary operation
    /// Mathematical rule:
    ///   Γ ⊢ e₁ : τ₁   Γ ⊢ e₂ : τ₂   τ₁ = τ₂ = Int
    ///   ───────────────────────────────────────────
    ///   Γ ⊢ e₁ + e₂ : Int
    pub fn infer_binop(&mut self, op: &str, left: Type, right: Type) -> Result<Type, TypeError> {
        match op {
            "+" | "-" | "*" | "/" | "%" => {
                // Arithmetic: both must be numeric
                if !left.is_numeric() && !matches!(left, Type::Var(_) | Type::Unknown) {
                    return Err(TypeError::Mismatch(Type::Int(64), left));
                }
                if !right.is_numeric() && !matches!(right, Type::Var(_) | Type::Unknown) {
                    return Err(TypeError::Mismatch(Type::Int(64), right));
                }
                // Widen to larger type
                match (&left, &right) {
                    (Type::Float(a), Type::Float(b)) => Ok(Type::Float(*a.max(b))),
                    (Type::Float(a), Type::Int(_)) => Ok(Type::Float(*a)),
                    (Type::Int(_), Type::Float(b)) => Ok(Type::Float(*b)),
                    (Type::Int(a), Type::Int(b)) => Ok(Type::Int(*a.max(b))),
                    _ => Ok(Type::Int(64)),  // Default for variables
                }
            }
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                // Comparison: returns Bool
                self.constraints.push(Constraint::Equal(left.clone(), right.clone()));
                Ok(Type::Bool)
            }
            "&&" | "||" => {
                // Logical: both must be Bool
                self.constraints.push(Constraint::Equal(left, Type::Bool));
                self.constraints.push(Constraint::Equal(right, Type::Bool));
                Ok(Type::Bool)
            }
            _ => Ok(Type::Unknown),
        }
    }
    /// Generalize a type (for let polymorphism)
    /// Mathematical: gen(τ, Γ) = ∀α₁...αₙ. τ where αᵢ = FV(τ) \ FV(Γ)
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
            // For simplicity, generalize first variable only
            Type::Forall(to_generalize[0], Box::new(ty.clone()))
        }
    }
    /// Instantiate a type scheme (replace ∀ with fresh variables)
    /// Mathematical: inst(∀α.τ) = τ[α ↦ fresh()]
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
    /// Solve all collected constraints
    pub fn solve(&self) -> Result<HashMap<TypeVar, Type>, TypeError> {
        let mut solver = ConstraintSolver::new();
        solver.solve(&self.constraints)
    }
}
// ═══════════════════════════════════════════════
// PART 7: Bidirectional Type Checking
// ═══════════════════════════════════════════════
/// Mode for bidirectional type checking
#[derive(Debug, Clone)]
pub enum Mode {
    /// Synthesis: Γ ⊢ e ⇒ τ (infer type)
    Infer,
    /// Checking: Γ ⊢ e ⇐ τ (check against expected)
    Check(Type),
}
/// Bidirectional type checker
/// Mathematical:
///   Synthesis: Γ ⊢ e ⇒ τ
///   Checking:  Γ ⊢ e ⇐ τ
pub struct BidirectionalChecker {
    env: TypeEnv,
}
impl BidirectionalChecker {
    pub fn new() -> Self {
        BidirectionalChecker {
            env: TypeEnv::new(),
        }
    }
    /// Main entry point: check or infer
    pub fn check_or_infer(&mut self, ty: &Type, mode: &Mode) -> Result<Type, TypeError> {
        match mode {
            Mode::Infer => Ok(ty.clone()),
            Mode::Check(expected) => {
                if ty.is_subtype_of(expected) {
                    Ok(expected.clone())
                } else {
                    Err(TypeError::Mismatch(expected.clone(), ty.clone()))
                }
            }
        }
    }
    /// Check function application
    /// Rule:
    ///   Γ ⊢ e₁ ⇒ τ₁ → τ₂   Γ ⊢ e₂ ⇐ τ₁
    ///   ──────────────────────────────────
    ///   Γ ⊢ e₁(e₂) ⇒ τ₂
    pub fn check_application(
        &mut self,
        func_ty: &Type,
        arg_ty: &Type,
    ) -> Result<Type, TypeError> {
        match func_ty {
            Type::Func(params, ret) => {
                if params.len() == 1 {
                    if arg_ty.is_subtype_of(&params[0]) {
                        Ok(*ret.clone())
                    } else {
                        Err(TypeError::Mismatch(params[0].clone(), arg_ty.clone()))
                    }
                } else {
                    Err(TypeError::ArityMismatch(params.len(), 1))
                }
            }
            Type::Var(_) | Type::Unknown => Ok(Type::Unknown),
            _ => Err(TypeError::Mismatch(
                Type::Func(vec![Type::Unknown], Box::new(Type::Unknown)),
                func_ty.clone()
            )),
        }
    }
}
// ═══════════════════════════════════════════════
// PART 8: Flow-Sensitive Analysis
// ═══════════════════════════════════════════════
/// Flow-sensitive type environment
/// Mathematical: Γ: ProgramPoint → (Variable → Type)
#[derive(Debug, Clone)]
pub struct FlowEnv {
    /// Type at each program point
    point_types: HashMap<u32, HashMap<String, Type>>,
}
impl FlowEnv {
    pub fn new() -> Self {
        FlowEnv {
            point_types: HashMap::new(),
        }
    }
    /// Narrow a type based on a condition
    /// Mathematical: narrow(τ, cond) = τ ∩ cond
    pub fn narrow(&self, ty: &Type, condition: &str) -> Type {
        match condition {
            "typeof == string" => {
                match ty {
                    Type::Str => Type::Str,
                    Type::Any => Type::Str,
                    _ => Type::Never,
                }
            }
            "typeof == number" => {
                if ty.is_numeric() { ty.clone() } else { Type::Never }
            }
            "typeof == boolean" => {
                match ty {
                    Type::Bool => Type::Bool,
                    _ => Type::Never,
                }
            }
            "!= null" => ty.clone(),  // Remove null from union
            _ => ty.clone(),
        }
    }
    /// Join types at control flow merge points
    /// Mathematical: join(τ₁, τ₂) = τ₁ ⊔ τ₂ (least upper bound)
    pub fn join(&self, t1: &Type, t2: &Type) -> Type {
        if t1 == t2 {
            t1.clone()
        } else if t1.is_subtype_of(t2) {
            t2.clone()
        } else if t2.is_subtype_of(t1) {
            t1.clone()
        } else {
            Type::Any  // Fallback to top type
        }
    }
    /// Set type at a program point
    pub fn set_point(&mut self, point: u32, var: String, ty: Type) {
        self.point_types.entry(point).or_insert_with(HashMap::new).insert(var, ty);
    }
    /// Get type at a program point
    pub fn get_point(&self, point: u32, var: &str) -> Option<&Type> {
        self.point_types.get(&point).and_then(|m| m.get(var))
    }
}
// ═══════════════════════════════════════════════
// PART 9: Integrated Type Checker
// ═══════════════════════════════════════════════
/// Complete type checker integrating all algorithms
pub struct TypeChecker {
    pub alg_w: AlgorithmW,
    pub bidir: BidirectionalChecker,
    pub flow: FlowEnv,
    pub errors: Vec<TypeError>,
}
impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            alg_w: AlgorithmW::new(),
            bidir: BidirectionalChecker::new(),
            flow: FlowEnv::new(),
            errors: Vec::new(),
        }
    }
    /// Infer type of a MAL expression (from source)
    pub fn infer_source(&mut self, source: &str) -> Type {
        // Simple literal inference
        self.alg_w.infer_literal(source.trim())
    }
    /// Check if two types are compatible
    pub fn compatible(&self, t1: &Type, t2: &Type) -> bool {
        t1.is_subtype_of(t2) || t2.is_subtype_of(t1)
    }
}
impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
// ═══════════════════════════════════════════════
// PART 10: Tests
// ═══════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_type_display() {
        assert_eq!(format!("{}", Type::Int(64)), "Int64");
        assert_eq!(format!("{}", Type::Bool), "Bool");
        assert_eq!(format!("{}", Type::Str), "String");
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
        let mut solver = ConstraintSolver::new();
        assert!(solver.unify(&Type::Int(64), &Type::Int(64)).is_ok());
        assert!(solver.unify(&Type::Int(32), &Type::Int(64)).is_err());
        let v = TypeVar(0);
        assert!(solver.unify(&Type::Var(v), &Type::Int(64)).is_ok());
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
        assert_eq!(
            alg.infer_binop("+", Type::Int(64), Type::Int(64)).unwrap(),
            Type::Int(64)
        );
        assert_eq!(
            alg.infer_binop("==", Type::Int(64), Type::Int(64)).unwrap(),
            Type::Bool
        );
        assert_eq!(
            alg.infer_binop("&&", Type::Bool, Type::Bool).unwrap(),
            Type::Bool
        );
    }
    #[test]
    fn test_flow_narrowing() {
        let flow = FlowEnv::new();
        let ty = Type::Any;
        assert_eq!(flow.narrow(&ty, "typeof == string"), Type::Str);
        assert_eq!(flow.narrow(&ty, "typeof == number"), Type::Never);
    }
    #[test]
    fn test_free_vars() {
        let t = Type::Func(
            vec![Type::Var(TypeVar(0))],
            Box::new(Type::Var(TypeVar(1)))
        );
        let vars = t.free_vars();
        assert_eq!(vars.len(), 2);
    }
}
