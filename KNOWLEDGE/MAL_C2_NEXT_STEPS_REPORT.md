# MAL — تقرير تنفيذ المراحل المتاحة

## النتيجة المختصرة

تمت إعادة التحقق من Parser وLexer وArena، وتشغيل مجموعة `math_complete.py` الخارجية. لا يجوز إعلان اكتمال C1.2 أو C1.3 أو Runtime لأن مكوناتها غير موجودة في البنية الحالية.

## الاختبارات

| المسار | النتيجة | الحالة |
|---|---:|---|
| Parser | 35/35 | `PROVEN_FOR_SCOPE` |
| اختبارات Read | 3/3 | `PROVEN_FOR_SCOPE` |
| Lexer | 9/9 | `PROVEN_FOR_SCOPE` |
| Arena | 7/7، مع 10K | `PROVEN_FOR_SCOPE` |
| math_complete external suite | 10/24 | `FAILED` كاختبار شامل |

المجموعة الخارجية أعادت نجاحات في Threads الأساسية، Lambda/Generics جزئياً، وCore backward compatibility، لكنها فشلت في FFI، Channels، وبعض Generics. كما أن سكربت الاختبار أعاد exit code صفراً رغم وجود 14 فشلاً؛ لذلك لا يُستخدم exit code وحده كدليل نجاح.

## قرارات النطاق

### C2

`PROVEN_FOR_SCOPE`: Concat وRead وPrint في Parser الحالي، ضمن اختبارات Rust المسجلة.

### Differential testing

`PLANNED`: توجد الآن نتيجة توافق خارجية محفوظة، لكنها ليست اختباراً تفاضلياً كاملاً بين AST Parser المحلي و`math_complete.py`. يلزم adapter أو corpus مشترك قبل إعلانها `PROVEN`.

### C1.2 — Set Theory

`ABSTAIN`: Lexer الحالي لا يحتوي Union/Intersection/Difference/Symmetric Difference/Cartesian Product/Subset/Superset. الإضافة ستكون تصميم لغة جديداً، وليست نقلاً مثبتاً من المرجع. تحتاج عقداً، precedence، AST variants، اختبارات موجبة وسالبة، وتوثيقاً قبل التنفيذ.

### C1.3 — Type Theory

`ABSTAIN`: لا توجد حالياً tokens أو AST أو parser لمسارات Π/Σ/type annotation/function type. لا يصح تنفيذها دفعة واحدة قبل عقد دلالي صغير.

### Runtime

`PLANNED`: Parser ينتج AST داخل Arena لكنه لا يوفر runtime/evaluator متكاملاً في هذا المستودع. البداية الآمنة هي evaluator offline محدود للحساب وConcat وPrint/Read، مع اختبار stdin/stdout، لا runtime عام.

## الخطوة التالية الأقل خطراً

1. إنشاء corpus مشترك صغير للميزات المثبتة فقط.
2. إضافة adapter يطبع تمثيل AST حتمياً من Parser المحلي.
3. مقارنة قبول/رفض الحالات مع المرجع، مع فصل اختلافات التنفيذ عن اختلافات اللغة.
4. بعد ذلك بناء evaluator محدود بحدود واضحة.

لا تُعدّل C1.2 أو C1.3 أو Runtime قبل اعتماد عقد النطاق والاختبارات.
