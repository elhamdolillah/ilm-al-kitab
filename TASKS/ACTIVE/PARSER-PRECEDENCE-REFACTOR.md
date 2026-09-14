# Parser Precedence Refactor — Multi-Phase Implementation
## Status
`PHASE_1_PLANNED`
## Goal
Refactor MAL Parser to support proper operator precedence hierarchy.
## Phase 1: Lexer Token Addition
Add these tokens: /, %, ^, ≤, ≥, ¬, ∧, ∨
## Phase 2: Parser Restructure
Layers: logical_or, logical_and, logical_not, comparison, additive, multiplicative, power, unary, postfix, primary
## Phase 3: Tests
Test all new operators and precedence rules
## Success Criteria
- All new tokens lex correctly
- Parser builds correct AST
- Precedence matches mathematical convention
- All tests pass
