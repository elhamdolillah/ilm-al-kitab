# Formal Mathematical Specification — ilm-al-kitab
## 1. Axiomatic Foundations
### 1.1 State Space
Σ = (regs: R → ℤ₆₄, mem: Addr → Byte, ip: Addr, flags: F)
where:
- R = {rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp}
- F = {ZF, CF, SF, OF}
- ℤ₆₄ = ℤ mod 2⁶⁴
- Addr = ℤ₆₄
- Byte = {0, 1, ..., 255}
### 1.2 Instruction Set
∀ instruction I ∈ ISA:
I: Σ → Σ
ISA = {mov, add, sub, push, pop, cmp, jmp, jz, jnz, jl, jle, jg, jge, call, ret, loop, syscall}
### 1.3 Arithmetic Operations
∀ a, b ∈ ℤ₆₄:
- add(a, b) = (a + b) mod 2⁶⁴
- sub(a, b) = (a - b) mod 2⁶⁴
- mul(a, b) = (a × b) mod 2⁶⁴
Properties:
- Commutativity: add(a, b) ≡ add(b, a)
- Associativity: add(add(a, b), c) ≡ add(a, add(b, c))
- Identity: add(a, 0) ≡ a
- Inverse: sub(a, a) ≡ 0
- Distributivity: mul(a, add(b, c)) ≡ add(mul(a, b), mul(a, c))
## 2. Set Theory Operations
### 2.1 Basic Operations
∀ A, B, C ⊆ ℤ:
| Operation | Symbol | Definition |
|---|---|---|
| Union | A ∪ B | {x | x ∈ A ∨ x ∈ B} |
| Intersection | A ∩ B | {x | x ∈ A ∧ x ∈ B} |
| Difference | A \ B | {x | x ∈ A ∧ x ∉ B} |
| Symmetric Difference | A Δ B | (A \ B) ∪ (B \ A) |
| Cartesian Product | A × B | {(a,b) | a ∈ A ∧ b ∈ B} |
| Subset | A ⊆ B | ∀x ∈ A, x ∈ B |
| Superset | A ⊇ B | B ⊆ A |
### 2.2 Algebraic Properties
Commutativity:
- A ∪ B ≡ B ∪ A
- A ∩ B ≡ B ∩ A
Associativity:
- (A ∪ B) ∪ C ≡ A ∪ (B ∪ C)
- (A ∩ B) ∩ C ≡ A ∩ (B ∩ C)
Identity:
- A ∪ ∅ ≡ A
- A ∩ U ≡ A (U = universe)
Annihilation:
- A ∩ ∅ ≡ ∅
Idempotence:
- A ∪ A ≡ A
- A ∩ A ≡ A
Distributivity:
- A ∪ (B ∩ C) ≡ (A ∪ B) ∩ (A ∪ C)
- A ∩ (B ∪ C) ≡ (A ∩ B) ∪ (A ∩ C)
Absorption:
- A ∪ (A ∩ B) ≡ A
- A ∩ (A ∪ B) ≡ A
De Morgan's Laws:
- (A ∪ B)' ≡ A' ∩ B'
- (A ∩ B)' ≡ A' ∪ B'
### 2.3 Order Properties
Reflexivity: A ⊆ A
Antisymmetry: (A ⊆ B ∧ B ⊆ A) ⟹ A = B
Transitivity: (A ⊆ B ∧ B ⊆ C) ⟹ A ⊆ C
## 3. Complementary Notation Equivalence
### 3.1 Theorem
∀ semantic operation O ∈ {print, plus, read}:
∃ T_old, T_new: eval(T_old, inputs) ≡ eval(T_new, inputs) ∀ inputs ∈ domain(O)
### 3.2 Proven Mappings
| New Notation | Old Notation | Type Signature |
|---|---|---|
| ⎕ | اطبع | ℤ → String |
| ⊕ | جمع | ℤ × ℤ → ℤ |
| ⊙ | اقرأ | stdin → ℤ |
### 3.3 Equivalence Proof Structure
Proof by Exhaustion:
- 11 test cases covering positive/negative/boundary scenarios
- Each case verifies: output_old == output_new
- Error cases verify: error_type_old == error_type_new
## 4. Syscall Semantics
### 4.1 Syscall Interface
∀ syscall number n = Σ.regs[rax]:
| n | Name | Signature | Formal Definition |
|---|---|---|---|
| 0 | sys_read | fd × buf × count → bytes | mem[rsi..rsi+rdx] ← stdin, rax ← bytes_read |
| 1 | sys_write | fd × buf × count → bytes | stdout ← mem[rsi..rsi+rdx], rax ← bytes_written |
| 60 | sys_exit | code → ∅ | raise ProgramExit(rdi) |
### 4.2 Error Semantics
∀ n ∉ {0, 1, 60}: raise UnknownSyscall(n)
∀ fd ∉ {0} for sys_read: raise InvalidFD(fd)
∀ fd ∉ {1, 2} for sys_write: raise InvalidFD(fd)
## 5. Control Flow Semantics
### 5.1 Jump Instructions
∀ label L, address a = addr(L):
| Instruction | Condition | Formal Definition |
|---|---|---|
| jmp | always | Σ'.ip ← a |
| jz | ZF = 1 | Σ'.ip ← a if ZF = 1 |
| jnz | ZF = 0 | Σ'.ip ← a if ZF = 0 |
| jl | SF ≠ OF | Σ'.ip ← a if SF ≠ OF |
| jle | SF ≠ OF ∨ ZF = 1 | Σ'.ip ← a if SF ≠ OF ∨ ZF = 1 |
| jg | SF = OF ∧ ZF = 0 | Σ'.ip ← a if SF = OF ∧ ZF = 0 |
| jge | SF = OF | Σ'.ip ← a if SF = OF |
### 5.2 Call/Return
call: L → Σ where:
- Σ'.mem[Σ.regs[rsp]-8] ← Σ.ip + inst_len
- Σ'.regs[rsp] ← Σ.regs[rsp] - 8
- Σ'.ip ← addr(L)
ret: Σ → Σ where:
- Σ'.ip ← Σ.mem[Σ.regs[rsp]]
- Σ'.regs[rsp] ← Σ.regs[rsp] + 8
Invariant: ∀ call/ret pair: ret(call(Σ)) restores Σ.ip
### 5.3 Loop Instructions
loop: L × R → Σ where:
- Σ'.regs[rcx] ← Σ.regs[rcx] - 1
- Σ'.ip ← addr(L) if Σ'.regs[rcx] ≠ 0, else Σ.ip + inst_len
Invariant: ∀ n: loop executes exactly n times where n = initial rcx
## 6. Memory Model
### 6.1 Address Space
mem: Addr → Byte where Addr = ℤ₆₄, Byte = {0, ..., 255}
∀ address a ∈ Addr:
- Read: read(a) = mem[a]
- Write: write(a, v) ⟹ mem[a] ← v
- Bounds: a < 2⁶⁴ (64-bit address space)
### 6.2 Stack Operations
push: Operand → Σ where:
- Σ'.mem[Σ.regs[rsp]-8] ← eval(op)
- Σ'.regs[rsp] ← Σ.regs[rsp] - 8
pop: R → Σ where:
- Σ'.regs[r] ← Σ.mem[Σ.regs[rsp]]
- Σ'.regs[rsp] ← Σ.regs[rsp] + 8
Invariant: ∀ x: pop(push(x)) ≡ x
## 7. Determinism and Evidence
### 7.1 Determinism Axiom
∀ program P, input I:
run(P, I) produces identical output across all executions
### 7.2 Evidence Axiom
∀ test T:
- T produces (stdout, exit_code, SHA-256)
- SHA-256 is invariant across runs
- Evidence stored in evidence/ directory
### 7.3 Fail-Closed Contract
∀ unexpected error E:
- STATUS ← FAIL_CLOSED
- exit_code ← 1
- No partial results returned
- E contains (type, message, context)
## 8. Constitutional Rules
### 8.1 Protection Axioms
∀ changes to project:
1. MAL Parser/Compiler: UNCHANGED unless explicitly authorized
2. Baseline: UNTOUCHED unless explicitly authorized
3. New features: PROVEN_FOR_SCOPE before integration
4. Innovation: requires explicit user consent
### 8.2 Proof Requirements
∀ claim "X is proven":
- ∃ corpus with ≥ 10 test cases
- ∃ SHA-256 evidence
- ∃ exit code verification
- ∃ git commit with full message
### 8.3 Equivalence Relation
∀ programs P₁, P₂:
P₁ ≡ P₂ ⟺ ∀ inputs I: run(P₁, I) = run(P₂, I)
