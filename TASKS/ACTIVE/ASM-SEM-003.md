# ASM-SEM-003 — Syscall Instructions

## الحالة
`PROVEN_FOR_SCOPE`

## الهدف
إكمال نموذج x86 ليشمل استدعاءات النظام الأساسية.

## النطاق
```text
syscall (dispatch based on rax)
sys_read (n=0)
sys_write (n=1)
sys_exit (n=60)
```

## خارج النطاق
```text
fork, execve, mmap, epoll, pipe2
```

## العقد
- كل syscall يحاكي سلوكه الأساسي في بيئة معزولة
- fail-closed: syscall غير معروف يرفع خطأً صريحاً

## معايير القبول
- 3 syscalls تعمل بشكل صحيح
- stdout خام وexit code وSHA-256
- عدم تغيير MAL Parser أو Compiler

## قرار السلامة
لا تضاف التعليمات إلى MAL Parser أو AOT compiler في هذه المرحلة.
---
## Mathematical Contract
∀ syscall n = Σ.regs[rax]:
- n = 0: sys_read(fd, buf, count) → mem[rsi..] ← stdin
- n = 1: sys_write(fd, buf, count) → stdout ← mem[rsi..]
- n = 60: sys_exit(code) → ProgramExit(rdi)
- n ∉ {0, 1, 60}: UnknownSyscall(n)
Evidence: SHA-256(stdout) invariant ∀ runs
