# هندسة نظام MAL
## Pipeline
Arabic source → Lexer → Parser (AST) → Type Checker (7 algorithms) → NIR → x86-64 → ELF
## نظام الأنواع (7 خوارزميات)
1. Hindley-Milner (Algorithm W) - استدلال كامل
2. Union-Find - حل قيود O(a(n))
3. Bidirectional - تحقق ثنائي
4. Flow-Sensitive - تضييق الأنواع
5. Union Types - أنواع الاتحاد
6. Generics - دوال عامة
7. Type Classes - أصناف نوعية
## الأنواع
- أولية: عدد, عشري, منطقي, نص, حرف
- مركبة: قائمة, زوج, سجل, دالة
- متقدمة: اتحاد, forall, traits, refinement, dependent
## Builtins
- رياضية: sqrt, abs, floor, ceil, min, max, round, power, exp
- نصوص: str_len, str_concat
- قوائم: list_len, list_sum
- I/O: print_int, print_str, num_to_str
## Traits المدمجة
- عدد (Numeric)
- قابلة_للطباعة (Printable)
- قابلة_للمقارنة (Comparable)
- قابلة_للجمع (Addable)
## استخدام CLI
  malc برنامج.مال                    # ترجمة عادية
  MAL_CHECK_ONLY=1 malc برنامج.مال   # تحقق فقط
  malc --check-only برنامج.مال       # تحقق فقط (CLI flag)
## الاختبارات
- 11 unit tests في type_inference
- 8 regression tests في cli/tests
- 5 أمثلة في examples/type_system
