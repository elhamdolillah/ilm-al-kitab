# Phase 1A — Existing Parser Contract

## Objective
Document and test current MAL parser behavior only.

## Required work
- Add AST-shape tests for `1 + 2 · 3`.
- Add AST-shape tests for `1 - 2 - 3`.
- Test direct calls: `f(x)` and `f(x, y + 1)`.
- Test fail-closed parsing for `f(x`.
- Retain `(f)(x)` as an ignored future-feature test.
- Document current syntax, precedence, associativity, and the absence of arbitrary postfix calls.

## Forbidden
- Do not add operators or tokens.
- Do not modify Lexer, Parser behavior, AST public APIs, Compiler, or `.qwen_autotest/run_all_tests.sh`.

## Acceptance
- `cargo fmt --check` passes.
- Parser release tests pass.
- `.qwen_autotest/run_all_tests.sh` prints `FINAL_STATUS: PASS`.
- Write `TASKS/COMPLETED/PHASE-1A-REPORT.md`.
