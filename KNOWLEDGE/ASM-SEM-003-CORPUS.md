# ASM-SEM-003 — Syscall Corpus

الحالة: `PROVEN_FOR_SCOPE`

هذه الحالات مواصفات اختبار، وليست دليلاً تنفيذياً حتى يمر model مستقل عليها.

| ID | البرنامج | الشرط المتوقع |
|---|---|---|
| SYS-001 | `rax=1; rdi=1; rsi=buf; rdx=5; syscall` | كتابة 5 بايتات إلى stdout |
| SYS-002 | `rax=0; rdi=0; rsi=buf; rdx=10; syscall` | قراءة 10 بايتات من stdin |
| SYS-003 | `rax=60; rdi=0; syscall` | إنهاء البرنامج مع exit code 0 |
| SYS-004 | `rax=60; rdi=42; syscall` | إنهاء البرنامج مع exit code 42 |
| SYS-005 | `rax=999; syscall` | `ERROR(UnknownSyscall)` |
| SYS-006 | `rax=1; rdi=99; rsi=buf; rdx=5; syscall` | `ERROR(InvalidFD)` |

## متطلبات المخرج

```text
raw stdout
raw stderr
exit code
SHA-256(stdout)
```
---
## Mathematical Specification
∀ test T ∈ corpus:
- T.syscall ∈ {sys_read, sys_write, sys_exit}
- sys_read: fd × buf × count → Σ where mem[rsi..] ← stdin
- sys_write: fd × buf × count → Σ where stdout ← mem[rsi..]
- sys_exit: code → ∅ (raises ProgramExit)
- ∀ n ∉ {0, 1, 60}: UnknownSyscall(n) raised
