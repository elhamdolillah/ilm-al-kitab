# تقرير الدمج الانتقائي — 2026-09-15

## النطاق والقاعدة

أُجريت قراءة متوازية للنسخة القديمة `/root/arabic_math_lang` والنسخة الجديدة `/root/ilm-al-kitab` على الـVPS. لم يُعدَّل كود النسخة القديمة، ولم يُعدَّل Parser أو Compiler أو Arena في النسخة الجديدة. هذا التقرير **تصنيف وقراءة فقط**، وليس قرار دمج تنفيذياً.

> شرط الدمج: لا يُرشَّح أي عنصر قديم للتكامل إلا إذا كان متوافقاً مع Arena/NodeID/Fail-Closed، ولا يتعارض مع `PROVEN_FOR_SCOPE` أو حماية baseline أو `#![forbid(unsafe_code)]`، وله دليل قابل لإعادة الفحص.

## نتيجة الجرد

المستودع القديم موجود في `/root/arabic_math_lang`، والجديد في `/root/ilm-al-kitab`. لم يوجد الملف المتوقع `/root/arabic_math_lang/docs/CONSTITUTION.md` في الشجرة الحالية؛ لذلك لا يمكن إعلان مقارنة بندية للدستور القديم. الدستور المتاح والقابل للتحقق هو `ilm-al-kitab/docs/CONSTITUTION.md`، وفيه متطلبات الدليل الخام والبصمة والاختبار المستقل والنطاق وحالة baseline.

## عناصر مرشحة للدمج المشروط

| العنصر القديم | دليل القراءة | توافقه مع الجديد | الحالة |
|---|---|---|---|
| قاعدة فصل الفكرة عن الحقيقة التنفيذية | حالات `RESEARCH`/`PLANNED`/`ABSTAIN` في تاريخ المشروع، ومبدأ fail-closed القائم | متوافق مباشرة مع البند صفر في دستور KNOB | `CANDIDATE_FOR_DOCUMENTATION_ONLY`؛ لا حاجة لنسخ قاعدة قديمة قبل استعادة نص دستورها الأصلي |
| خوارزمية عربية رمزية ذات مراحل Lexer/Parser/Type/Compiler | `/root/arabic_math_lang/math_complete.py` يحوي مراحل تحليل واستنتاج نوع وتجميع، والجديد يحوي Lexer/Parser/Arena Rust | التكامل المباشر غير مثبت؛ تمثيل AST وNodeID مختلفان | `DEFERRED_UNTIL_ADAPTER` |
| اختبارات السلوك الموجب والسالب | `/root/arabic_math_lang/test_all_phases.py` يحتوي حالات build/run وexpect-error | يمكن تحويل corpus إلى corpus مستقل للجديد، بشرط adapter يحدد التكافؤ ولا يغيّر baseline | `CANDIDATE_AS_RESEARCH_CORPUS` |
| مبادئ الملكية الخطية/فحص المتغيرات الحرة | دوال `check_ownership` و`get_free_vars` في `math_complete.py` | الفكرة متوافقة مع هدف Arena، لكن دلالتها ليست NodeID Rust ولا يوجد إثبات تكافؤ | `DEFERRED_UNTIL_SEMANTIC_MAPPING` |

## عناصر مؤجلة بلا تعارض مثبت

| العنصر | السبب | الحالة المقترحة |
|---|---|---|
| `أُس()`/القوة الثابتة النقطة Q32.32 وادعاءات C40 | يوجد ذكر/تنفيذ رياضي في النسخة القديمة، لكن لا يوجد في هذه القراءة دليل مستقل مكتمل يربطه بدلالات MAL Rust الحالية؛ كما أن Parser الجديد لا يملك runtime حسابياً كاملاً | `DEFERRED_UNTIL_NEEDED` |
| دوال `جذر`, `أرضية`, `قوة`, `مطلق` | اختبارات قديمة ظاهرة في `test_all_phases.py`، لكن الجديد Parser/Arena وليس evaluator متوافقاً | `DEFERRED_UNTIL_RUNTIME_CONTRACT` |
| FFI وChannels وThreads | موجودة كمسارات قديمة/اختبارات أو أمثلة، لكنها تحتاج حدود أمان وعقداً مستقلاً ولا يجوز نقلها إلى Parser | `RESEARCH_ONLY` |
| x86-64/WASM وsyscalls الخام | أمثلة Assembly و`syscall` موجودة في الشجرة القديمة، والجديد لديه أبحاث ASM منفصلة؛ لا يوجد تكامل مثبت ولا يجوز إدخال unsafe أو syscall في Parser | `DEFERRED_ASM_MODEL_ONLY` |
| تغطية 44+ مرحلة | رقم تغطية لا يثبت التكافؤ أو الصحة في بنية MAL الجديدة | `REJECT_AS_PROOF; RETAIN_AS_INVENTORY_ONLY` |
| دعم λ وGenerics | KNOB القديم يصف λ كفجوة، والنسخة الجديدة لا يجوز أن ترث التنفيذ دون عقد grammar/AST/type/runtime واختبارات مستقلة | `DEFERRED_UNTIL_CONTRACT` |

## عناصر مرفوضة حالياً بسبب تعارض أو غياب شرط التكامل

| العنصر | سبب الرفض الدقيق |
|---|---|
| نسخ `math_complete.py` أو دمجه داخل Rust Parser/Arena | خرق لفصل البنية الحالية، ولا يوجد adapter أو إثبات NodeID/AST equivalence؛ كما أن الدستور يحمي Parser/Compiler baseline |
| نقل `eval` أو `network` من أدوات قديمة إلى المسار الحتمي | يتعارض مع قواعد `EVAL_EXEC=DENY` و`NETWORK=DISABLED` الموثقة في KNOB |
| نقل syscalls أو Assembly إلى MAL Parser/Compiler | خارج نطاق parser الحالي ويتطلب نموذجاً مستقلاً؛ قد يخرق `forbid(unsafe_code)` إذا نُفذ بالطريقة القديمة |
| إعلان دقة Q32.32 أو نجاح 44+ مرحلة كحقيقة في KNOB | لا يكفي ادعاء سابق؛ يلزم stdout خام، SHA-256، exit code مستقل، ونطاق معلن داخل النسخة الجديدة |
| نسخ الدستور القديم | الملف `docs/CONSTITUTION.md` القديم غير موجود في الشجرة الحالية، لذا الأصل غير متاح للتحقق ولا يجوز إعادة بنائه من الذاكرة أو دمجه بالثقة |

## مقارنة منهجية للإنجازات المزعومة

### اللغة العربية الرمزية وAOT

النسخة القديمة تحتوي على محلل واستنتاج نوع وتجميع في `math_complete.py`، وهذا دليل على وجود مسار كود، وليس وحده دليلاً على AOT حتمي x86-64 وWASM في النطاق الكامل. الجديد يثبت حالياً أجزاء Parser/Lexer/Arena، لذلك التصنيف الصحيح هو `DEFERRED_UNTIL_CROSS_TARGET_EVIDENCE`.

### `أُس()` والدقة الرياضية

الفكرة ذات قيمة عالية للمشروع، لكن لا تُدمج كحقيقة أو runtime feature قبل استخراج خوارزمية محددة، عقد overflow/rounding، corpus مستقل، ومرجع ثانٍ يعيد البصمة نفسها. حالياً `RESEARCH/` فقط.

### الملكية الخطية

وجود `check_ownership` في القديم مرشح بحثي قوي. التكامل الصحيح ليس نسخ الدالة؛ بل تعريف mapping بين المتغيرات القديمة و`NodeID`/Arena وكتابة اختبارات رفض/قبول مستقلة. لذلك لا دمج تنفيذي الآن.

### التوازي وsyscalls

أمثلة Assembly القديمة تُظهر مادة بحثية، لكنها ليست دليلاً على أن MAL الجديد يجب أن ينفذ syscalls خاماً. تُحفظ كـASM semantic model فقط، وبـfail-closed، ومن دون إدخالها إلى Parser/Compiler.

### الاتساع المرحلي

عدد المراحل القديمة مفيد كفهرس gap registry، لكنه ليس مقياس صحة. كل مرحلة تحتاج عقداً واختباراً وبصمة ونطاقاً قبل `PROVEN_FOR_SCOPE`.

## توصية تنفيذية لـQwen

```text
1. لا تنقل أي ملف قديم مباشرة.
2. أنشئ adapter/corpus فقط في RESEARCH أو branch معزول.
3. لكل مفهوم: اكتب CONTRACT قبل الكود.
4. افحص Arena/NodeID والـAST الحالي فعلياً، ولا تفترض API.
5. شغّل baseline قبل وبعد؛ إذا فشل baseline فتوقف.
6. لا تضف Parser/Compiler behavior حتى يوافق المستخدم على العقد وتنجح الاختبارات.
7. لا تستخدم eval أو network أو unsafe أو syscalls في المسار الحتمي.
8. لا تعلن PROVEN إلا بدليل خام + SHA-256 + اختبار مستقل + نطاق + commit.
9. إذا تعارض القديم مع الجديد، سجل REJECTED_CONFLICT ولا تكيّفه بالقوة.
```

## الحالة الدستورية

```text
BASELINE_MODIFIED_OLD=NO
BASELINE_MODIFIED_NEW=NO  # لا تعديل كودي في جلسة القراءة هذه
OLD_CONSTITUTION_VERIFIED=NO  # الملف غير موجود في الشجرة القديمة الحالية
DIRECT_CODE_MERGE=NO
DOCUMENTARY_REPORT=YES
```

## القرار

لا يوجد دمج فعلي في هذه الجلسة. أفضل ما يمكن حمله بأمان من النسخة القديمة هو **منهج الاختبار، corpus الحالات، ومبادئ الملكية والدقة الرياضية كمواد بحثية**؛ أما التنفيذ، والدستور القديم غير المتاح، وادعاءات AOT/WASM/syscalls، فتظل مؤجلة أو مرفوضة حتى تتوافر أدلة توافق مستقلة.
