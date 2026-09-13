# Qwen — خطة تنفيذ المراحل المتبقية لمشروع ilm-al-kitab

## الدور

أنت وكيل تطوير محلي يعمل على VPS CPU-only. نفّذ مرحلة واحدة فقط في كل دورة، ولا تعلن النجاح إلا بدليل خام وSHA-256.

## ترتيب المراحل

### Phase 1A — Parser Contract

- اقرأ `AGENT_CONTRACT.md` و`TASKS/ACTIVE/SYMBOLS_NEXT.md`.
- شغّل baseline قبل أي تعديل.
- ثبّت فقط السلوك الموثق: AST للحساب، direct calls، precedence، وfail-closed.
- لا تضف رمزاً أو AST variant أو compiler behavior غير منصوص عليه.
- شغّل `cargo fmt --check` والاختبارات و`.qwen_autotest/run_all_tests.sh`.

### Phase 1B — Arithmetic Contract

لا تبدأ إلا إذا مرّت Phase 1A. أضف أصغر مجموعة متفق عليها فقط:

```text
- ثم / أو + حسب ما يثبته الكود الحالي
```

قبل أي رمز جديد، أنشئ عقداً يحدد precedence وassociativity والتمثيل العددي وoverflow وdivision-by-zero. عند غياب قرار واضح: اكتب `PHASE-BLOCKED.md` وتوقف.

### Phase 1C — Comparisons

لا تبدأ إلا بعقد صريح لـ`< > ≤ ≥ == !=`، واختبارات lexer/parser/compiler مستقلة. لا تستنتج معنى الرموز من الوثائق وحدها.

### Phase 1D — Boolean

لا تبدأ إلا بعد تحديد literals وcross-type equality وprecedence للـ`¬ ∧ ∨ ⊻`. أي غموض يوقف المرحلة.

### Phase 1E — Sets

نفّذها أولاً كنموذج بحثي مستقل، لا في MAL Parser، حتى يثبت universe وhashability وcanonical output. العمليات: `∪ ∩ ∖ Δ × ⊆ ⊇`.

### Phase ASM-SEM-001

ابنِ model دلالي offline مستقل فقط للتعليمات:

```text
mov add sub push pop cmp test
```

استعمل `TASKS/ACTIVE/ASM-SEM-001.md` و`KNOWLEDGE/ASM-SEM-001-CORPUS.md`. لا تعدّل Parser أو Compiler MAL.

## قواعد حتمية

1. baseline أولاً؛ إذا فشل لا تعدّل.
2. مرحلة واحدة فقط لكل دورة.
3. لا `rm -rf`، لا `git reset --hard`، لا force push، لا apt/pip/network/firewall/SSH.
4. لا تحذف اختباراً موجوداً.
5. لا تستخدم `unsafe` في Rust.
6. كل تعديل صغير ومراجع بـ`git diff --check`.
7. عند الفشل: احفظ stdout/stderr، SHA-256، واكتب `TASKS/ACTIVE/PHASE-BLOCKED.md`.
8. عند النجاح: شغّل كل gates، ثم اكتب تقريراً في `TASKS/COMPLETED/` ولا تبدأ المرحلة التالية تلقائياً.
9. لا تغيّر `.qwen_autotest/run_all_tests.sh`.
10. لا تعدّل كود MAL في Phase ASM-SEM-001.

## بروتوكول الاستجابة

أعد بالترتيب:

```text
STATUS: PASS|BLOCKED|FAIL
PHASE: <name>
CHANGED_FILES: <list>
TESTS: <exact results>
EVIDENCE: <paths and sha256>
NEXT: <next phase or human decision>
```

إذا لم يوجد قرار بشري مطلوب، نفّذ المرحلة الحالية حتى نهايتها ثم توقف آلياً.
