# تقرير أتمتة Qwen المحلية

## الحالة

`BLOCKED`

## ما تم تثبيته

تم إنشاء ملف مراحل موحد لـQwen ومشغل آمن يعمل على VPS داخل `/root/ilm-al-kitab`.

المشغل:

```text
tools/qwen_autonomous_cycle.sh
```

ويقوم قبل أي تعديل بما يلي:

1. تشغيل `.qwen_autotest/run_all_tests.sh`.
2. حفظ stdout وSHA-256.
3. إرسال المرحلة الحالية إلى Ollama المحلي.
4. قبول patch فقط بين `PATCH_BEGIN` و`PATCH_END`.
5. منع المسارات الحساسة ومشغل الاختبارات من التعديل.
6. تشغيل الاختبارات بعد patch.
7. عكس patch نفسه عند فشل الاختبارات، دون استعمال `git checkout -- .`.

المساعد:

```text
tools/qwen_json_helper.py
```

وخطة المراحل:

```text
QWEN_REMAINING_PHASES.md
```

## نتيجة التشغيل

تم تشغيل دورة على `TASKS/ACTIVE/SYMBOLS_NEXT.md`.

النتيجة:

```text
STATUS: BLOCKED
REASON: both chat API and ollama CLI failed
```

واجهة Ollama chat لم تُرجع استجابة ضمن 60 ثانية، ثم فشل مسار `ollama run` ضمن 180 ثانية. لم يُطبق أي patch، ولم يُعدل كود MAL.

## قرار fail-closed

بقيت المرحلة دون تنفيذ لأن وكيل Qwen لم يقدم patch صالحاً. لا يجوز اعتبار timeout نجاحاً، ولا تشغيل أوامر مولدة من نموذج غير مستجيب.

## المراحل المحمية

- Phase 1A: Parser Contract.
- Phase 1B: Arithmetic Contract بعد قرار صريح للتمثيل والـoverflow.
- Phase 1C: Comparisons بعد عقد precedence والأنواع.
- Phase 1D: Boolean بعد تحديد literals والدلالات.
- Phase 1E: Sets كنموذج مستقل أولاً.
- ASM-SEM-001 كنموذج offline مستقل.

## الخطوة المطلوبة لاحقاً

تشخيص خدمة Ollama وذاكرة الخادم، ثم إعادة اختبار Qwen بطلب قصير مستقل. لا تبدأ دورة تعديل قبل أن يجيب Qwen ضمن مهلة محددة ويجتاز baseline.
