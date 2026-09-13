# بروتوكول علم الكتاب — ilm al-Kitab

## المبدأ الأساسي
النماذج قابلة للاستبدال. المعرفة ليست كذلك. العمل يجب أن ينجو من النموذج.

## المصدر الوحيد للحقيقة
KNOB.md — أي معلومة غير موجودة فيه تُعتبر غير مُتحقق منها.

## دورة العمل الإلزامية
1. اقرأ KNOB.md
2. حدد الحالة الحالية
3. اختر المهمة ذات الأولوية الأعلى
4. نفّذ
5. تحقّق
6. حدّث KNOB.md
7. التزم في git

## مستويات الدليل
VERIFIED > TESTED > REPRODUCED > PROBABLE > PROPOSED > UNVERIFIED > REJECTED

## قيود دستورية
- RAW_POINTERS=DENY
- EVAL_EXEC=DENY
- NETWORK=DISABLED_BY_CONTRACT (في المسار الحتمي)
- لا ساعة نظام في المسار الحتمي
- حدود ثابتة للذاكرة والعمق والوقود
- ABSTAIN عند الغموض

## تنسيق الاستجابة
كل عمل جوهري يبدأ بـ:
- المزامنة (نقطة التحقق الحالية)
- الخطة
- التنفيذ
- التحقق
- تحديث KNOB
- الإجراء التالي
---
## Mathematical Protocol Rules
### Determinism Axiom
∀ program P, input I:
run(P, I) produces identical output across all executions
### Fail-Closed Axiom
∀ error E:
- E is explicit and typed
- E contains sufficient context for debugging
- No silent failures
### Evidence Axiom
∀ test T:
- T produces (stdout, exit_code, SHA-256)
- SHA-256 is invariant across runs
- Evidence is stored in evidence/ directory
### Constitutional Axioms
1. MAL Parser/Compiler: UNCHANGED unless explicitly authorized
2. Baseline: UNTOUCHED unless explicitly authorized
3. New features: PROVEN_FOR_SCOPE before integration
4. Innovation: requires explicit user consent
