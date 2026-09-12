# MAL Interop Interface Contract
## Status: PLANNED
## Purpose
تكامل MAL المحلي (Rust/Arena) مع المكونات الخارجية عبر المفاهيم، لا نسخ الملفات.
## 1. NodeID
Local: `pub struct NodeID(pub u32)` مع `INVALID = u32::MAX`
External: Opaque handle مع validation
## 2. ASTNode Common Subset
Local: Empty, Int, BinOp, Call, Lambda (Arena-based)
External: Python tuples (dynamic)
Integration: Map Python tuples → Rust ASTNode
## 3. Errors
Local: `ArenaError`, `ParserError` (structured, fail-closed)
External: `raise Exception("رسالة عربية")` (fail-fast)
Contract: Structured errors with line/col
## 4. Determinism
Guaranteed: No network, no randomness, no time
Allowed: File I/O for source code
Forbidden: HTTP, rand, SystemTime
## 5. Integration Strategy
- Read `math_complete.py` for concepts
- Port deterministic algorithms (lambda eval)
- Differential testing against external suite
- Document mappings in KNOWLEDGE/
## Status
- Phase A (Inventory): ✅ COMPLETE
- Phase B (Discovery): ✅ COMPLETE
- Phase B (Contract): 📝 PLANNED (this file)
- Phase C (Integration): ⏳ PENDING approval
## Next Steps
1. Review and approve this contract
2. Define test cases for differential testing
3. Port simple cases (Int, BinOp) first
4. Add Call/Lambda support
5. Document all decisions
## Implementation Phases (Updated Priority)
### Phase C1: Comprehensive Mathematics Integration (أولوية قصوى)
**الهدف**: رفع نسبة الرياضيات من 5.9% إلى >80%
**Sub-phase C1.1: Priority 1 Mathematics (Linear Logic + Quantifiers)**
- Add 7 AST variants: LinearLet, Tensor, Par, OfCourse, ForAll, Exists, SetMembership
- Add 7 Lexer tokens: LinearImplication, Tensor, Par, OfCourse, ForAll, Exists, In, NotIn
- Add ~15 tests covering all new variants
- Target: 16 AST variants (9 current + 7 new)
- Estimated effort: ~500 lines of Rust code
**Sub-phase C1.2: Priority 2 Mathematics (Set Theory + Fixed-point)**
- Add 9 AST variants: SetUnion, SetIntersection, SetDifference, SetSymmetricDifference, CartesianProduct, Subset, Superset, Mu, Nu
- Add 9 Lexer tokens
- Add ~20 tests
- Target: 25 AST variants (16 + 9 new)
- Estimated effort: ~700 lines of Rust code
**Sub-phase C1.3: Priority 3 Mathematics (Type Theory)**
- Add 4 AST variants: DependentProduct, DependentSum, TypeAnnotation, FunctionType
- Add 4 Lexer tokens
- Add ~15 tests
- Target: 29 AST variants (25 + 4 new)
- Estimated effort: ~600 lines of Rust code
**Sub-phase C1.4: Priority 4 Mathematics (Category Theory + Proof Theory)**
- Add 8 AST variants: Morphism, Composition, Identity, NaturalTransformation, Proves, Entails, Therefore, Because, QED
- Add 8 Lexer tokens
- Add ~20 tests
- Target: 37 AST variants (29 + 8 new)
- Estimated effort: ~800 lines of Rust code
**Phase C1 Completion Criteria**:
- All 37 AST variants implemented and tested
- Mathematics coverage >80%
- All tests pass (110+ tests total)
- Differential testing against `math_complete.py` successful
- Documentation in KNOB.md complete
### Phase C2: Gradual Integration (بعد اكتمال C1)
**الهدف**: نقل المفاهيم غير الرياضية من الخارجي إلى المحلي
**Sub-phase C2.1: Test Cases Migration**
- Copy test cases from `math_complete.py` to `MAL/tests/`
- Adapt tests to use new AST variants
- Ensure all tests pass with new mathematics layer
**Sub-phase C2.2: Examples Migration**
- Copy example programs from `arabic-math-lang/examples/`
- Adapt examples to use new syntax
- Document examples in `MAL/docs/`
**Sub-phase C2.3: Documentation Integration**
- Copy relevant documentation from `arabic-math-lang/docs/`
- Translate and adapt to Rust/MAL context
- Update `KNOB.md` with integration decisions
**Phase C2 Completion Criteria**:
- All test cases migrated and passing
- Examples working with new mathematics layer
- Documentation complete and accurate
- No regression in existing functionality
## Decision Required (Updated)
This contract must be reviewed and approved before Phase C1 can begin. Approval means:
1. All 7 mathematics layers are acceptable
2. Phase C1 (mathematics first) priority is sound
3. Phase C2 (gradual integration after mathematics) is acceptable
4. Constitutional compliance is verified
5. Ready to proceed with comprehensive mathematics integration
**Status**: Awaiting review and approval for Phase C1
