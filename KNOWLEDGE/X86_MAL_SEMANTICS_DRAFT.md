# مسودة دلالات x86-64 وربطها بـMAL

## الحالة الدستورية

`RESEARCH`

هذه الوثيقة تحول المراجع الرياضية المرفقة إلى عقد بحثي قابل للتقسيم والاختبار. لا تعلن أن MAL تدعم تعليمات x86 المذكورة، ولا أن التعريفات مدققة رسمياً. كل مجموعة تنتقل لاحقاً إلى `PROVEN_FOR_SCOPE` فقط بعد نموذج تنفيذي واختبارات مستقلة.

## 1. تصحيح نموذج الحالة

لأغراض x86-64 العملي نستخدم الحالة:

```text
Σ = Mem × Reg × Flags × Control
```

حيث:

- `Mem` خريطة جزئية من عناوين byte إلى قيم byte، وليست مصفوفة كاملة قابلة للتنفيذ.
- `Reg` خريطة للسجلات العامة والمؤشرات وسجلات التحكم المطلوبة في النطاق.
- `Flags` يحوي CF وPF وAF وZF وSF وOF مع تحديد ما إذا كانت التعليمة تغير كل علَم.
- `Control` يحوي RIP وحالة التوقف واتجاه السلاسل عند الحاجة.

الدوال الأساسية:

```text
read_reg : RegName × Σ → Value
write_reg: RegName × Value × Σ → Σ
read_mem : Address × Width × Σ ⇀ Value
write_mem: Address × Value × Width × Σ ⇀ Σ
```

السهم الجزئي `⇀` مهم لأن القراءة أو الكتابة قد تفشل بسبب عنوان غير صالح أو عرض غير صحيح. لا يجوز تحويل الفشل إلى قيمة صامتة.

## 2. النطاق الأول القابل للإثبات

يبدأ التكامل بمجموعة صغيرة لا تتطلب نموذجاً كاملاً للمعالج:

| المجموعة | التعليمات | الحالة |
|---|---|---|
| نقل | `mov` | `PLANNED` |
| حساب | `add`, `sub` | `PLANNED` |
| مكدس | `push`, `pop` | `PLANNED` |
| مقارنة | `cmp`, `test` | `PLANNED` |
| تحكم | `jmp`, `je`, `jne`, `loop` | `PLANNED` |
| نظام | `syscall` لـ`read/write/exit` فقط | `PLANNED` |

لا تدخل `SIMD` أو floating point أو المقاطعات أو كل صيغ addressing في المرحلة الأولى.

## 3. دلالات انتقال صغيرة

نعرّف تكوين البرنامج كزوج:

```text
⟨I ; P, σ⟩ → ⟨P, σ'⟩
```

إذا كانت دلالة التعليمة معرفة:

```text
⟦I⟧(σ) = σ'
```

فإن:

```text
⟨I ; P, σ⟩ → ⟨P, σ'⟩
```

وإذا كانت غير معرفة أو فشلت الذاكرة:

```text
⟦I⟧(σ) = ERROR(e)
```

ولا يسمح المسار الحتمي بالاستمرار وكأن التعليمة نجحت.

## 4. تعليمات النقل والحساب

```text
val(op, σ) = read_reg(op, σ)       إذا كان op سجلاً
val(op, σ) = immediate(op)         إذا كان op قيمة فورية

⟦mov d,s⟧(σ) = write_reg(d, val(s,σ), σ)

r = val(d,σ) +₆₄ val(s,σ)
⟦add d,s⟧(σ) = write_reg(d, r, set_add_flags(d,s,r,σ))

r = val(d,σ) -₆₄ val(s,σ)
⟦sub d,s⟧(σ) = write_reg(d, r, set_sub_flags(d,s,r,σ))
```

تحتاج `set_add_flags` و`set_sub_flags` مواصفة مستقلة لكل علَم؛ لا يكفي ذكر ZF وحده.

## 5. المكدس

في النطاق 64-bit:

```text
σ1 = write_reg(rsp, read_reg(rsp,σ) -₆₄ 8, σ)
⟦push x⟧(σ) = write_mem(read_reg(rsp,σ1), val(x,σ), 8, σ1)

v = read_mem(read_reg(rsp,σ), 8, σ)
σ1 = write_reg(dst, v, σ)
⟦pop dst⟧(σ) = write_reg(rsp, read_reg(rsp,σ)+₆₄ 8, σ1)
```

يجب اختبار invariant يثبت أن `push` ثم `pop` يعيدان `rsp` وقيمة المسجل ضمن شروط عدم فشل الذاكرة.

## 6. المقارنة والتحكم

```text
⟦cmp a,b⟧(σ) = set_sub_flags(a,b,val(a,σ)-₆₄ val(b,σ),σ)
⟦test a,b⟧(σ) = set_logic_flags(val(a,σ) ∧ val(b,σ),σ)

cond(je,σ)  = ZF(σ) = 1
cond(jne,σ) = ZF(σ) = 0
cond(jl,σ)  = SF(σ) ⊕ OF(σ) = 1
```

تغيير `RIP` لا ينفذ وحده في نموذج MAL؛ يحتاج interpreter أو code generator يطابقه ويختبره.

## 7. النظام

يبدأ `syscall` بثلاث حالات فقط:

```text
rax=0 → read
rax=1 → write
rax=60 → exit
```

أي رقم آخر يعيد `ERROR(UnsupportedSyscall)`. لا يجوز ادعاء دعم `fork`, `execve`, `pipe2`, `epoll` أو `eventfd2` بمجرد كتابة أسمائها في جدول.

## 8. واجهة MAL المقترحة

يحتاج التكامل إلى طبقة IR مستقلة، لا ربط مباشر مع كل تفاصيل AST الحالية:

```text
enum Instruction {
    Mov(Operand, Operand),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Push(Operand),
    Pop(Register),
    Cmp(Operand, Operand),
    Test(Operand, Operand),
    Jmp(Label),
    Jcc(Condition, Label),
    Syscall,
}
```

هذه القائمة مواصفة تصميم فقط. لا تُضاف إلى Rust قبل عقد types وerrors وtest corpus.

## 9. اختبارات القبول

لكل تعليمة يجب وجود:

1. حالة صحيحة بسيطة.
2. حدود 0 و`2^64-1` عند الصلة.
3. overflow أو underflow.
4. عنوان ذاكرة غير صالح.
5. تحقق flags المتغيرة وغير المتغيرة.
6. stdout خام وexit code.
7. SHA-256 للمخرج.

الحد الأدنى للانتقال إلى `PROVEN_FOR_SCOPE` هو نجاح جميع الحالات داخل النطاق، وليس نجاح مثال واحد.

## 10. العلاقة مع MAL الحالية

`MAL` الحالية تثبت Parser وLexer وArena وC2 ضمن نطاقها. هذا لا يثبت code generator x86 أو evaluator أو syscall runtime. لذلك تكون الخطة:

1. بناء IR صغير مستقل.
2. كتابة model checker offline محدود.
3. مقارنة IR مع Assembly الناتج في أمثلة صغيرة.
4. إضافة codegen بعد ثبات الدلالات.
5. تشغيل الاختبارات التفاضلية قبل أي إعلان.

## 11. مراحل لاحقة

- `ASM-SEM-001`: عقد الحالة والأنواع.
- `ASM-SEM-002`: mov/add/sub/cmp/test.
- `ASM-SEM-003`: push/pop وinvariant المكدس.
- `ASM-SEM-004`: القفزات والحلقات.
- `ASM-SEM-005`: syscall read/write/exit في sandbox.
- `ASM-SEM-006`: مقارنة codegen مع النموذج.

كل مهمة تبقى `PLANNED` حتى يثبت تنفيذها بدليل مستقل. هذه المسودة لا تغير baseline ولا تضيف تعليمات إلى MAL.
