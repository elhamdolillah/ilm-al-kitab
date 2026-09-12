# MAL Interop Interface Contract — Real Gap Analysis
## Status: PLANNED (Awaiting approval for Phase C1.1)

## Purpose
رفع نسبة الرياضيات في MAL عبر سد الفجوة الحقيقية بين Lexer وASTNode وParser.
التكامل عبر المفاهيم (concept mapping)، لا نسخ الملفات.

## Current State (Verified by Code Inspection)

### ASTNode (9 variants)
Empty, Int, FixedPoint, Str, Ident, BinOp, Call, List, Lambda

### Lexer (32 tokens, includes math symbols)
- Forall (∀) — token 65
- In (∈) — token 67
- Mu (μ) — token 69
- Lambda (λ) — token 71
- LinearImplication (⊸) — NOT PRESENT

### Parser (7 parse functions)
- parse_lambda handles λ
- No handling for Forall, In, Mu, LinearImplication

## Real Gap Analysis

| Symbol | Lexer | ASTNode | Parser | External Uses | Priority |
|---|---|---|---|---|---|
| λ (Lambda) | ✅ | ✅ | ✅ | — | Already done |
| ⊸ (Linear Implication) | ❌ | ❌ | ❌ | 19 | **CRITICAL** |
| ∀ (Forall) | ✅ | ❌ | ❌ | 5 | **HIGH** |
| ∈ (In) | ✅ | ❌ | ❌ | 4 | **HIGH** |
| μ (Mu) | ✅ | ❌ | ❌ | 5 | **HIGH** |

## Implementation Plan: Sub-phase C1.1

### 1. Add to Lexer (1 new token)
- LinearImplication (⊸)

### 2. Add to ASTNode (4 new variants)
- LinearLet { name: NodeID, value: NodeID, body: NodeID }
- ForAll { var: NodeID, set: NodeID, body: NodeID }
- SetMembership { elem: NodeID, set: NodeID }
- Mu { var: NodeID, body: NodeID }

### 3. Add Parser functions (4 new)
- parse_linear_let for ⊸
- parse_forall for ∀
- parse_set_membership for ∈
- parse_mu for μ

### 4. Add tests (~13 tests)
- Linear Let: 5 tests (basic, nested, scope, double-move rejection, error)
- ForAll: 3 tests (basic, nested quantifiers, empty set)
- SetMembership: 2 tests (basic, type mismatch)
- Mu: 3 tests (basic, recursion, divergence detection)

### Estimated Effort (Revised)
- Lexer: ~10 lines
- ASTNode: ~40 lines
- Parser: ~200 lines
- Tests: ~100 lines
- **Total: ~350 lines** (revised from 500)

### Target
- ASTNode variants: 9 → 13 (4 new)
- Lexer tokens: 32 → 33 (1 new)
- Tests: 13 → 26 (13 new)

## Completion Criteria
- All 13 tests pass
- No regression in existing 13 parser tests
- Differential testing against math_complete.py successful
- Documentation in KNOB.md complete

## Decision Required
This contract must be approved before Phase C1.1 can begin.
Approval means:
1. Gap analysis is accurate
2. Implementation priority is sound
3. Ready to proceed with Linear Logic + Quantifiers
4. BASELINE_MODIFIED=YES will be declared before any ASTNode changes

**Status:** Awaiting explicit approval

