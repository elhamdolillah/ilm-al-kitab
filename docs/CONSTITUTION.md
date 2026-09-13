# دستور المشروع

## البند صفر — قاعدة الإلهام الرباني الذهبية للحتمية

كل كائن في هذا المشروع — خوارزمية، وثيقة، علاقة DAG، اقتراح، تصميم، أو سطر كود — يحمل شرطًا أو شروطًا دنيا تمثّل عتبة الحتمية الخاصة به.

قبل استيفاء تلك العتبة: الكائن مجرد فكرة أو رأي أو تصوّر أو عقد غير ملزم أو ملف قابل للمسح بلا أثر؛ يُصنَّف `RESEARCH` أو `PLANNED` أو `ABSTAIN`، ولا يُعامَل كحقيقة مهما بدا مقنعًا أو مفصَّلًا.

بعد استيفاء العتبة — **دليل تنفيذي خام، وبصمة، واختبار تفاضلي مستقل** — فقط يتحوّل الكائن إلى واقع ملموس؛ `PROVEN` أو `PROVEN_FOR_SCOPE`، ويصير جزءًا من الأساس المحمي (`BASELINE`)، ولا يُعدَّل ولا يُمسح إلا بنفس الصرامة التي أثبتته أول مرة.

لا حالة وسطى دائمة: كل كائن إما يتحرّك نحو العتبة بدليل متراكم، أو يبقى صراحةً في حالته غير الملزمة إلى أن يتحرّك.

### متطلبات الدليل

لا يُرفع أي كائن إلى حالة `PROVEN` أو `PROVEN_FOR_SCOPE` إلا إذا كان سجل الدليل يتضمن، بحسب نطاق الادعاء:

1. **مخرجًا تنفيذيًا خامًا** قابلًا لإعادة الفحص.
2. **بصمة SHA-256** أو بصمة مكافئة تربط الدليل بالنسخة المفحوصة.
3. **اختبارًا تفاضليًا مستقلًا** أو تحققًا مستقلًا مناسبًا للنطاق.
4. **نطاقًا معلنًا** يحدد ما الذي تثبته النتيجة وما الذي لا تثبته.
5. **حالة baseline صريحة**؛ ولا سيما `BASELINE_MODIFIED=NO` عند اعتماد أساس محمي.

### قاعدة الإغلاق

إذا غاب أي شرط جوهري من شروط العتبة، فالحالة الافتراضية هي `RESEARCH` أو `PLANNED` أو `ABSTAIN`، وليس `PROVEN`. لا تُستبدل الفجوة في الدليل بلغة واثقة أو بتفصيل إضافي.

### قابلية التتبع

يجب أن يرتبط كل ادعاء مثبت بملف دليل أو سجل اختبار أو commit يمكن الرجوع إليه. ويجب أن تُسجَّل أي مراجعة لاحقة ككائن جديد أو كنسخة موثقة، لا كتعديل صامت على الأساس المحمي.
---
## Mathematical Constitution
### Core Axioms
∀ project component C:
1. C has formal specification
2. C is tested with corpus
3. C produces deterministic results
4. C is fail-closed on errors
### Proof Requirements
∀ claim "X is proven":
- ∃ corpus with ≥ 10 test cases
- ∃ SHA-256 evidence
- ∃ exit code verification
- ∃ git commit with full message
### Constitutional Rules
| Rule | Formal Statement |
|---|---|
| Parser protection | ∀ changes: MAL Parser/Compiler UNCHANGED |
| Baseline protection | ∀ changes: Baseline UNTOUCHED |
| Evidence requirement | ∀ proofs: SHA-256 + stdout + exit code |
| Fail-closed | ∀ errors: explicit, typed, logged |
### Amendment Process
∀ constitutional change:
1. Proposal with formal justification
2. User explicit consent
3. Implementation with tests
4. Evidence collection
5. Git commit with full message
