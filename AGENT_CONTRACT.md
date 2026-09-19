# MAL Symbol Integration Agent Contract

## Mission
Execute exactly one active phase from TASKS/ACTIVE/SYMBOLS_NEXT.md per run.

## Required preflight
1. Read this contract and the active task.
2. Run /root/ilm-al-kitab/tools/run_symbol_phase.sh.
3. Stop immediately if the baseline fails.
4. Inspect actual Lexer, Parser, Arena AST, compiler, evaluator, and tests before edits.
5. Never assume a token, AST variant, or public API exists.

## Scope
1. Perform only the active task.
2. Do not start a later phase.
3. Do not delete existing tests.
4. Do not use unsafe Rust.
5. Do not install packages, use the network, or change unrelated files.
6. Do not edit .qwen_autotest/run_all_tests.sh unless the active task explicitly permits it.

## Required gates
1. Run cargo fmt --check.
2. Run the targeted package tests in release mode.
3. Run /root/ilm-al-kitab/.qwen_autotest/run_all_tests.sh.
4. Success requires FINAL_STATUS: PASS.

## Failure handling
1. If a command fails, diagnose and make only small local fixes within the active scope.
2. If the phase remains blocked, do not proceed.
3. Write TASKS/ACTIVE/PHASE-BLOCKED.md with the exact failure, files touched, test output summary, and needed decision.
4. Leave later phases untouched.

## Semantic decisions
Stop and write PHASE-BLOCKED.md instead of guessing when the task needs an unspecified decision about:
- Numeric representation or overflow.
- Division or modulo by zero.
- Meaning of ⊕.
- Boolean literals or cross-type equality.
- Set universe or complement.
- Lambda grammar extensions.
- Meaning or precedence of a new symbol.

## Success handling
On success:
1. Write TASKS/COMPLETED/PHASE-REPORT.md.
2. Report changed files, exact syntax, precedence, associativity, tests run, and known limits.
3. Do not begin the next phase.
