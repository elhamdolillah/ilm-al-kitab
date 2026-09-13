# ASM-SEM-002 — Corpus أولي

الحالة: `PLANNED`

هذه الحالات مواصفات اختبار، وليست دليلاً تنفيذياً حتى يمر model مستقل عليها.

| ID | البرنامج | الشرط المتوقع |
|---|---|---|
| JMP-001 | `jmp target` | rip = target، باقي الحالة لم تتغير |
| JE-001 | `mov rax, 5; cmp rax, 5; je target` | rip = target لأن ZF=1 |
| JE-002 | `mov rax, 5; cmp rax, 6; je target` | rip يتقدم للتعليمية التالية لأن ZF=0 |
| JNE-001 | `mov rax, 5; cmp rax, 6; jne target` | rip = target لأن ZF=0 |
| JL-001 | `mov rax, -1; cmp rax, 0; jl target` | rip = target لأن -1 < 0 (signed) |
| CALL-001 | `call func` | rip = func، rsp -= 8، [rsp] = return_addr |
| CALL-002 | `mov rsp, 0x2000; call func` | rsp = 0x1FF8، [0x1FF8] = return_addr |
| RET-001 | `mov rsp, 0x1FF8; mov [rsp], 0x100; ret` | rip = 0x100، rsp = 0x2000 |
| LOOP-001 | `mov rcx, 3; loop target` | rcx = 2، rip = target |
| LOOP-002 | `mov rcx, 1; loop target` | rcx = 0، rip يتقدم (لا قفزة) |
| ERR-003 | `jmp invalid_addr` | `ERROR(InvalidJumpTarget)` |

## متطلبات المخرج
```text
raw stdout
raw stderr
exit code
SHA-256(stdout)
```
