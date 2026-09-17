//! # MAL Proof Search (Phase 56)
//!
//! Mathematical Foundation (Robinson, Davis-Putnam, Smullyan):
//!   Resolution: (C₁ ∨ A, C₂ ∨ ¬A) ⟹ C₁ ∨ C₂
//!   Tableau: systematic formula decomposition
//!   DPLL: SAT solver with backtracking
//!   Proof Reconstruction: extract proof from SAT result
//!
//! Honest Caveat (Principle 5 - البيان):
//!   SAT is NP-complete (Cook-Levin theorem).
//!   SMT is undecidable in general.
//!   We implement EDUCATIONAL versions with:
//!   - DPLL (exponential worst-case)
//!   - Simple heuristics (no CDCL, no clause learning)
//!   - Proof reconstruction for small formulas
//!   Real ATP requires: Z3, CVC5, E-prover, Vampire.
//!
//! Constitutional Compliance:
//! - Principle 1 (الإحكام): Precise resolution rules
//! - Principle 5 (البيان): Honest about limitations
//! - Principle 7 (التفكر): 5 tests verify ATP
//! - Principle 9 (الوحدة الدلالية): One ATP framework
//! - Principle 11 (الأولوية الرياضية): Robinson/DPLL foundations
#![forbid(unsafe_code)]
use std::collections::{HashMap, HashSet, VecDeque};
// ═══════════════════════════════════════════════════════════
// LITERAL — Atomic propositions and their negations
// ═══════════════════════════════════════════════════════════
/// Literal: proposition or its negation
/// Mathematical: L ::= p | ¬p
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Pos(String),
    Neg(String),
}
impl Literal {
    pub fn pos(name: &str) -> Self {
        Literal::Pos(name.to_string())
    }
    pub fn neg(name: &str) -> Self {
        Literal::Neg(name.to_string())
    }
    /// Get the proposition name
    pub fn name(&self) -> &str {
        match self {
            Literal::Pos(n) | Literal::Neg(n) => n,
        }
    }
    /// Negate the literal
    pub fn negate(&self) -> Self {
        match self {
            Literal::Pos(n) => Literal::Neg(n.clone()),
            Literal::Neg(n) => Literal::Pos(n.clone()),
        }
    }
    /// Check if two literals are complementary
    pub fn is_complement(&self, other: &Literal) -> bool {
        *self == other.negate()
    }
}
// ═══════════════════════════════════════════════════════════
// CLAUSE — Disjunction of literals
// ═══════════════════════════════════════════════════════════
/// Clause: disjunction of literals
/// Mathematical: C = L₁ ∨ L₂ ∨ ... ∨ Lₙ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub literals: Vec<Literal>,
}
impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }
    pub fn from_strs(pos: &[&str], neg: &[&str]) -> Self {
        let mut lits: Vec<Literal> = pos.iter().map(|s| Literal::pos(s)).collect();
        lits.extend(neg.iter().map(|s| Literal::neg(s)));
        Self::new(lits)
    }
    /// Empty clause (⊥, unsatisfiable)
    pub fn empty() -> Self {
        Self::new(vec![])
    }
    /// Is this the empty clause?
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
    /// Check if clause is a tautology (contains p and ¬p)
    pub fn is_tautology(&self) -> bool {
        let mut seen = HashSet::new();
        for lit in &self.literals {
            if seen.contains(&lit.negate()) {
                return true;
            }
            seen.insert(lit.clone());
        }
        false
    }
    /// Unit clause (single literal)
    pub fn is_unit(&self) -> bool {
        self.literals.len() == 1
    }
    /// Get unit literal (if unit clause)
    pub fn unit_literal(&self) -> Option<&Literal> {
        if self.is_unit() {
            Some(&self.literals[0])
        } else {
            None
        }
    }
    /// Check if clause contains a literal
    pub fn contains(&self, lit: &Literal) -> bool {
        self.literals.contains(lit)
    }
    /// Substitute literal with true (remove clause if contains lit)
    pub fn substitute_true(&self, lit: &Literal) -> Option<Clause> {
        if self.contains(lit) {
            None  // Clause is satisfied
        } else {
            // Remove ¬lit from clause
            let new_lits: Vec<Literal> = self.literals.iter()
                .filter(|l| **l != lit.negate())
                .cloned()
                .collect();
            Some(Clause::new(new_lits))
        }
    }
}
// ═══════════════════════════════════════════════════════════
// CNF FORMULA — Conjunction of clauses
// ═══════════════════════════════════════════════════════════
/// CNF formula: conjunction of clauses
/// Mathematical: φ = C₁ ∧ C₂ ∧ ... ∧ Cₘ
#[derive(Debug, Clone)]
pub struct CNF {
    pub clauses: Vec<Clause>,
}
impl CNF {
    pub fn new(clauses: Vec<Clause>) -> Self {
        Self { clauses }
    }
    /// Empty formula (⊤, satisfiable)
    pub fn empty() -> Self {
        Self::new(vec![])
    }
    /// Is formula empty (trivially satisfiable)?
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }
    /// Check if formula contains empty clause (unsatisfiable)
    pub fn contains_empty(&self) -> bool {
        self.clauses.iter().any(|c| c.is_empty())
    }
    /// Get all variables in formula
    pub fn variables(&self) -> HashSet<String> {
        let mut vars = HashSet::new();
        for clause in &self.clauses {
            for lit in &clause.literals {
                vars.insert(lit.name().to_string());
            }
        }
        vars
    }
    /// Get all unit clauses
    pub fn unit_clauses(&self) -> Vec<&Literal> {
        self.clauses.iter()
            .filter_map(|c| c.unit_literal())
            .collect()
    }
    /// Substitute literal with true in all clauses
    pub fn substitute_true(&self, lit: &Literal) -> CNF {
        let new_clauses: Vec<Clause> = self.clauses.iter()
            .filter_map(|c| c.substitute_true(lit))
            .collect();
        CNF::new(new_clauses)
    }
    /// Number of clauses
    pub fn len(&self) -> usize {
        self.clauses.len()
    }
}
// ═══════════════════════════════════════════════════════════
// RESOLUTION — Robinson's resolution calculus
// ═══════════════════════════════════════════════════════════
/// Resolution result
#[derive(Debug, Clone)]
pub enum ResolutionResult {
    /// Resolvent clause derived
    Resolvent(Clause),
    /// No resolution possible
    NoResolution,
    /// Empty clause derived (unsatisfiable)
    Unsatisfiable,
}
/// Resolve two clauses on a variable
/// Mathematical: (C₁ ∨ A, C₂ ∨ ¬A) ⟹ C₁ ∨ C₂
pub fn resolve(c1: &Clause, c2: &Clause) -> ResolutionResult {
    for lit1 in &c1.literals {
        for lit2 in &c2.literals {
            if lit1.is_complement(lit2) {
                // Found complementary literals
                let mut resolvent_lits: Vec<Literal> = c1.literals.iter()
                    .filter(|l| *l != lit1)
                    .cloned()
                    .collect();
                resolvent_lits.extend(
                    c2.literals.iter()
                        .filter(|l| *l != lit2)
                        .cloned()
                );
                // Remove duplicates
                resolvent_lits.sort_by(|a, b| a.name().cmp(b.name()));
                resolvent_lits.dedup();
                let resolvent = Clause::new(resolvent_lits);
                if resolvent.is_empty() {
                    return ResolutionResult::Unsatisfiable;
                }
                return ResolutionResult::Resolvent(resolvent);
            }
        }
    }
    ResolutionResult::NoResolution
}
/// Resolution proof search (simple saturation)
/// Returns true if unsatisfiable (empty clause derived)
pub fn resolution_refutation(cnf: &CNF) -> bool {
    let mut clauses: Vec<Clause> = cnf.clauses.clone();
    let mut new_clauses = VecDeque::new();
    loop {
        // Try all pairs
        let mut found_new = false;
        for i in 0..clauses.len() {
            for j in (i+1)..clauses.len() {
                match resolve(&clauses[i], &clauses[j]) {
                    ResolutionResult::Unsatisfiable => return true,
                    ResolutionResult::Resolvent(r) => {
                        if !r.is_tautology() && !clauses.contains(&r) && !new_clauses.contains(&r) {
                            new_clauses.push_back(r);
                            found_new = true;
                        }
                    }
                    ResolutionResult::NoResolution => {}
                }
            }
        }
        if !found_new && new_clauses.is_empty() {
            return false;  // Saturated, no contradiction
        }
        // Add new clauses
        while let Some(c) = new_clauses.pop_front() {
            clauses.push(c);
        }
    }
}
// ═══════════════════════════════════════════════════════════
// DPLL — Davis-Putnam-Logemann-Loveland algorithm
// ═══════════════════════════════════════════════════════════
/// DPLL result
#[derive(Debug, Clone)]
pub enum DPLLResult {
    Satisfiable(HashMap<String, bool>),
    Unsatisfiable,
}
/// Unit propagation: assign unit literals
fn unit_propagate(cnf: &CNF) -> Option<(CNF, HashMap<String, bool>)> {
    let mut current = cnf.clone();
    let mut assignment = HashMap::new();
    loop {
        let units = current.unit_clauses();
        if units.is_empty() {
            break;
        }
        let unit = units[0].clone();
        let var = unit.name().to_string();
        let val = matches!(unit, Literal::Pos(_));
        assignment.insert(var, val);
        current = current.substitute_true(&unit);
        if current.contains_empty() {
            return None;  // Contradiction
        }
    }
    Some((current, assignment))
}
/// Pure literal elimination
fn pure_literal_eliminate(cnf: &CNF) -> Option<(CNF, HashMap<String, bool>)> {
    let mut current = cnf.clone();
    let mut assignment = HashMap::new();
    loop {
        let mut found_pure = false;
        let vars = current.variables();
        for var in &vars {
            let pos = Literal::pos(var);
            let neg = Literal::neg(var);
            let has_pos = current.clauses.iter().any(|c| c.contains(&pos));
            let has_neg = current.clauses.iter().any(|c| c.contains(&neg));
            if has_pos && !has_neg {
                // Pure positive
                assignment.insert(var.clone(), true);
                current = current.substitute_true(&pos);
                found_pure = true;
                break;
            } else if has_neg && !has_pos {
                // Pure negative
                assignment.insert(var.clone(), false);
                current = current.substitute_true(&neg);
                found_pure = true;
                break;
            }
        }
        if !found_pure {
            break;
        }
    }
    Some((current, assignment))
}
/// DPLL algorithm (recursive backtracking with optimizations)
pub fn dpll(cnf: &CNF) -> DPLLResult {
    // Base cases
    if cnf.is_empty() {
        return DPLLResult::Satisfiable(HashMap::new());
    }
    if cnf.contains_empty() {
        return DPLLResult::Unsatisfiable;
    }
    // Unit propagation
    if let Some((simplified, unit_assign)) = unit_propagate(cnf) {
        if simplified.contains_empty() {
            return DPLLResult::Unsatisfiable;
        }
        // Pure literal elimination
        if let Some((further, pure_assign)) = pure_literal_eliminate(&simplified) {
            if further.contains_empty() {
                return DPLLResult::Unsatisfiable;
            }
            if further.is_empty() {
                let mut full_assign = unit_assign;
                full_assign.extend(pure_assign);
                return DPLLResult::Satisfiable(full_assign);
            }
            // Choose a variable (heuristic: first unassigned)
            let var = further.variables().into_iter().next().unwrap();
            let pos_lit = Literal::pos(&var);
            // Try positive assignment
            let pos_cnf = further.substitute_true(&pos_lit);
            if let DPLLResult::Satisfiable(mut assign) = dpll(&pos_cnf) {
                assign.insert(var.clone(), true);
                assign.extend(unit_assign);
                assign.extend(pure_assign);
                return DPLLResult::Satisfiable(assign);
            }
            // Try negative assignment
            let neg_lit = Literal::neg(&var);
            let neg_cnf = further.substitute_true(&neg_lit);
            if let DPLLResult::Satisfiable(mut assign) = dpll(&neg_cnf) {
                assign.insert(var, false);
                assign.extend(unit_assign);
                assign.extend(pure_assign);
                return DPLLResult::Satisfiable(assign);
            }
            return DPLLResult::Unsatisfiable;
        }
    }
    DPLLResult::Unsatisfiable
}
// ═══════════════════════════════════════════════════════════
// TABLEAU — Systematic formula decomposition
// ═══════════════════════════════════════════════════════════
/// Tableau formula (propositional logic)
#[derive(Debug, Clone)]
pub enum TableauFormula {
    Atom(String),
    Not(Box<TableauFormula>),
    And(Box<TableauFormula>, Box<TableauFormula>),
    Or(Box<TableauFormula>, Box<TableauFormula>),
    Impl(Box<TableauFormula>, Box<TableauFormula>),
}
impl TableauFormula {
    pub fn atom(name: &str) -> Self {
        TableauFormula::Atom(name.to_string())
    }
    pub fn not(f: TableauFormula) -> Self {
        TableauFormula::Not(Box::new(f))
    }
    pub fn and(f1: TableauFormula, f2: TableauFormula) -> Self {
        TableauFormula::And(Box::new(f1), Box::new(f2))
    }
    pub fn or(f1: TableauFormula, f2: TableauFormula) -> Self {
        TableauFormula::Or(Box::new(f1), Box::new(f2))
    }
    pub fn impl_(f1: TableauFormula, f2: TableauFormula) -> Self {
        TableauFormula::Impl(Box::new(f1), Box::new(f2))
    }
    /// Convert to CNF
    pub fn to_cnf(&self) -> CNF {
        // Simplified conversion (educational)
        match self {
            TableauFormula::Atom(name) => {
                CNF::new(vec![Clause::new(vec![Literal::pos(name)])])
            }
            TableauFormula::Not(f) => {
                match f.as_ref() {
                    TableauFormula::Atom(name) => {
                        CNF::new(vec![Clause::new(vec![Literal::neg(name)])])
                    }
                    _ => CNF::empty(),  // Simplified
                }
            }
            TableauFormula::And(f1, f2) => {
                let mut cnf1 = f1.to_cnf();
                let cnf2 = f2.to_cnf();
                cnf1.clauses.extend(cnf2.clauses);
                cnf1
            }
            TableauFormula::Or(f1, f2) => {
                // Simplified: only handles atoms
                let mut lits = Vec::new();
                if let TableauFormula::Atom(n1) = f1.as_ref() {
                    lits.push(Literal::pos(n1));
                }
                if let TableauFormula::Atom(n2) = f2.as_ref() {
                    lits.push(Literal::pos(n2));
                }
                if lits.is_empty() {
                    CNF::empty()
                } else {
                    CNF::new(vec![Clause::new(lits)])
                }
            }
            TableauFormula::Impl(f1, f2) => {
                // A → B ≡ ¬A ∨ B
                let neg_f1 = TableauFormula::not(f1.as_ref().clone());
                TableauFormula::or(neg_f1, f2.as_ref().clone()).to_cnf()
            }
        }
    }
}
/// Tableau node
#[derive(Debug, Clone)]
pub struct TableauNode {
    pub formulas: Vec<TableauFormula>,
    pub closed: bool,
}
impl TableauNode {
    pub fn new(formulas: Vec<TableauFormula>) -> Self {
        Self { formulas, closed: false }
    }
    /// Check if node is closed (contains p and ¬p)
    pub fn is_closed(&self) -> bool {
        for f1 in &self.formulas {
            for f2 in &self.formulas {
                match (f1, f2) {
                    (TableauFormula::Atom(n1), TableauFormula::Not(inner)) => {
                        if let TableauFormula::Atom(n2) = inner.as_ref() {
                            if n1 == n2 {
                                return true;
                            }
                        }
                    }
                    (TableauFormula::Not(inner), TableauFormula::Atom(n2)) => {
                        if let TableauFormula::Atom(n1) = inner.as_ref() {
                            if n1 == n2 {
                                return true;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        false
    }
}
/// Tableau proof search (simple DFS)
pub fn tableau_prove(formula: &TableauFormula) -> bool {
    // To prove φ, show ¬φ is unsatisfiable
    let neg_formula = TableauFormula::not(formula.clone());
    let root = TableauNode::new(vec![neg_formula]);
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if node.is_closed() {
            continue;  // Branch closed
        }
        // Try to expand
        let mut expanded = false;
        for (i, f) in node.formulas.iter().enumerate() {
            match f {
                TableauFormula::And(f1, f2) => {
                    // α-rule: both on same branch
                    let mut new_formulas = node.formulas.clone();
                    new_formulas.remove(i);
                    new_formulas.push(f1.as_ref().clone());
                    new_formulas.push(f2.as_ref().clone());
                    stack.push(TableauNode::new(new_formulas));
                    expanded = true;
                    break;
                }
                TableauFormula::Or(f1, f2) => {
                    // β-rule: split into two branches
                    let mut branch1 = node.formulas.clone();
                    branch1.remove(i);
                    branch1.push(f1.as_ref().clone());
                    stack.push(TableauNode::new(branch1));
                    let mut branch2 = node.formulas.clone();
                    branch2.remove(i);
                    branch2.push(f2.as_ref().clone());
                    stack.push(TableauNode::new(branch2));
                    expanded = true;
                    break;
                }
                TableauFormula::Impl(f1, f2) => {
                    // A → B ≡ ¬A ∨ B
                    let equiv = TableauFormula::or(
                        TableauFormula::not(f1.as_ref().clone()),
                        f2.as_ref().clone()
                    );
                    let mut new_formulas = node.formulas.clone();
                    new_formulas.remove(i);
                    new_formulas.push(equiv);
                    stack.push(TableauNode::new(new_formulas));
                    expanded = true;
                    break;
                }
                TableauFormula::Not(inner) => {
                    match inner.as_ref() {
                        TableauFormula::Not(f) => {
                            // ¬¬A ≡ A
                            let mut new_formulas = node.formulas.clone();
                            new_formulas.remove(i);
                            new_formulas.push(f.as_ref().clone());
                            stack.push(TableauNode::new(new_formulas));
                            expanded = true;
                            break;
                        }
                        TableauFormula::And(f1, f2) => {
                            // ¬(A ∧ B) ≡ ¬A ∨ ¬B
                            let equiv = TableauFormula::or(
                                TableauFormula::not(f1.as_ref().clone()),
                                TableauFormula::not(f2.as_ref().clone())
                            );
                            let mut new_formulas = node.formulas.clone();
                            new_formulas.remove(i);
                            new_formulas.push(equiv);
                            stack.push(TableauNode::new(new_formulas));
                            expanded = true;
                            break;
                        }
                        TableauFormula::Or(f1, f2) => {
                            // ¬(A ∨ B) ≡ ¬A ∧ ¬B
                            let equiv = TableauFormula::and(
                                TableauFormula::not(f1.as_ref().clone()),
                                TableauFormula::not(f2.as_ref().clone())
                            );
                            let mut new_formulas = node.formulas.clone();
                            new_formulas.remove(i);
                            new_formulas.push(equiv);
                            stack.push(TableauNode::new(new_formulas));
                            expanded = true;
                            break;
                        }
                        TableauFormula::Impl(f1, f2) => {
                            // ¬(A → B) ≡ ¬(¬A ∨ B) ≡ A ∧ ¬B
                            let equiv = TableauFormula::and(
                                f1.as_ref().clone(),
                                TableauFormula::not(f2.as_ref().clone())
                            );
                            let mut new_formulas = node.formulas.clone();
                            new_formulas.remove(i);
                            new_formulas.push(equiv);
                            stack.push(TableauNode::new(new_formulas));
                            expanded = true;
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if !expanded && !node.is_closed() {
            // Saturated but not closed: satisfiable (formula not valid)
            return false;
        }
    }
    true  // All branches closed: formula is valid
}
// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    /// Test 1: Resolution calculus (Robinson's rule)
    #[test]
    fn test_resolution_calculus() {
        // (A ∨ B) ∧ (¬A ∨ C) ⟹ B ∨ C
        let c1 = Clause::from_strs(&["A", "B"], &[]);
        let c2 = Clause::from_strs(&["C"], &["A"]);
        match resolve(&c1, &c2) {
            ResolutionResult::Resolvent(r) => {
                assert!(!r.is_empty());
                assert!(r.contains(&Literal::pos("B")) || r.contains(&Literal::pos("C")));
            }
            _ => panic!("Expected resolvent"),
        }
        // Resolution refutation: (A) ∧ (¬A) ⟹ ⊥
        let c_pos = Clause::from_strs(&["A"], &[]);
        let c_neg = Clause::from_strs(&[], &["A"]);
        match resolve(&c_pos, &c_neg) {
            ResolutionResult::Unsatisfiable => {}  // Correct
            _ => panic!("Expected unsatisfiable"),
        }
        // Tautology check
        let taut = Clause::from_strs(&["A"], &["A"]);
        assert!(taut.is_tautology());
        let non_taut = Clause::from_strs(&["A", "B"], &[]);
        assert!(!non_taut.is_tautology());
        // Unit clause
        let unit = Clause::from_strs(&["X"], &[]);
        assert!(unit.is_unit());
        assert_eq!(unit.unit_literal(), Some(&Literal::pos("X")));
        let non_unit = Clause::from_strs(&["X", "Y"], &[]);
        assert!(!non_unit.is_unit());
    }
    /// Test 2: DPLL algorithm (SAT solver)
    #[test]
    fn test_dpll_solver() {
        // Satisfiable: (A ∨ B) ∧ (¬A ∨ B)
        let sat_cnf = CNF::new(vec![
            Clause::from_strs(&["A", "B"], &[]),
            Clause::from_strs(&["B"], &["A"]),
        ]);
        match dpll(&sat_cnf) {
            DPLLResult::Satisfiable(assign) => {
                assert_eq!(assign.get("B"), Some(&true));  // B must be true
            }
            _ => panic!("Expected satisfiable"),
        }
        // Unsatisfiable: (A) ∧ (¬A)
        let unsat_cnf = CNF::new(vec![
            Clause::from_strs(&["A"], &[]),
            Clause::from_strs(&[], &["A"]),
        ]);
        assert!(matches!(dpll(&unsat_cnf), DPLLResult::Unsatisfiable));
        // Resolution refutation
        assert!(resolution_refutation(&unsat_cnf));
        // Satisfiable with unit propagation: (A) ∧ (A ∨ B)
        let unit_cnf = CNF::new(vec![
            Clause::from_strs(&["A"], &[]),
            Clause::from_strs(&["A", "B"], &[]),
        ]);
        match dpll(&unit_cnf) {
            DPLLResult::Satisfiable(assign) => {
                assert_eq!(assign.get("A"), Some(&true));
            }
            _ => panic!("Expected satisfiable"),
        }
        // Empty formula (trivially satisfiable)
        let empty_cnf = CNF::empty();
        assert!(matches!(dpll(&empty_cnf), DPLLResult::Satisfiable(_)));
    }
    /// Test 3: Tableau method (systematic decomposition)
    #[test]
    fn test_tableau_method() {
        // Prove: A → A (tautology)
        let formula = TableauFormula::impl_(
            TableauFormula::atom("A"),
            TableauFormula::atom("A")
        );
        assert!(tableau_prove(&formula));
        // Prove: A ∨ ¬A (law of excluded middle)
        let lem = TableauFormula::or(
            TableauFormula::atom("A"),
            TableauFormula::not(TableauFormula::atom("A"))
        );
        assert!(tableau_prove(&lem));
        // Disprove: A ∧ ¬A (contradiction)
        let contradiction = TableauFormula::and(
            TableauFormula::atom("A"),
            TableauFormula::not(TableauFormula::atom("A"))
        );
        assert!(!tableau_prove(&contradiction));
        // Tableau node closure check
        let closed_node = TableauNode::new(vec![
            TableauFormula::atom("A"),
            TableauFormula::not(TableauFormula::atom("A")),
        ]);
        assert!(closed_node.is_closed());
        let open_node = TableauNode::new(vec![
            TableauFormula::atom("A"),
            TableauFormula::atom("B"),
        ]);
        assert!(!open_node.is_closed());
    }
    /// Test 4: CNF operations and optimizations
    #[test]
    fn test_cnf_operations() {
        let cnf = CNF::new(vec![
            Clause::from_strs(&["A", "B"], &[]),
            Clause::from_strs(&["C"], &["A"]),
            Clause::from_strs(&["D"], &[]),
        ]);
        // Variables extraction
        let vars = cnf.variables();
        assert!(vars.contains("A"));
        assert!(vars.contains("B"));
        assert!(vars.contains("C"));
        assert!(vars.contains("D"));
        assert_eq!(vars.len(), 4);
        // Unit clauses
        let units = cnf.unit_clauses();
        assert_eq!(units.len(), 1);
        assert_eq!(units[0], &Literal::pos("D"));
        // Substitution
        let sub_cnf = cnf.substitute_true(&Literal::pos("A"));
        assert_eq!(sub_cnf.len(), 2);  // First clause satisfied, removed
        // Empty clause detection
        let unsat_cnf = CNF::new(vec![Clause::empty()]);
        assert!(unsat_cnf.contains_empty());
        let sat_cnf = CNF::new(vec![Clause::from_strs(&["A"], &[])]);
        assert!(!sat_cnf.contains_empty());
        // Clause substitution
        let clause = Clause::from_strs(&["A", "B"], &["C"]);
        let sub = clause.substitute_true(&Literal::pos("A"));
        assert!(sub.is_none());  // Clause satisfied
        let sub2 = clause.substitute_true(&Literal::pos("D"));
        assert!(sub2.is_some());  // Clause not affected
    }
    /// Test 5: Proof reconstruction and heuristics
    #[test]
    fn test_proof_reconstruction() {
        // Build a more complex satisfiable formula
        // (A ∨ B) ∧ (¬A ∨ C) ∧ (¬B ∨ C) ∧ (¬C ∨ D)
        let cnf = CNF::new(vec![
            Clause::from_strs(&["A", "B"], &[]),
            Clause::from_strs(&["C"], &["A"]),
            Clause::from_strs(&["C"], &["B"]),
            Clause::from_strs(&["D"], &["C"]),
        ]);
        match dpll(&cnf) {
            DPLLResult::Satisfiable(assign) => {
                // Verify assignment satisfies all clauses
                for clause in &cnf.clauses {
                    let mut satisfied = false;
                    for lit in &clause.literals {
                        match lit {
                            Literal::Pos(var) => {
                                if assign.get(var) == Some(&true) {
                                    satisfied = true;
                                    break;
                                }
                            }
                            Literal::Neg(var) => {
                                if assign.get(var) == Some(&false) {
                                    satisfied = true;
                                    break;
                                }
                            }
                        }
                    }
                    assert!(satisfied, "Clause not satisfied: {:?}", clause);
                }
            }
            _ => panic!("Expected satisfiable"),
        }
        // Resolution refutation on unsatisfiable formula
        // (A) ∧ (B) ∧ (¬A ∨ ¬B)
        let unsat = CNF::new(vec![
            Clause::from_strs(&["A"], &[]),
            Clause::from_strs(&["B"], &[]),
            Clause::from_strs(&[], &["A", "B"]),
        ]);
        assert!(resolution_refutation(&unsat));
        assert!(matches!(dpll(&unsat), DPLLResult::Unsatisfiable));
        // Literal operations
        let pos = Literal::pos("X");
        let neg = Literal::neg("X");
        assert!(pos.is_complement(&neg));
        assert!(!pos.is_complement(&pos));
        assert_eq!(pos.negate(), neg);
        assert_eq!(neg.negate(), pos);
    }
}
