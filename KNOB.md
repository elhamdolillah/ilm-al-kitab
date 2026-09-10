# علم الكتاب — KNOB
## Knowledge of the Book

**الإصدار:** 1.0
**آخر تحديث:** 2026-09-09
**آخر نقطة تحقق مُتحقق منها:** NONE
**الحالة العالمية الحالية:** التأسيس

---

# 1. المهمة

إكمال تطوير:
- **MAL**: اللغة العربية الرياضية (مترجم AOT + مفسر + طبقات ذكاء)
- **UORI**: البنية الموحدة للتشغيل/الزمن الحقيقي/الذكاء
- **UORI+**: البنية الخلفية المطورة

---

# 2. MAL

## الحالة: قيد التطوير النشط

## المكونات الحالية:
| المكوّن | الملف | الحالة الدستورية |
|---------|-------|------------------|
| MiniLexer | arabic_interpreter.py | ✅ مطابق جزئياً |
| MiniParser | arabic_interpreter.py | ✅ مطابق جزئياً |
| MiniInterpreter | arabic_interpreter.py | ✅ مطابق جزئياً |
| طبقة الفصاحة | fasaha.py | ❌ ABSTAIN (eval) |
| المساعد الذكي | smart_assistant.py | ❌ ABSTAIN (eval+network) |
| واجهة Kivy | main.py | ✅ مُسلَّم (Chat UI) |
| Arena Rust | rust/mal_ownership_arena | ⏸️ ABSTAIN (NodeID) |
| مترجم AOT | math_complete.py | ⏸️ λ غير مُنفَّذ |

## المكتمل:
- [x] MiniLexer + MiniParser + MiniInterpreter (نواة تفسيرية)
- [x] واجهة Kivy Chat UI (main.py مُعاد كتابته)
- [x] Arena Rust: 5 اختبارات PASS (نطاق محدود)
- [x] قاعدة معرفة SQLite (KnowledgeBase)

## الجاري:
- [ ] إصلاح Arena بواجهة NodeID(u32)
- [ ] اختبار 10,000 عقدة
- [ ] إضافة دعم λ في math_complete.py
- [ ] تعقيم fasaha.py من eval

## المحظور:
- [ ] MAL_AR_RUNTIME: UNAVAILABLE
- [ ] STAGE0_GATE: ABSTAIN_UNTIL_EVIDENCE

## التالي:
1. إعادة كتابة Arena بـ NodeID + اختبار 10K
2. تعديل compile_stmt_local لدعم λ
3. إنشاء corpus موجب وسالب

---

# 3. UORI

## الحالة: مخطط — لم يبدأ التنفيذ

---

# 4. UORI+

## الحالة: مخطط — لم يبدأ التنفيذ

---

# 5. الحقائق المُتحقق منها

| الحقيقة | الدليل | التاريخ | النموذج |
|---------|--------|---------|---------|
| Arena: 5 اختبارات PASS | cargo test --release | 2026-08-25 | Qwen |
| MiniLexer/Parser/Interpreter: لا eval | فحص كود | 2026-09-10 | Qwen |
| fasaha.py: يحتوي eval() | سطر في نفّذ_جملة_عربية | 2026-09-10 | Qwen |
| smart_assistant.py: eval + network | skill_calculator + chat_completion | 2026-09-10 | Qwen |

---

# 6. الفرضيات غير المُتحقق منها

| الفرضية | السبب | الأولوية |
|---------|-------|----------|
| Arena بدون Vec ممكنة | لم تُختبر | حرجة |
| λ يعمل في math_complete | لم يُنفَّذ | عالية |
| تعقيم eval ممكن بدون فقدان وظيفة | لم يُجرَّب | عالية |

---

# 7. القرارات

| القرار | المبرر | التاريخ | النموذج |
|--------|--------|---------|---------|
| Kotlin + sshj لتطبيق SSH | أداء + Foreground Service | 2026-09-09 | Qwen |
| Kivy Chat UI للواجهة العربية | إعادة استخدام كود موجود | 2026-09-09 | Qwen |
| Rust للـ Arena الحتمية | forbid(unsafe_code) | 2026-08-25 | Qwen |

---

# 8. المناهج المرفوضة

| المنهج | سبب الرفض | التاريخ |
|--------|-----------|---------|
| mal_runner Rust (فحص نصي) | غلاف CLI بدون تنفيذ دلالي | 2026-09-09 |
| eval() في الفصاحة | انتهاك EVAL_EXEC=DENY | 2026-09-09 |
| network في المسار الحتمي | انتهاك NETWORK=DISABLED | 2026-09-09 |

---

# 9. المشاكل المفتوحة

| المشكلة | الأولوية | الحالة | النموذج المكلف |
|---------|----------|--------|----------------|
| Arena: Handle بدل NodeID | حرجة | مفتوحة | غير مُسند |
| لا اختبار 10,000 عقدة | حرجة | مفتوحة | غير مُسند |
| eval في fasaha.py | عالية | مفتوحة | غير مُسند |
| eval + network في smart_assistant | عالية | مفتوحة | غير مُسند |
| λ غير مدعوم في math_complete | عالية | مفتوحة | غير مُسند |

---

# 10. المهام النشطة

#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════
# ilm-al-kitab — سكريبت التأسيس الكامل
# المرحلة 1: إنشاء البنية + الملفات + الرفع
# ═══════════════════════════════════════════════════════════════
set -euo pipefail

# ─── المتغيرات (عدّل هذه الأسطر فقط) ───
GITHUB_USER="YOUR_USERNAME"        # ← استبدل باسمك على GitHub
REPO_NAME="ilm-al-kitab"
REPO_URL="https://github.com/${GITHUB_USER}/${REPO_NAME}.git"
LOCAL_DIR="${HOME}/${REPO_NAME}"
DATE=$(date +%Y-%m-%d)

echo "╔══════════════════════════════════════════════╗"
echo "║   ilm-al-kitab — التأسيس الكامل              ║"
echo "║   ${DATE}                                  ║"
echo "╚══════════════════════════════════════════════╝"

# ─── 1. إنشاء المجلد وتهيئة git ───
echo ""
echo "▶ [1/7] إنشاء المجلد وتهيئة git..."
rm -rf "${LOCAL_DIR}"
mkdir -p "${LOCAL_DIR}"
cd "${LOCAL_DIR}"
git init
git config user.name "ilm-al-kitab"
git config user.email "${GITHUB_USER}@users.noreply.github.com"
git branch -M main

# ─── 2. إنشاء البنية المجلدية ───
echo "▶ [2/7] إنشاء البنية المجلدية..."
mkdir -p TASKS/ACTIVE TASKS/COMPLETED TASKS/ARCHIVED
mkdir -p CHECKPOINTS KNOWLEDGE SESSIONS RESEARCH META
mkdir -p MAL/src MAL/tests MAL/docs
mkdir -p UORI/src UORI/tests UORI/docs
mkdir -p UORI_PLUS/src UORI_PLUS/tests UORI_PLUS/docs
mkdir -p evidence

# ─── 3. إنشاء .gitignore ───
echo "▶ [3/7] إنشاء .gitignore..."
cat > .gitignore << 'EOF'
# أسرار — لا تُرفع أبداً
*.key
*.pem
*.env
.env
secrets/
*.db
*.sqlite3
__pycache__/
*.pyc
*.pyo
target/
node_modules/
*.log
.DS_Store
EOF

# ─── 4. إنشاء KNOB.md ───
echo "▶ [4/7] إنشاء KNOB.md (الدماغ المركزي)..."
cat > KNOB.md << 'KNOBEOF'
# علم الكتاب — KNOB
## Knowledge of the Book

**الإصدار:** 1.0
**آخر تحديث:** 2026-09-09
**آخر نقطة تحقق مُتحقق منها:** NONE
**الحالة العالمية الحالية:** التأسيس

---

# 1. المهمة

إكمال تطوير:
- **MAL**: اللغة العربية الرياضية (مترجم AOT + مفسر + طبقات ذكاء)
- **UORI**: البنية الموحدة للتشغيل/الزمن الحقيقي/الذكاء
- **UORI+**: البنية الخلفية المطورة

---

# 2. MAL

## الحالة: قيد التطوير النشط

## المكونات الحالية:
| المكوّن | الملف | الحالة الدستورية |
|---------|-------|------------------|
| MiniLexer | arabic_interpreter.py | ✅ مطابق جزئياً |
| MiniParser | arabic_interpreter.py | ✅ مطابق جزئياً |
| MiniInterpreter | arabic_interpreter.py | ✅ مطابق جزئياً |
| طبقة الفصاحة | fasaha.py | ❌ ABSTAIN (eval) |
| المساعد الذكي | smart_assistant.py | ❌ ABSTAIN (eval+network) |
| واجهة Kivy | main.py | ✅ مُسلَّم (Chat UI) |
| Arena Rust | rust/mal_ownership_arena | ⏸️ ABSTAIN (NodeID) |
| مترجم AOT | math_complete.py | ⏸️ λ غير مُنفَّذ |

## المكتمل:
- [x] MiniLexer + MiniParser + MiniInterpreter (نواة تفسيرية)
- [x] واجهة Kivy Chat UI (main.py مُعاد كتابته)
- [x] Arena Rust: 5 اختبارات PASS (نطاق محدود)
- [x] قاعدة معرفة SQLite (KnowledgeBase)

## الجاري:
- [ ] إصلاح Arena بواجهة NodeID(u32)
- [ ] اختبار 10,000 عقدة
- [ ] إضافة دعم λ في math_complete.py
- [ ] تعقيم fasaha.py من eval

## المحظور:
- [ ] MAL_AR_RUNTIME: UNAVAILABLE
- [ ] STAGE0_GATE: ABSTAIN_UNTIL_EVIDENCE

## التالي:
1. إعادة كتابة Arena بـ NodeID + اختبار 10K
2. تعديل compile_stmt_local لدعم λ
3. إنشاء corpus موجب وسالب

---

# 3. UORI

## الحالة: مخطط — لم يبدأ التنفيذ

---

# 4. UORI+

## الحالة: مخطط — لم يبدأ التنفيذ

---

# 5. الحقائق المُتحقق منها

| الحقيقة | الدليل | التاريخ | النموذج |
|---------|--------|---------|---------|
| Arena: 5 اختبارات PASS | cargo test --release | 2026-08-25 | Qwen |
| MiniLexer/Parser/Interpreter: لا eval | فحص كود | 2026-09-10 | Qwen |
| fasaha.py: يحتوي eval() | سطر في نفّذ_جملة_عربية | 2026-09-10 | Qwen |
| smart_assistant.py: eval + network | skill_calculator + chat_completion | 2026-09-10 | Qwen |

---

# 6. الفرضيات غير المُتحقق منها

| الفرضية | السبب | الأولوية |
|---------|-------|----------|
| Arena بدون Vec ممكنة | لم تُختبر | حرجة |
| λ يعمل في math_complete | لم يُنفَّذ | عالية |
| تعقيم eval ممكن بدون فقدان وظيفة | لم يُجرَّب | عالية |

---

# 7. القرارات

| القرار | المبرر | التاريخ | النموذج |
|--------|--------|---------|---------|
| Kotlin + sshj لتطبيق SSH | أداء + Foreground Service | 2026-09-09 | Qwen |
| Kivy Chat UI للواجهة العربية | إعادة استخدام كود موجود | 2026-09-09 | Qwen |
| Rust للـ Arena الحتمية | forbid(unsafe_code) | 2026-08-25 | Qwen |

---

# 8. المناهج المرفوضة

| المنهج | سبب الرفض | التاريخ |
|--------|-----------|---------|
| mal_runner Rust (فحص نصي) | غلاف CLI بدون تنفيذ دلالي | 2026-09-09 |
| eval() في الفصاحة | انتهاك EVAL_EXEC=DENY | 2026-09-09 |
| network في المسار الحتمي | انتهاك NETWORK=DISABLED | 2026-09-09 |

---

# 9. المشاكل المفتوحة

| المشكلة | الأولوية | الحالة | النموذج المكلف |
|---------|----------|--------|----------------|
| Arena: Handle بدل NodeID | حرجة | مفتوحة | غير مُسند |
| لا اختبار 10,000 عقدة | حرجة | مفتوحة | غير مُسند |
| eval في fasaha.py | عالية | مفتوحة | غير مُسند |
| eval + network في smart_assistant | عالية | مفتوحة | غير مُسند |
| λ غير مدعوم في math_complete | عالية | مفتوحة | غير مُسند |

---

# 10. المهام النشطة

---

# 11. نقاط التحقق

| نقطة التحقق | المشروع | التاريخ | النموذج | النتيجة |
|-------------|---------|---------|---------|---------|
| STAGE0-ARENA-TEST | MAL | 2026-08-25 | Qwen | PASS (نطاق محدود) |
| KIVY-CHAT-UI | MAL | 2026-09-09 | Qwen | DELIVERED |
| KNOB-INIT | ALL | 2026-09-09 | Qwen | FOUNDATION |

---

# 12. الإجراء الأفضل التالي

**الأولوية القصوى:** إعادة كتابة Arena بواجهة `NodeID(u32)` + اختبار 10,000 عقدة.

---

# 13. تعليمات الاسترداد

إذا اختفت الجلسة الحالية:
1. اقرأ هذا الملف
2. اقرأ آخر نقطة تحقق في CHECKPOINTS/
3. افحص المستودع: `git log --oneline -10`
4. شغّل الاختبارات الموجودة
5. استمر من الإجراء الأفضل التالي

---

# 14. الجلسة الأخيرة

**النموذج:** Qwen
**المهمة:** تأسيس KNOB
**النتيجة:** تم إنشاء البنية الأولية
**التالي:** تنفيذ أول مهمة (MAL-ARENA-001)

---
# 16. أدوات التنفيذ المتاحة (Execution Tools)

| الأداة | النموذج | الاستخدام الأمثل | الحالة |
|--------|---------|------------------|--------|
| Hermes CLI | hermes-qwen-code (1.5b محلي) | قراءة KNOB، تصنيف، قرارات خفيفة | ✅ مُفعَّل |
| aider | cerebras/gpt-oss-120b (سحابي مجاني) | كتابة Rust، تعديل كود معقد، Arena, Parser | ✅ مُفعَّل |
| Ollama API | qwen2.5:1.5b | استدعاءات مباشرة عند الحاجة | ✅ مُفعَّل |

## قاعدة التوزيع
- مهام < 50 سطر كود + استدلال بسيط → Hermes/Qwen
- مهام > 50 سطر كود + استدلال معقد → aider/Cerebras
- المهام الحرجة (MAL-ARENA-001, MAL-COMPILER-042) → aider فقط
