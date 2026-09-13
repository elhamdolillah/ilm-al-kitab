# Formal Theorems and Proofs — ilm-al-kitab
## 1. Arithmetic Theorems
### 1.1 Commutativity of Addition
Theorem: ∀ a, b ∈ ℤ₆₄: add(a, b) ≡ add(b, a)
Proof:
By definition, add(a, b) = (a + b) mod 2⁶⁴
Since integer addition is commutative: a + b = b + a
Therefore: add(a, b) = (a + b) mod 2⁶⁴ = (b + a) mod 2⁶⁴ = add(b, a) ∎
### 1.2 Associativity of Addition
Theorem: ∀ a, b, c ∈ ℤ₆₄: add(add(a, b), c) ≡ add(a, add(b, c))
Proof:
add(add(a, b), c) = ((a + b) mod 2⁶⁴ + c) mod 2⁶⁴
                  = (a + b + c) mod 2⁶⁴
add(a, add(b, c)) = (a + (b + c) mod 2⁶⁴) mod 2⁶⁴
                  = (a + b + c) mod 2⁶⁴
Therefore: add(add(a, b), c) ≡ add(a, add(b, c)) ∎
### 1.3 Identity Element
Theorem: ∀ a ∈ ℤ₆₄: add(a, 0) ≡ a
Proof:
add(a, 0) = (a + 0) mod 2⁶⁴ = a mod 2⁶⁴ = a ∎
### 1.4 Inverse Element
Theorem: ∀ a ∈ ℤ₆₄: sub(a, a) ≡ 0
Proof:
sub(a, a) = (a - a) mod 2⁶⁴ = 0 mod 2⁶⁴ = 0 ∎
### 1.5 Distributivity
Theorem: ∀ a, b, c ∈ ℤ₆₄: mul(a, add(b, c)) ≡ add(mul(a, b), mul(a, c))
Proof:
mul(a, add(b, c)) = a × (b + c) mod 2⁶⁴
                  = (a × b + a × c) mod 2⁶⁴
add(mul(a, b), mul(a, c)) = (a × b + a × c) mod 2⁶⁴
Therefore: mul(a, add(b, c)) ≡ add(mul(a, b), mul(a, c)) ∎
## 2. Set Theory Theorems
### 2.1 Commutativity of Union
Theorem: ∀ A, B ⊆ ℤ: A ∪ B ≡ B ∪ A
Proof:
x ∈ A ∪ B ⟺ x ∈ A ∨ x ∈ B
x ∈ B ∪ A ⟺ x ∈ B ∨ x ∈ A
Since logical OR is commutative: x ∈ A ∨ x ∈ B ⟺ x ∈ B ∨ x ∈ A
Therefore: A ∪ B ≡ B ∪ A ∎
### 2.2 Commutativity of Intersection
Theorem: ∀ A, B ⊆ ℤ: A ∩ B ≡ B ∩ A
Proof:
x ∈ A ∩ B ⟺ x ∈ A ∧ x ∈ B
x ∈ B ∩ A ⟺ x ∈ B ∧ x ∈ A
Since logical AND is commutative: x ∈ A ∧ x ∈ B ⟺ x ∈ B ∧ x ∈ A
Therefore: A ∩ B ≡ B ∩ A ∎
### 2.3 Associativity of Union
Theorem: ∀ A, B, C ⊆ ℤ: (A ∪ B) ∪ C ≡ A ∪ (B ∪ C)
Proof:
x ∈ (A ∪ B) ∪ C ⟺ (x ∈ A ∨ x ∈ B) ∨ x ∈ C
x ∈ A ∪ (B ∪ C) ⟺ x ∈ A ∨ (x ∈ B ∨ x ∈ C)
Since logical OR is associative: (x ∈ A ∨ x ∈ B) ∨ x ∈ C ⟺ x ∈ A ∨ (x ∈ B ∨ x ∈ C)
Therefore: (A ∪ B) ∪ C ≡ A ∪ (B ∪ C) ∎
### 2.4 De Morgan's Laws
Theorem 1: ∀ A, B ⊆ ℤ: (A ∪ B)' ≡ A' ∩ B'
Proof:
x ∈ (A ∪ B)' ⟺ x ∉ A ∪ B
             ⟺ ¬(x ∈ A ∨ x ∈ B)
             ⟺ x ∉ A ∧ x ∉ B
             ⟺ x ∈ A' ∧ x ∈ B'
             ⟺ x ∈ A' ∩ B'
Therefore: (A ∪ B)' ≡ A' ∩ B' ∎
Theorem 2: ∀ A, B ⊆ ℤ: (A ∩ B)' ≡ A' ∪ B'
Proof:
x ∈ (A ∩ B)' ⟺ x ∉ A ∩ B
             ⟺ ¬(x ∈ A ∧ x ∈ B)
             ⟺ x ∉ A ∨ x ∉ B
             ⟺ x ∈ A' ∨ x ∈ B'
             ⟺ x ∈ A' ∪ B'
Therefore: (A ∩ B)' ≡ A' ∪ B' ∎
### 2.5 Absorption Laws
Theorem 1: ∀ A, B ⊆ ℤ: A ∪ (A ∩ B) ≡ A
Proof:
x ∈ A ∪ (A ∩ B) ⟺ x ∈ A ∨ (x ∈ A ∧ x ∈ B)
Since x ∈ A implies x ∈ A ∨ (x ∈ A ∧ x ∈ B):
A ⊆ A ∪ (A ∩ B)
Since x ∈ A ∨ (x ∈ A ∧ x ∈ B) implies x ∈ A:
A ∪ (A ∩ B) ⊆ A
Therefore: A ∪ (A ∩ B) ≡ A ∎
Theorem 2: ∀ A, B ⊆ ℤ: A ∩ (A ∪ B) ≡ A
Proof:
x ∈ A ∩ (A ∪ B) ⟺ x ∈ A ∧ (x ∈ A ∨ x ∈ B)
Since x ∈ A implies x ∈ A ∧ (x ∈ A ∨ x ∈ B):
A ⊆ A ∩ (A ∪ B)
Since x ∈ A ∧ (x ∈ A ∨ x ∈ B) implies x ∈ A:
A ∩ (A ∪ B) ⊆ A
Therefore: A ∩ (A ∪ B) ≡ A ∎
## 3. Stack Invariant Theorem
Theorem: ∀ x: pop(push(x)) ≡ x
Proof:
push(x): Σ'.mem[Σ.regs[rsp]-8] ← x, Σ'.regs[rsp] ← Σ.regs[rsp]-8
pop(r): Σ''.regs[r] ← Σ'.mem[Σ'.regs[rsp]], Σ''.regs[rsp] ← Σ'.regs[rsp]+8
After push: Σ'.regs[rsp] = Σ.regs[rsp] - 8, Σ'.mem[Σ.regs[rsp]-8] = x
After pop: Σ''.regs[r] = Σ'.mem[Σ'.regs[rsp]] = Σ'.mem[Σ.regs[rsp]-8] = x
           Σ''.regs[rsp] = Σ'.regs[rsp] + 8 = Σ.regs[rsp]
Therefore: pop(push(x)) ≡ x ∎
## 4. Call/Return Invariant Theorem
Theorem: ∀ call/ret pair: ret(call(Σ)) restores Σ.ip
Proof:
call(L): Σ'.mem[Σ.regs[rsp]-8] ← Σ.ip + inst_len
         Σ'.regs[rsp] ← Σ.regs[rsp] - 8
         Σ'.ip ← addr(L)
ret: Σ''.ip ← Σ'.mem[Σ'.regs[rsp]]
     Σ''.regs[rsp] ← Σ'.regs[rsp] + 8
After call: Σ'.mem[Σ.regs[rsp]-8] = Σ.ip + inst_len
            Σ'.regs[rsp] = Σ.regs[rsp] - 8
After ret: Σ''.ip = Σ'.mem[Σ'.regs[rsp]] = Σ'.mem[Σ.regs[rsp]-8] = Σ.ip + inst_len
           Σ''.regs[rsp] = Σ'.regs[rsp] + 8 = Σ.regs[rsp]
Therefore: ret(call(Σ)) restores Σ.ip ∎
## 5. Loop Invariant Theorem
Theorem: ∀ n: loop executes exactly n times where n = initial rcx
Proof:
Let rcx₀ = n (initial value)
Each iteration: rcx ← rcx - 1
Loop continues while rcx ≠ 0
After k iterations: rcx = rcx₀ - k = n - k
Loop stops when rcx = 0, i.e., n - k = 0, i.e., k = n
Therefore: loop executes exactly n times ∎
## 6. Determinism Theorem
Theorem: ∀ program P, input I: run(P, I) produces identical output
Proof:
By induction on program structure:
- Base case: single instruction I — deterministic by definition
- Inductive step: if P₁ and P₂ are deterministic, then P₁; P₂ is deterministic
- Conditional: if condition is deterministic, then branch is deterministic
- Loop: if body is deterministic and iteration count is fixed, then loop is deterministic
Therefore: ∀ P, I: run(P, I) is deterministic ∎
## 7. Complementary Notation Theorem
Theorem: ∀ semantic operation O ∈ {print, plus, read}:
∃ T_old, T_new: eval(T_old, inputs) ≡ eval(T_new, inputs)
Proof:
For O = print:
- T_old = اطبع, T_new = ⎕
- Both output str(x) to stdout
- Therefore: eval(اطبع, x) ≡ eval(⎕, x) ∎
For O = plus:
- T_old = جمع, T_new = ⊕
- Both compute a + b
- Therefore: eval(جمع, a, b) ≡ eval(⊕, a, b) ∎
For O = read:
- T_old = اقرأ, T_new = ⊙
- Both read from stdin
- Therefore: eval(اقرأ) ≡ eval(⊙) ∎
