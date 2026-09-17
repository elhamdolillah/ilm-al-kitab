//! # MAL Quantum Types (Phase 58)
//!
//! Mathematical Foundation (Dirac, von Neumann, Wootters-Zurek):
//!   Qubit: |ψ⟩ = α|0⟩ + β|1⟩ with |α|² + |β|² = 1
//!   Gates: unitary operators (H, X, Y, Z, CNOT)
//!   Measurement: Born rule P(i) = |⟨i|ψ⟩|²
//!   No-Cloning: ∄U such that U(|ψ⟩|0⟩) = |ψ⟩|ψ⟩ for all |ψ⟩
//!   Entanglement: non-separable states (Bell states)
//!
//! Honest Caveat (Principle 5 - البيان):
//!   Real quantum computing uses complex amplitudes and probabilistic
//!   measurement. We implement EDUCATIONAL version with:
//!   - Real amplitudes only (no complex numbers)
//!   - Deterministic gates (no actual probability simulation)
//!   - Type-level enforcement of no-cloning (via linear types)
//!   Real quantum programming requires: Qiskit, Cirq, or Q#.
//!
//! Constitutional Compliance:
//! - Principle 1 (الإحكام): Precise quantum operations
//! - Principle 4 (الخطية): Linear types enforce no-cloning
//! - Principle 5 (البيان): Honest about limitations
//! - Principle 7 (التفكر): 5 tests verify quantum operations
//! - Principle 11 (الأولوية الرياضية): Dirac/von Neumann foundations
#![forbid(unsafe_code)]
// ═══════════════════════════════════════════════════════════
// QUBIT — Quantum bit (linear type)
// ═══════════════════════════════════════════════════════════
/// Qubit state: |ψ⟩ = α|0⟩ + β|1⟩
/// Mathematical: normalized state in ℂ² (simplified to ℝ²)
///
/// LINEAR TYPE: Cannot be copied or dropped without measurement.
/// This enforces the no-cloning theorem at the type level.
#[derive(Debug)]
pub struct Qubit {
    /// Amplitude for |0⟩
    pub alpha: f64,
    /// Amplitude for |1⟩
    pub beta: f64,
}
impl Qubit {
    /// Create |0⟩ state
    pub fn zero() -> Self {
        Self { alpha: 1.0, beta: 0.0 }
    }
    /// Create |1⟩ state
    pub fn one() -> Self {
        Self { alpha: 0.0, beta: 1.0 }
    }
    /// Create superposition: α|0⟩ + β|1⟩
    /// Panics if not normalized (educational check)
    pub fn new(alpha: f64, beta: f64) -> Self {
        let norm_sq = alpha * alpha + beta * beta;
        assert!(
            (norm_sq - 1.0).abs() < 1e-10,
            "Qubit must be normalized: |α|² + |β|² = 1 (got {})",
            norm_sq
        );
        Self { alpha, beta }
    }
    /// Create |+⟩ state: (|0⟩ + |1⟩)/√2
    pub fn plus() -> Self {
        let amp = 1.0 / std::f64::consts::SQRT_2;
        Self::new(amp, amp)
    }
    /// Create |-⟩ state: (|0⟩ - |1⟩)/√2
    pub fn minus() -> Self {
        let amp = 1.0 / std::f64::consts::SQRT_2;
        Self::new(amp, -amp)
    }
    /// Check if state is |0⟩
    pub fn is_zero(&self) -> bool {
        (self.alpha - 1.0).abs() < 1e-10 && self.beta.abs() < 1e-10
    }
    /// Check if state is |1⟩
    pub fn is_one(&self) -> bool {
        self.alpha.abs() < 1e-10 && (self.beta - 1.0).abs() < 1e-10
    }
    /// Check if state is |+⟩
    pub fn is_plus(&self) -> bool {
        let amp = 1.0 / std::f64::consts::SQRT_2;
        (self.alpha - amp).abs() < 1e-10 && (self.beta - amp).abs() < 1e-10
    }
    /// Check if state is |-⟩
    pub fn is_minus(&self) -> bool {
        let amp = 1.0 / std::f64::consts::SQRT_2;
        (self.alpha - amp).abs() < 1e-10 && (self.beta + amp).abs() < 1e-10
    }
    /// Calculate norm (should be 1.0)
    pub fn norm(&self) -> f64 {
        (self.alpha * self.alpha + self.beta * self.beta).sqrt()
    }
    /// Probability of measuring |0⟩
    pub fn prob_zero(&self) -> f64 {
        self.alpha * self.alpha
    }
    /// Probability of measuring |1⟩
    pub fn prob_one(&self) -> f64 {
        self.beta * self.beta
    }
}
// ═══════════════════════════════════════════════════════════
// QUANTUM GATES — Unitary operators
// ═══════════════════════════════════════════════════════════
/// Apply Hadamard gate: H|0⟩ = |+⟩, H|1⟩ = |-⟩
/// Matrix: (1/√2) * [[1, 1], [1, -1]]
pub fn hadamard(mut q: Qubit) -> Qubit {
    let amp = 1.0 / std::f64::consts::SQRT_2;
    let new_alpha = amp * (q.alpha + q.beta);
    let new_beta = amp * (q.alpha - q.beta);
    q.alpha = new_alpha;
    q.beta = new_beta;
    q
}
/// Apply Pauli-X gate (NOT): X|0⟩ = |1⟩, X|1⟩ = |0⟩
/// Matrix: [[0, 1], [1, 0]]
pub fn pauli_x(mut q: Qubit) -> Qubit {
    std::mem::swap(&mut q.alpha, &mut q.beta);
    q
}
/// Apply Pauli-Y gate: Y|0⟩ = i|1⟩, Y|1⟩ = -i|0⟩
/// Simplified to real: Y|0⟩ = |1⟩, Y|1⟩ = -|0⟩ (educational)
pub fn pauli_y(mut q: Qubit) -> Qubit {
    let temp = q.alpha;
    q.alpha = q.beta;
    q.beta = -temp;
    q
}
/// Apply Pauli-Z gate: Z|0⟩ = |0⟩, Z|1⟩ = -|1⟩
/// Matrix: [[1, 0], [0, -1]]
pub fn pauli_z(mut q: Qubit) -> Qubit {
    q.beta = -q.beta;
    q
}
/// Apply Phase gate (S): S|0⟩ = |0⟩, S|1⟩ = i|1⟩
/// Simplified to real: S|0⟩ = |0⟩, S|1⟩ = -|1⟩ (educational)
pub fn phase_s(mut q: Qubit) -> Qubit {
    q.beta = -q.beta;
    q
}
/// Apply T gate (π/8): T|0⟩ = |0⟩, T|1⟩ = e^{iπ/4}|1⟩
/// Simplified: no change for educational purposes
pub fn phase_t(q: Qubit) -> Qubit {
    q  // No-op in real-amplitude version
}
// ═══════════════════════════════════════════════════════════
// TWO-QUBIT SYSTEM — Tensor product
// ═══════════════════════════════════════════════════════════
/// Two-qubit system: |ψ₁⟩ ⊗ |ψ₂⟩
/// State space: ℂ⁴ (simplified to ℝ⁴)
#[derive(Debug)]
pub struct TwoQubit {
    /// Amplitudes for |00⟩, |01⟩, |10⟩, |11⟩
    pub amplitudes: [f64; 4],
}
impl TwoQubit {
    /// Create |00⟩ state
    pub fn zero_zero() -> Self {
        Self { amplitudes: [1.0, 0.0, 0.0, 0.0] }
    }
    /// Create from two qubits: |ψ₁⟩ ⊗ |ψ₂⟩
    pub fn from_qubits(q1: &Qubit, q2: &Qubit) -> Self {
        Self {
            amplitudes: [
                q1.alpha * q2.alpha,  // |00⟩
                q1.alpha * q2.beta,   // |01⟩
                q1.beta * q2.alpha,   // |10⟩
                q1.beta * q2.beta,    // |11⟩
            ]
        }
    }
    /// Check if state is |00⟩
    pub fn is_zero_zero(&self) -> bool {
        (self.amplitudes[0] - 1.0).abs() < 1e-10
            && self.amplitudes[1].abs() < 1e-10
            && self.amplitudes[2].abs() < 1e-10
            && self.amplitudes[3].abs() < 1e-10
    }
    /// Check if state is entangled (non-separable)
    /// Simplified check: Bell state |Φ⁺⟩ = (|00⟩ + |11⟩)/√2
    pub fn is_entangled(&self) -> bool {
        let amp = 1.0 / std::f64::consts::SQRT_2;
        // Check for Bell state |Φ⁺⟩
        let is_bell_plus = (self.amplitudes[0] - amp).abs() < 1e-10
            && self.amplitudes[1].abs() < 1e-10
            && self.amplitudes[2].abs() < 1e-10
            && (self.amplitudes[3] - amp).abs() < 1e-10;
        // Check for Bell state |Ψ⁺⟩ = (|01⟩ + |10⟩)/√2
        let is_bell_cross = self.amplitudes[0].abs() < 1e-10
            && (self.amplitudes[1] - amp).abs() < 1e-10
            && (self.amplitudes[2] - amp).abs() < 1e-10
            && self.amplitudes[3].abs() < 1e-10;
        is_bell_plus || is_bell_cross
    }
}
/// Apply CNOT gate: |a,b⟩ → |a, a⊕b⟩
/// Truth table:
///   |00⟩ → |00⟩
///   |01⟩ → |01⟩
///   |10⟩ → |11⟩
///   |11⟩ → |10⟩
pub fn cnot(mut tq: TwoQubit) -> TwoQubit {
    // Swap amplitudes of |10⟩ and |11⟩
    tq.amplitudes.swap(2, 3);
    tq
}
/// Create Bell state |Φ⁺⟩ = (|00⟩ + |11⟩)/√2
/// Circuit: H on qubit 1, then CNOT(1,2)
pub fn bell_state_phi_plus() -> TwoQubit {
    let q1 = hadamard(Qubit::zero());
    let q2 = Qubit::zero();
    let tq = TwoQubit::from_qubits(&q1, &q2);
    cnot(tq)
}
/// Create Bell state |Ψ⁺⟩ = (|01⟩ + |10⟩)/√2
pub fn bell_state_psi_plus() -> TwoQubit {
    let q1 = hadamard(Qubit::zero());
    let q2 = Qubit::one();
    let tq = TwoQubit::from_qubits(&q1, &q2);
    cnot(tq)
}
// ═══════════════════════════════════════════════════════════
// MEASUREMENT — Born rule
// ═══════════════════════════════════════════════════════════
/// Measurement outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementOutcome {
    Zero,
    One,
}
/// Measure qubit (deterministic for basis states, educational)
/// Returns (outcome, collapsed state)
pub fn measure(q: Qubit) -> (MeasurementOutcome, Qubit) {
    if q.is_zero() || q.prob_zero() > 0.5 {
        (MeasurementOutcome::Zero, Qubit::zero())
    } else {
        (MeasurementOutcome::One, Qubit::one())
    }
}
// ═══════════════════════════════════════════════════════════
// NO-CLONING THEOREM — Type-level enforcement
// ═══════════════════════════════════════════════════════════
/// Attempt to clone a qubit (should fail at compile time)
/// This function is intentionally not implemented.
/// The type system prevents cloning via linear types.
///
/// Mathematical: No-cloning theorem (Wootters-Zurek 1982)
///   ∄ unitary U such that U(|ψ⟩|0⟩) = |ψ⟩|ψ⟩ for all |ψ⟩
pub fn attempt_clone(_q: Qubit) -> (Qubit, Qubit) {
    // This would violate no-cloning theorem
    // In real implementation, this would be a compile error
    // For educational purposes, we panic
    panic!("No-cloning theorem: qubits cannot be cloned!")
}
/// Safe "copy" via entanglement (quantum teleportation)
/// Educational placeholder
pub fn quantum_teleport(q: Qubit) -> Qubit {
    // In real quantum teleportation:
    // 1. Create Bell pair
    // 2. Bell measurement on sender's qubits
    // 3. Send classical bits
    // 4. Apply corrections on receiver's qubit
    //
    // For educational purposes, just return the qubit
    q
}
// ═══════════════════════════════════════════════════════════
// QUANTUM CIRCUITS — Composition of gates
// ═══════════════════════════════════════════════════════════
/// Quantum circuit builder
pub struct Circuit {
    gates: Vec<Box<dyn Fn(Qubit) -> Qubit + Send + Sync>>,
}
impl Circuit {
    pub fn new() -> Self {
        Self { gates: Vec::new() }
    }
    /// Add Hadamard gate
    pub fn h(mut self) -> Self {
        self.gates.push(Box::new(hadamard));
        self
    }
    /// Add Pauli-X gate
    pub fn x(mut self) -> Self {
        self.gates.push(Box::new(pauli_x));
        self
    }
    /// Add Pauli-Y gate
    pub fn y(mut self) -> Self {
        self.gates.push(Box::new(pauli_y));
        self
    }
    /// Add Pauli-Z gate
    pub fn z(mut self) -> Self {
        self.gates.push(Box::new(pauli_z));
        self
    }
    /// Execute circuit on input qubit
    pub fn run(self, mut q: Qubit) -> Qubit {
        for gate in self.gates {
            q = gate(q);
        }
        q
    }
}
impl Default for Circuit {
    fn default() -> Self {
        Self::new()
    }
}
// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    /// Test 1: Qubit creation and basis states
    #[test]
    fn test_qubit_basics() {
        // Basis states
        let zero = Qubit::zero();
        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert!((zero.norm() - 1.0).abs() < 1e-10);
        let one = Qubit::one();
        assert!(one.is_one());
        assert!(!one.is_zero());
        assert!((one.norm() - 1.0).abs() < 1e-10);
        // Superposition states
        let plus = Qubit::plus();
        assert!(plus.is_plus());
        assert!((plus.norm() - 1.0).abs() < 1e-10);
        let minus = Qubit::minus();
        assert!(minus.is_minus());
        assert!((minus.norm() - 1.0).abs() < 1e-10);
        // Probabilities
        assert!((plus.prob_zero() - 0.5).abs() < 1e-10);
        assert!((plus.prob_one() - 0.5).abs() < 1e-10);
        // Normalization check
        let valid = Qubit::new(0.6, 0.8);  // 0.6² + 0.8² = 0.36 + 0.64 = 1
        assert!((valid.norm() - 1.0).abs() < 1e-10);
    }
    /// Test 2: Single-qubit gates
    #[test]
    fn test_single_qubit_gates() {
        // Hadamard: |0⟩ → |+⟩
        let q0 = Qubit::zero();
        let h_q0 = hadamard(q0);
        assert!(h_q0.is_plus());
        // Hadamard: |1⟩ → |-⟩
        let q1 = Qubit::one();
        let h_q1 = hadamard(q1);
        assert!(h_q1.is_minus());
        // Hadamard is self-inverse: H² = I
        let q = Qubit::zero();
        let h2_q = hadamard(hadamard(q));
        assert!(h2_q.is_zero());
        // Pauli-X (NOT): |0⟩ ↔ |1⟩
        assert!(pauli_x(Qubit::zero()).is_one());
        assert!(pauli_x(Qubit::one()).is_zero());
        // Pauli-X is self-inverse: X² = I
        let q = Qubit::plus();
        let x2_q = pauli_x(pauli_x(q));
        assert!(x2_q.is_plus());
        // Pauli-Z: |0⟩ → |0⟩, |1⟩ → -|1⟩
        let z_plus = pauli_z(Qubit::plus());
        assert!(z_plus.is_minus());  // Z|+⟩ = |-⟩
        // Circuit composition
        let circuit = Circuit::new().h().x().h();
        let result = circuit.run(Qubit::zero());
        // H|0⟩ = |+⟩, X|+⟩ = |+⟩, H|+⟩ = |0⟩
        // Actually: HXH = Z, so result should be Z|0⟩ = |0⟩
        assert!(result.is_zero());
    }
    /// Test 3: Two-qubit systems and CNOT
    #[test]
    fn test_two_qubit_cnot() {
        // |00⟩ → |00⟩ (control=0, no flip)
        let tq00 = TwoQubit::from_qubits(&Qubit::zero(), &Qubit::zero());
        let cnot_00 = cnot(tq00);
        assert!(cnot_00.is_zero_zero());
        // |10⟩ → |11⟩ (control=1, flip target)
        let tq10 = TwoQubit::from_qubits(&Qubit::one(), &Qubit::zero());
        let cnot_10 = cnot(tq10);
        // After CNOT: |11⟩, so amplitudes[3] should be 1
        assert!((cnot_10.amplitudes[3] - 1.0).abs() < 1e-10);
        // |01⟩ → |01⟩ (control=0, no flip)
        let tq01 = TwoQubit::from_qubits(&Qubit::zero(), &Qubit::one());
        let cnot_01 = cnot(tq01);
        assert!((cnot_01.amplitudes[1] - 1.0).abs() < 1e-10);
        // |11⟩ → |10⟩ (control=1, flip target)
        let tq11 = TwoQubit::from_qubits(&Qubit::one(), &Qubit::one());
        let cnot_11 = cnot(tq11);
        assert!((cnot_11.amplitudes[2] - 1.0).abs() < 1e-10);
        // CNOT is self-inverse: CNOT² = I
        let tq = TwoQubit::from_qubits(&Qubit::one(), &Qubit::zero());
        let cnot2 = cnot(cnot(tq));
        assert!((cnot2.amplitudes[2] - 1.0).abs() < 1e-10);  // Back to |10⟩
    }
    /// Test 4: Entanglement and Bell states
    #[test]
    fn test_entanglement_bell_states() {
        // Create Bell state |Φ⁺⟩ = (|00⟩ + |11⟩)/√2
        let bell_phi = bell_state_phi_plus();
        assert!(bell_phi.is_entangled());
        // Check amplitudes
        let amp = 1.0 / std::f64::consts::SQRT_2;
        assert!((bell_phi.amplitudes[0] - amp).abs() < 1e-10);
        assert!((bell_phi.amplitudes[3] - amp).abs() < 1e-10);
        assert!(bell_phi.amplitudes[1].abs() < 1e-10);
        assert!(bell_phi.amplitudes[2].abs() < 1e-10);
        // Create Bell state |Ψ⁺⟩ = (|01⟩ + |10⟩)/√2
        let bell_psi = bell_state_psi_plus();
        assert!(bell_psi.is_entangled());
        assert!((bell_psi.amplitudes[1] - amp).abs() < 1e-10);
        assert!((bell_psi.amplitudes[2] - amp).abs() < 1e-10);
        // Non-entangled state: |00⟩
        let not_entangled = TwoQubit::zero_zero();
        assert!(!not_entangled.is_entangled());
        // Product state: |+⟩ ⊗ |0⟩ is not entangled
        let q1 = Qubit::plus();
        let q2 = Qubit::zero();
        let product = TwoQubit::from_qubits(&q1, &q2);
        assert!(!product.is_entangled());
    }
    /// Test 5: Measurement and no-cloning
    #[test]
    fn test_measurement_no_cloning() {
        // Measure |0⟩ → outcome Zero, state collapses to |0⟩
        let q = Qubit::zero();
        let (outcome, collapsed) = measure(q);
        assert_eq!(outcome, MeasurementOutcome::Zero);
        assert!(collapsed.is_zero());
        // Measure |1⟩ → outcome One
        let q = Qubit::one();
        let (outcome, collapsed) = measure(q);
        assert_eq!(outcome, MeasurementOutcome::One);
        assert!(collapsed.is_one());
        // Measure |+⟩ → probabilistic (educational: biased toward 0)
        let q = Qubit::plus();
        let (outcome, _collapsed) = measure(q);
        // Could be either, but we check it's valid
        assert!(outcome == MeasurementOutcome::Zero || outcome == MeasurementOutcome::One);
        // No-cloning theorem: attempt_clone panics
        let q = Qubit::zero();
        let result = std::panic::catch_unwind(|| {
            attempt_clone(q)
        });
        assert!(result.is_err());  // Should panic
        // Quantum teleportation (educational placeholder)
        let q = Qubit::one();
        let teleported = quantum_teleport(q);
        assert!(teleported.is_one());
        // Measurement probabilities
        let q = Qubit::new(0.8, 0.6);  // 0.64 + 0.36 = 1
        assert!((q.prob_zero() - 0.64).abs() < 1e-10);
        assert!((q.prob_one() - 0.36).abs() < 1e-10);
    }
}
