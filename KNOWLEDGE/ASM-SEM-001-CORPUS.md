# ASM-SEM-001 — Corpus أولي

الحالة: `PROVEN_FOR_SCOPE`

هذه الحالات مواصفات اختبار، وليست دليلاً تنفيذياً حتى يمر model مستقل عليها.

| ID | البرنامج | الشرط المتوقع |
|---|---|---|
| MOV-001 | `mov rax, 7` | `rax=7`، وباقي الحالة لا تتغير |
| ADD-001 | `mov rax, 7; add rax, 5` | `rax=12`، وتحديث الأعلام المحددة فقط |
| SUB-001 | `mov rax, 7; sub rax, 5` | `rax=2`، وتحديث الأعلام المحددة فقط |
| STACK-001 | `rsp=0x1000; push rax; pop rbx` | `rbx` يساوي قيمة `rax`، و`rsp=0x1000` |
| CMP-001 | `mov rax, 4; cmp rax, 4` | `ZF=1`، ولا تُخزن نتيجة المقارنة |
| TEST-001 | `mov rax, 0; test rax, rax` | `ZF=1`، ولا يتغير `rax` |
| ERR-001 | `pop rax` مع ذاكرة غير صالحة | `ERROR(InvalidMemory)` |
| ERR-002 | `mov unknown, 1` | `ERROR(UnknownRegister)` |
| BOUND-001 | `2^64-1 + 1` | نتيجة معيارية 0، مع أعلام overflow/carry بحسب العقد |

## متطلبات المخرج

يجب أن ينتج الاختبار:

```text
raw stdout
raw stderr
exit code
SHA-256(stdout)
```

ولا يكفي تطابق قيمة واحدة إذا كانت flags أو الحالة الجانبية غير صحيحة.
