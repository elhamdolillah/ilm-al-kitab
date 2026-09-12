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
