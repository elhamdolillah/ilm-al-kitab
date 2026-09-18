# KNOB.md - Project State of Truth
Last Updated: 2026-09-18
Session Achievement: Complete Arabic to ELF compiler pipeline
## Architecture Overview
Arabic Source (.mal)
    |
mal_lexer        Tokenizes UTF-8/Arabic
    |            Tokens: :=, !, sqrt, abs, etc
mal_parser       Recursive descent to AST
    |            ASTNode tree in Arena
mal_nir          AST to SSA IR
    |            NIRFunction with values, blocks
mal_native       NIR to x86-64 NASM
    |            Direct syscalls, no libc
malc-native      CLI tool
    |            Reads .mal, writes ELF
    V
ELF Binary (runs on Linux x86-64)
## Crates
| Crate | Purpose | Status |
|-------|---------|--------|
| mal_arena | ASTNode storage, NodeID, TypeTag | OK |
| mal_types | BinaryOp, UnaryOp, ScalarValue | OK |
| mal_lexer | Tokenization, UTF-8 safe | OK |
| mal_parser | Recursive descent parser | OK |
| mal_nir | SSA-based IR + AST lowering | OK |
| mal_native | x86-64 codegen + helpers | OK |
| mal_index | Index types | OK |
| malc-native | CLI compiler | OK |
## Language Features (Working)
### Arithmetic
5 + 3 = 8
10 - 3 = 7
7 * 6 = 42
100 / 10 = 10
### Comparison
5 > 3 = 1 (true)
3 = 5 = 0 (false)
### Logical
(5 > 3) AND (2 < 4) = 1
NOT 0 = 1 (boolean NOT)
!0 = 1 (alias)
### Assignment
s := 5  (Arabic variable)
### Builtins (Arabic + English)
| Arabic | English | Tag | Example |
|--------|---------|-----|---------|
| root | sqrt | 100 | root(144) = 12 |
| abs | abs | 101 | abs(-42) = 42 |
| floor | floor | 102 | |
| power | power | 103 | power(2,10) = 1024 |
| exp | exp | 104 | |
| print | print_int | 105 | print(42) |
## Known Limitations
| Limitation | Workaround | Status |
|------------|------------|--------|
| Exit code 8-bit (0-255) | Use print() for values >= 256 | Active |
| Nested calls register alloc | Fix in mal_native codegen | Investigating |
## Running the Compiler
cd MAL/tools/malc-native
cargo build --release
./target/release/malc-native examples/sum.mal -o /tmp/sum.bin
chmod +x /tmp/sum.bin
/tmp/sum.bin
Show NIR: ./target/release/malc-native --emit-nir examples/sum.mal
Show ASM: ./target/release/malc-native --emit-asm examples/sum.mal
## Test Coverage
Arithmetic: 8/8 passing
Comparison: 7/7 passing
Logical: 4/4 passing
Unary: 3/3 passing
Assignment: 3/3 passing
Builtins: 6/6 passing
Print: 2/2 passing (stdout)
Regression: 100% maintained
Total: 30+ test cases
## Key Design Decisions
1. No libc - Direct syscalls (sys_write, sys_exit, sys_brk)
2. Arena allocation - No GC, deterministic memory
3. SSA IR (NIR) - Single static assignment
4. UTF-8 native - Arabic identifiers via char_indices()
5. Fail-closed - Explicit error types, no panics
## Git Commits This Session
- 8d6633e - Complete AST to NIR with nested calls
- 30f31ba - Full language support + Arabic identifiers
- 684b2a0 - End-to-end pipeline fully working
