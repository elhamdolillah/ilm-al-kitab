# MAL-KNOB — قاعدة معرفة مشروع MAL
تنبيه: هذا الملف يحتوي على قسمين:
1. قسم MAL (أدناه) — قاعدة المعرفة الرسمية
2. قسم التراث (بعد الفاصل) — الوثائق التاريخية
---
## المبادئ الدستورية (11 مبدأً)
| # | المبدأ | English |
|---|--------|---------|
| 1 | الإحكام | Precision |
| 2 | التيسير | Simplicity |
| 3 | العدل | Balance |
| 4 | الأمانة | Ownership |
| 5 | البيان | Transparency |
| 6 | الحفظ | Safety |
| 7 | التفكر | Verifiability |
| 8 | الشمولية التقنية | Technical Universality |
| 9 | الوحدة الدلالية | Semantic Unity |
| 10 | الشمولية المعرفية | Epistemic Inclusivity |
| 11 | الأولوية الرياضية | Mathematical Primacy |
---
## رؤية MAL: الوسيط الرياضي الموحد
MAL ليست لغة برمجة أخرى، بل:
- وسيط رياضي بين الإنسان والآلة
- طبقة توحيد بين لغات البرمجة
- صيغة مضغوطة للمعنى الدلالي
المخطط:
  لغات طبيعية (عربي/إنجليزي)
          | AI Translation
          v
    MAL Source (رياضي)
          | Parser
          v
    MAL-IR (SSA + Sigma)
          | Backend
    +-----+-----+
    v     v     v
   x86  ARM   WASM
---
## القاعدة الذهبية (المبدأ 11)
في كل مقترح: افحص التكرار، ابحث عن Sigma، قدّم الرموز الرياضية.
مبرهنة الضغط الدلالي:
  |Sigma_math| <= |syntax_code| <= |text_natural|
---
## حالة MAL (2026-09-16)
| المكون | الاختبارات |
|--------|-----------|
| lexer | 13 |
| arena | 7 |
| parser | 74 |
| ownership | 5 |
| typecheck | 5 |
| pattern | 5 |
| index | 12 |
| kb | 5 |
| nir | 5 |
| adt | 5 |
| المجموع | 136 |
---
## سجل المقترحات
| ID | المقترح | المقابل الرياضي | الحالة |
|----|---------|----------------|--------|
| P-001 | Feature Registry | F = {f_i} | OK |
| P-002 | Semantic Bridge | phi: AST -> FeatureID | OK |
| P-003 | Knowledge Base | K = (R, G, V) | OK |
| P-004 | MAL-NIR | SSA + Sigma | OK |
| P-005 | إلغاء كل الطبقات | مستحيل (Rice) | REJECTED |
| P-006 | الأولوية الرياضية | |Sigma| <= |syntax| | OK |
---
## الفاصل: الوثائق التاريخية
ما يلي هو المحتوى الأصلي لـ KNOB.md (مشروع UORI السابق).
يُحفظ للأرشفة والمرجع التاريخي.
---
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

---
# 18. بروتوكول استلام الأوامر من المحادثات (Code Intake Protocol)

**الحالة:** ✅ VERIFIED (2026-09-11)
**الدليل:** اختبار Termux النص → 4 كتل مستخرجة، 0 تنفيذ غير مقصود

## المبدأ
أي نص ملصق من محادثة LLM (Termux، Qwen، Claude، إلخ) يُعتبر **بيانات غير موثوقة** حتى يمر بثلاث طبقات:

### الطبقة 1: الاستخراج الحتمي (إلزامي)
```bash
python3 /root/.hermes/extract_termux_code.py \
  /root/incoming.txt \
  --output-dir /root/extracted_code \
  --json /root/extracted_manifest.json
```
- لا تنفيذ.
- لا git.
- لا curl.
- فقط فصل الكود عن الشرح.

### الطبقة 2: المراجعة البشرية (إلزامي)
```bash
cat /root/extracted_manifest.json | jq '.blocks[] | {step, language, sha256, requires_confirmation}'
sed -n '1,80p' /root/extracted_code/step-NN.sh
```
- كل كتلة تُقرأ قبل تنفيذها.
- الكتل المعلمة `requires_confirmation: true` تتطلب موافقة صريحة.

### الطبقة 3: التنفيذ المقيد (بعد الموافقة فقط)
```bash
bash /root/extracted_code/step-NN.sh 2>&1 | tee evidence/step-NN.stdout
sha256sum evidence/step-NN.stdout > evidence/step-NN.sha256
```

## ما يُحظر صراحة
- ❌ لصق أوامر مباشرة في الطرفية من محادثة LLM.
- ❌ `curl | sh` أو `wget | bash`.
- ❌ `git push --force` أو `git reset --hard` من نص مستخرج.
- ❌ السماح لـ Hermes/Qwen 1.5B باتخاذ قرار تنفيذ نيابة عنك.

## دور Hermes/Qwen في هذا البروتوكول
- ✅ تحليل الملفات المستخرجة القصيرة.
- ✅ شرح كود غامض.
- ✅ اقتراح اختبارات.
- ❌ **لا** استخراج أوامر من محادثات طويلة.
- ❌ **لا** تنفيذ أوامر مباشرة.

## القرار الدستوري
`CODE_INTAKE_PROTOCOL = ENFORCED`
`LLM_AS_EXECUTOR = REJECTED`
`DETERMINISTIC_PARSER = PRIMARY`

---
# 18. بروتوكول استلام الأوامر من المحادثات (Code Intake Protocol)

**الحالة:** ✅ VERIFIED (2026-09-11)
**الدليل:** اختبار Termux النص → 4 كتل مستخرجة، 0 تنفيذ غير مقصود

## المبدأ
أي نص ملصق من محادثة LLM (Termux، Qwen، Claude، إلخ) يُعتبر **بيانات غير موثوقة** حتى يمر بثلاث طبقات:

### الطبقة 1: الاستخراج الحتمي (إلزامي)
```bash
python3 /root/.hermes/extract_termux_code.py \
  /root/incoming.txt \
  --output-dir /root/extracted_code \
  --json /root/extracted_manifest.json
```
- لا تنفيذ.
- لا git.
- لا curl.
- فقط فصل الكود عن الشرح.

### الطبقة 2: المراجعة البشرية (إلزامي)
```bash
cat /root/extracted_manifest.json | jq '.blocks[] | {step, language, sha256, requires_confirmation}'
sed -n '1,80p' /root/extracted_code/step-NN.sh
```
- كل كتلة تُقرأ قبل تنفيذها.
- الكتل المعلمة `requires_confirmation: true` تتطلب موافقة صريحة.

### الطبقة 3: التنفيذ المقيد (بعد الموافقة فقط)
```bash
bash /root/extracted_code/step-NN.sh 2>&1 | tee evidence/step-NN.stdout
sha256sum evidence/step-NN.stdout > evidence/step-NN.sha256
```

## ما يُحظر صراحة
- ❌ لصق أوامر مباشرة في الطرفية من محادثة LLM.
- ❌ `curl | sh` أو `wget | bash`.
- ❌ `git push --force` أو `git reset --hard` من نص مستخرج.
- ❌ السماح لـ Hermes/Qwen 1.5B باتخاذ قرار تنفيذ نيابة عنك.

## دور Hermes/Qwen في هذا البروتوكول
- ✅ تحليل الملفات المستخرجة القصيرة.
- ✅ شرح كود غامض.
- ✅ اقتراح اختبارات.
- ❌ **لا** استخراج أوامر من محادثات طويلة.
- ❌ **لا** تنفيذ أوامر مباشرة.

## القرار الدستوري
`CODE_INTAKE_PROTOCOL = ENFORCED`
`LLM_AS_EXECUTOR = REJECTED`
`DETERMINISTIC_PARSER = PRIMARY`

---
# Checkpoint: VPS-REMOTE-SYNC

- **التاريخ:** 2026-09-11
- **النموذج:** Qwen (تخطيط) + VPS (تنفيذ)
- **الحالة:** ✅ VERIFIED
- **SHA-256 HEAD:** `1db8cc5d710776fb105d2e1b4e68e5b11105dd21`
- **التغييرات:**
  - إصلاح SSH alias: `github-kitab` → `id_ed25519_kitab`
  - فصل مفتاح الحساب السيادي عن المفاتيح الأخرى
  - rebase آمن بدون `--force` (احترام DEC-004)
  - Git تجاوز commit مكرر تلقائياً (6b35ad8)
- **الدليل:** مزامنة VPS ↔ GitHub (SHA-256 متطابق)
- **القرار:** `VPS_REMOTE_SYNC = PASSED`
- **البروتوكول المُحترم:** لا `--force`، لا `reset --hard`، حفظ backup branch

---
# Checkpoint: MAL-ARENA-001 (STAGE0_ARENA_GATE)

- **التاريخ:** 2026-09-11
- **النموذج المنفِّذ:** Qwen 3.8 (تصميم كود) + VPS (تنفيذ حتمي)
- **الحالة:** ✅ VERIFIED
- **SHA-256 للمخرج الخام:** `dae49e181117cf0d61218f8629c9f5679a101040eb4c76c4c4adbee42f1db9e5`
- **Exit code:** 0
- **الاختبارات الناجحة:** 6/6
  1. `test_basic_allocation`
  2. `test_invalid_id_rejected`
  3. `test_capacity_exceeded_abstain`
  4. `test_type_mismatch_abstain`
  5. `test_stale_handle_rejected`
  6. `test_10k_nodes_stress` (10,000 عقدة متتالية في 0.00s)

## التغييرات الدستورية
- ✅ `#![forbid(unsafe_code)]`
- ✅ `NodeID(u32)` كـ newtype (استبدال `Handle` القديم)
- ✅ `Box<[ASTNode]>` بدلاً من `Vec` (تخصيص ثابت لمرة واحدة)
- ✅ `ArenaError`: `CapacityExceeded`, `InvalidNodeID`, `TypeMismatch`
- ✅ Fail-Closed: جميع حالات الفشل تُرجع `Err`، لا panic

## الملفات المُضافة
- `MAL/src/arena/Cargo.toml` (80 bytes)
- `MAL/src/arena/src/lib.rs` (271 سطرًا، 8208 bytes)
- `evidence/MAL-ARENA-001.stdout` (727 bytes)
- `evidence/MAL-ARENA-001.sha256` (96 bytes)
- `evidence/MAL-ARENA-001-CHECK.stdout`
- `evidence/MAL-ARENA-001-CHECK.sha256`

## القرار الدستوري
```
STAGE0_ARENA_GATE         = PASSED
STAGE0_LOCAL_ARENA_RESULT = PASS
MAL_AR_RUNTIME            = RESEARCH (يحتاج lexer/parser)
BASELINE_MODIFIED         = YES (Arena جديد مُثبت)
AUTO_PROMOTION            = ALLOW_TO_STAGE1
```

## الانتقال للمهمة التالية
`MAL-COMPILER-042`: إضافة دعم λ في `math_complete.py`
  - الحالة: HIGH PRIORITY
  - المُنفَّذ الموصى به: aider أو تنفيذ يدوي

---
# Checkpoint: MAL-COMPILER-LAMBDA (λ Support Verification)

- **التاريخ:** 2026-09-11
- **النموذج المنفِّذ:** Qwen 3.8 (تحليل) + VPS (تنفيذ)
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256 للمخرج الخام:** `b44a2675eb91968d81b03903a7a8379233a729a311ff55fcd1f4eccc9efae2b8`

## الاكتشاف الحاسم
دعم λ **مكتمل بالفعل** في `math_complete.py` (840 سطرًا):
- Parser: `("دالة", params, body)` (سطر 169)
- Closure generation: arena + label داخلي (سطور 458-493)
- Indirect call: `call r10` (سطر 620)
- System V calling convention: `ARG_REGS = ["rdi","rsi","rdx","rcx","r8","r9"]`

## الاختبار المُثبت
```mal
مضروب ≡ λن. (ن = 1) ؟ 1 : ن · مضروب(ن - 1)
⎕ مضروب(5)
```
**الناتج المتوقع:** `120` (closure متكرر)

## الملفات المُضافة
- `MAL/src/compiler/math_complete.py` (40KB, 840 سطرًا)
- `evidence/MAL-COMPILER-LAMBDA.stdout`
- `evidence/MAL-COMPILER-LAMBDA.sha256`

## القرار الدستوري
```
MAL-COMPILER-042        = CANCELLED (λ support already exists)
LAMBDA_SUPPORT          = PROVEN_FOR_SCOPE (recursive closures)
NEXT_TASK               = MAL-LEXER-PARSER-001 (lexer/parser حتمي)
```

## المهام الملغاة
- ~~`MAL-COMPILER-042`: إضافة دعم λ~~ (موجود بالفعل)

---
# Checkpoint: MAL-LEXER-001 (Deterministic Lexer)

- **التاريخ:** 2026-09-11
- **النموذج المنفِّذ:** Qwen 3.8 (تصميم كود) + VPS (تنفيذ حتمي)
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256 للمخرج الخام:** `762a5a3a3d7c8c95ceb705a46ea56285f844631ea2ffd205ad719414df21e3e3`
- **Exit code:** 0
- **الاختبارات الناجحة:** 9/9 (0.00s)

## نطاق الادعاء (Declared Scope)
**ما يُثبته الـ Lexer:**
- تقسيم رموز حتمي لمجموعة MAL (أرقام ASCII، نصوص ، معرفات Unicode، جدول رموز ثابت)
- Fail-Closed: أي محرف خارج اللغة → `LexerError` (ليس panic)
- تتبع أسطر/أعمدة دقيق للرسائل التشخيصية
- نطاقات بايت (لا تخصيص نص لكل رمز)

**ما لا يُثبته (خارج النطاق):**
- لا Parser بعد (المرحلة التالية)
- لا دلالات ولا أنواع
- لا أرقام عربية-هندية (٠١٢٣) — ASCII فقط مطابقةً للمرجع

## الاختبارات الدستورية
1. `test_tokenize_assignment` — تقسيم أساسي
2. `test_tokenize_lambda_define` — رموز λ و ≡ و ·
3. `test_tokenize_print_call` — ⎕ واستدعاء دالة
4. `test_tokenize_block_and_list` — ﴿ ﴾ و ⋄ و ⟨ ⟩ و ،
5. `test_ident_and_str_ranges` — نطاقات بايت صحيحة
6. `test_line_col_tracking` — تتبع متعدد الأسطر
7. `test_unterminated_string_abstain` — Fail-Closed للنص غير المغلق
8. `test_unexpected_char_abstain` — Fail-Closed للمحرف المرفوض
9. `test_number_overflow_abstain` — Fail-Closed لفيضان i64

## التغييرات الدستورية
- ✅ `#![forbid(unsafe_code)]`
- ✅ لا تخصيص heap لكل رمز (نطاقات بايت فقط)
- ✅ لا زمن، لا عشوائية، لا شبكة
- ✅ Fail-Closed: جميع الأخطاء `LexerError`، لا panic

## الملفات المُضافة
- `MAL/src/lexer/Cargo.toml` (202 bytes)
- `MAL/src/lexer/src/lib.rs` (357 سطرًا، 10418 bytes)
- `evidence/MAL-LEXER-001.stdout`
- `evidence/MAL-LEXER-001.sha256`

## القرار الدستوري
```
MAL-LEXER-001           = PROVEN_FOR_SCOPE
LEXER_FAIL_CLOSED       = VERIFIED (3/3 abstain tests passed)
NEXT_TASK               = MAL-PARSER-001 (parser عودي نزولي يبني AST في Arena)
```

---

## ⚠️ تنبيه دستوري — MAL-PARSER-001 (2026-09-11)

**الخطأ:** تم الـ commit `16d2204` بعنوان "✅ MAL-PARSER-001" بينما كانت الاختبارات فاشلة (2/9).

**التصنيف:** FAKE_PROGRESS — مخالفة للدستور.

**الإجراء التصحيحي:**
1. ✅ commit message صُحّح إلى WIP
2. ⏳ إصلاح الاختبارات الفاشلة الـ 7
3. ⏳ إعادة الـ commit بعنوان ✅ فقط بعد نجاح 9/9
4. ⏳ تسجيل الخطأ في قسم REJECTED APPROACHES لمنع تكراره

**الاختبارات الفاشلة:**
- test_parse_number
- test_parse_binary_add
- test_parse_binary_mul
- test_parse_precedence
- test_parse_parens
- test_parse_assignment
- test_parse_error_unexpected_eof

**الاختبارات الناجحة (2):**
- test_parse_block
- (آخر غير مذكور)

---

## ✅ MAL-PARSER-001 — PROVEN_FOR_SCOPE (2026-09-11)

**الحالة:** VERIFIED  
**النطاق:** Parser عودي نزولي يبني AST في Arena (بدون print/λ في هذه المرحلة)  
**الدليل:** `evidence/MAL-PARSER-001.stdout`  
**الاختبارات:** 9/9 نجحت  
**`lib.rs` SHA-256:** يُملأ من `evidence/MAL-PARSER-001.lib.sha256`  
**`stdout` SHA-256:** يُملأ من `evidence/MAL-PARSER-001.sha256`  
**commit:** التالي بعد هذا الـ commit  

**الإصلاحات المُثبتة:**
1. `parse_program` يُعيد آخر stmt مباشرةً عند وجود stmt واحد (بدل تغليفه في List).
2. `parse_primary` يعالج `TokenKind::Eof` بـ `UnexpectedEof` بدل `Expected`.

**الاختبارات الفاشلة سابقاً — الآن ناجحة:**
- test_parse_number ✅
- test_parse_binary_add ✅
- test_parse_binary_mul ✅
- test_parse_precedence ✅
- test_parse_parens ✅
- test_parse_assignment ✅
- test_parse_error_unexpected_eof ✅
- test_parse_block ✅ (كان ناجحاً)
- test_parse_error_expected_paren ✅ (كان ناجحاً)

**ملاحظة:** الطباعة (⎕) واللامدا (λ) لم تُختبَرا في هذه المرحلة. ستُعالَجا في MAL-PARSER-002.

---

## ❌ REJECTED APPROACH — تعديل Parser بالـ sed

**التاريخ:** 2026-09-11  
**الخطأ:** استخدام `sed -i` لتعديل `src/lib.rs` أدى إلى تكرار سطور و`unclosed delimiter` عند السطر 399.  
**الدرس:** كل تعديل على ملفات Rust يجب أن يكون عبر Python (regex + brace-matching) أو محرر تفاعلي، **لا sed**.  
**القاعدة الجديدة:** `DEC-005 — لا sed على ملفات Rust`.

---
# Checkpoint: MAL-PARSER-002 (Lambda in Rust Parser)

- التاريخ: 2026-09-11
- الحالة: PROVEN_FOR_SCOPE
- SHA-256: `deedcce4668d2eba9b7e170a9de93e1e96e5b27dd99c51c6b4b5373f70750a24`
- الاختبارات: 13/13 (9 أساس + 4 lambda)
- تعديل BASELINE: Arena v2 (Lambda variant) مثبت في evidence/MAL-ARENA-001-v2.stdout
- ملاحظة: مسار Python (math_complete.py) مغلق؛ لا patch عليه (MAL-COMPILER-042 CANCELLED)

---
# 17. تكامل MAL وUORI/UORI+ وتوجيه LLM

## الحالة الحالية

- `arabic-math-lang` هو المرجع الخارجي لـMAL، وليس مصدراً للنسخ الأعمى.
- `MAL` داخل هذا المستودع هو مساحة التكامل والاختبار.
- `UORI` و`UORI_PLUS` ما زالا في نطاق التخطيط ما لم توجد أدلة تنفيذ مستقلة.
- مهمة التتبع: `TASKS/ACTIVE/KNOB-MAL-INTEROP-001.md`.
- سياسة التكامل: `KNOWLEDGE/PROJECT_INTEROPERABILITY.md`.
- توجيه Hermes/Qwen: `KNOWLEDGE/LLM_GUIDANCE.md`.

## نتائج فحص البيئة

- remote مستودع `arabic-math-lang` يستخدم `github-kitab` بعد إزالة رمز وصول كان مكشوفاً.
- فحص الأدلة أظهر تطابق معظم بصمات stdout، مع ملف Parser قديم غير مطابق للملف الحالي؛ لا يُستخدم كدليل جديد.
- فحص Rust في جلسة التوثيق كان `ENVIRONMENT_BLOCKED` لأن `cargo` غير موجود في PATH.

## قاعدة الإثبات

لا تُرفع حالة أي تكامل إلى `PROVEN_FOR_SCOPE` إلا بعد stdout خام، SHA-256 مطابق، اختبار مستقل، نطاق معلن، وcommit قابل للتتبع. لا يعتمد النموذج اللغوي الحالة الدستورية ولا يعلن نجاحاً لم يُفحص.
---
# ✅ Checkpoint: MAL-PARSER-002 (Function Application — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-12
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `b766ec356178f3cc703ee4e18510d533624916977b70f6522ae66f04c1eee482`
- **الاختبارات:** 13/13 (9 أساس + 4 lambda)
- **Exit code:** 0
- **التصحيح الدستوري:** هذا الـ checkpoint يصحح الادعاء الكاذب من `ea7979d` الذي سجّل `13/13` بينما الحقيقة كانت `12/13` مع فشل `test_parse_lambda_apply`
## النطاق المُثبت
- دعم `(expr)(args)` في ذراع `TokenKind::LParen` بـ `parse_primary`
- بناء `ASTNode::Call { func: expr, args: args_node }` عند وجود أقواس متعددة
- معالجة الوسائط: `NodeID::INVALID` (فارغ)، `args[0]` (وسيط واحد)، `List` (متعدد)
- لا تراجع في الـ 12 اختباراً الأخرى
## التغييرات
- `MAL/src/parser/src/lib.rs`: إضافة حلقة `while peek == LParen` في ذراع LParen (25 سطراً مضافاً)
- `evidence/MAL-PARSER-002-r2.stdout`: الدليل الخام
- `evidence/MAL-PARSER-002-r2.sha256`: البصمة
- نسخة احتياطية: `lib.rs.bak_<timestamp>`
## القرار الدستوري
القرار الدستوري:
- MAL-PARSER-002         = PROVEN_FOR_SCOPE
- FUNCTION_APPLICATION   = PROVEN (صيغة (expr)(args))
- BASELINE_MODIFIED      = YES (ذراع واحد في parse_primary)
- NO_REGRESSION          = VERIFIED (12/12 اختبار سابق نجح)
## الانتقال للمهمة التالية
الخطوة التالية: `MAL-INTEROP-001` المرحلة B (عقد واجهة) أو `MAL-PARSER-003` (دعم `expr(args)` بدون أقواس)
---
# ✅ Checkpoint: MAL-INTEROP-001-C1.1 (Linear Logic + Quantifiers — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-13
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `5543720812b0ae9e7398a9b224f6e70e2dd3a0550655a621ff080e2e9324f6f9`
- **الاختبارات:** 23/23 (13 قديم + 10 جديد)
- **Exit code:** 0
## النطاق المُثبت
### Linear Logic (Priority 1 — Critical)
- إضافة token `LinearImplication` (⊸) إلى Lexer
- إصلاح ربط `⊸`: أصبح يشير إلى `LinearImplication` بدلاً من `Move`
- إضافة variant `LinearLet` إلى ASTNode
- اختباران API لـ LinearLet (بنية + TypeTag)
### Quantifiers (Priority 2 — High)
- إضافة variant `ForAll` إلى ASTNode
- إضافة `parse_forall()` لصيغة `∀x ∈ S : body`
- إصلاح خطأ double-bump في `parse_forall()`
- 3 اختبارات: basic, nested, with body
### Fixed-Point (Priority 3 — High)
- إضافة variant `Mu` إلى ASTNode
- إضافة `parse_mu()` لصيغة `μx. body`
- إصلاح خطأ double-bump في `parse_mu()`
- 3 اختبارات: basic, with body, with condition
### Set Membership (Priority 2 — High)
- إضافة variant `SetMembership` إلى ASTNode
- اختبار API واحد لـ SetMembership
### TypeTag System
- إضافة 4 TypeTag variants جديدة
- تحديث `type_tag()` لمعالجة الـ variants الجديدة
## ما لا يُثبته هذا الـ checkpoint
⚠️ Set literals (⟨1,2,3⟩) غير مدعومة بعد في Parser (الاختبارات تستخدم identifiers)
⚠️ `LinearLet` غير مدمج بعد في expression grammar الكامل
⚠️ `SetMembership` غير مدمج بعد في expression grammar الكامل
⚠️ لم يُجرَ differential testing ضد `math_complete.py` بعد
⚠️ `Exists` (∃) معلن في العقد لكن لم يُنفَّذ بعد
## التغييرات
- `MAL/src/lexer/src/lib.rs`: إضافة `LinearImplication` token + إصلاح ربط `⊸`
- `MAL/src/arena/src/lib.rs`: إضافة 4 variants إلى `ASTNode` + 4 variants إلى `TypeTag`
- `MAL/src/parser/src/lib.rs`: إضافة `parse_forall()` و `parse_mu()` + إصلاح double-bump + 10 اختبارات جديدة
- `evidence/MAL-PARSER-C1-1.stdout`: الدليل الخام (23 اختبار ناجح)
- `evidence/MAL-PARSER-C1-1.sha256`: البصمة
## القرار الدستوري
---
# ✅ Checkpoint: MAL-INTEROP-001-C1.1 (Linear Logic + Quantifiers — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-13
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `5543720812b0ae9e7398a9b224f6e70e2dd3a0550655a621ff080e2e9324f6f9`
- **الاختبارات:** 23/23 (13 قديم + 10 جديد)
- **Exit code:** 0
## النطاق المُثبت
### Linear Logic (Priority 1 — Critical)
- إضافة token `LinearImplication` (⊸) إلى Lexer
- إصلاح ربط `⊸`: أصبح يشير إلى `LinearImplication` بدلاً من `Move`
- إضافة variant `LinearLet` إلى ASTNode
- اختباران API لـ LinearLet (بنية + TypeTag)
### Quantifiers (Priority 2 — High)
- إضافة variant `ForAll` إلى ASTNode
- إضافة `parse_forall()` لصيغة `∀x ∈ S : body`
- إصلاح خطأ double-bump في `parse_forall()`
- 3 اختبارات: basic, nested, with body
### Fixed-Point (Priority 3 — High)
- إضافة variant `Mu` إلى ASTNode
- إضافة `parse_mu()` لصيغة `μx. body`
- إصلاح خطأ double-bump في `parse_mu()`
- 3 اختبارات: basic, with body, with condition
### Set Membership (Priority 2 — High)
- إضافة variant `SetMembership` إلى ASTNode
- اختبار API واحد لـ SetMembership
### TypeTag System
- إضافة 4 TypeTag variants جديدة
- تحديث `type_tag()` لمعالجة الـ variants الجديدة
## ما لا يُثبته هذا الـ checkpoint
⚠️ Set literals (⟨1,2,3⟩) غير مدعومة بعد في Parser (الاختبارات تستخدم identifiers)
⚠️ `LinearLet` غير مدمج بعد في expression grammar الكامل
⚠️ `SetMembership` غير مدمج بعد في expression grammar الكامل
⚠️ لم يُجرَ differential testing ضد `math_complete.py` بعد
⚠️ `Exists` (∃) معلن في العقد لكن لم يُنفَّذ بعد
## التغييرات
- `MAL/src/lexer/src/lib.rs`: إضافة `LinearImplication` token + إصلاح ربط `⊸`
- `MAL/src/arena/src/lib.rs`: إضافة 4 variants إلى `ASTNode` + 4 variants إلى `TypeTag`
- `MAL/src/parser/src/lib.rs`: إضافة `parse_forall()` و `parse_mu()` + إصلاح double-bump + 10 اختبارات جديدة
- `evidence/MAL-PARSER-C1-1.stdout`: الدليل الخام (23 اختبار ناجح)
- `evidence/MAL-PARSER-C1-1.sha256`: البصمة
## القرار الدستوري
---
# ✅ Checkpoint: MAL-INTEROP-001-C1.1 (Linear Logic + Quantifiers — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-13
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `5543720812b0ae9e7398a9b224f6e70e2dd3a0550655a621ff080e2e9324f6f9`
- **الاختبارات:** 23/23 (13 قديم + 10 جديد)
- **Exit code:** 0
## النطاق المُثبت
### Linear Logic (Priority 1 — Critical)
- إضافة token `LinearImplication` (⊸) إلى Lexer
- إصلاح ربط `⊸`: أصبح يشير إلى `LinearImplication` بدلاً من `Move`
- إضافة variant `LinearLet` إلى ASTNode
- اختباران API لـ LinearLet (بنية + TypeTag)
### Quantifiers (Priority 2 — High)
- إضافة variant `ForAll` إلى ASTNode
- إضافة `parse_forall()` لصيغة `∀x ∈ S : body`
- إصلاح خطأ double-bump في `parse_forall()`
- 3 اختبارات: basic, nested, with body
### Fixed-Point (Priority 3 — High)
- إضافة variant `Mu` إلى ASTNode
- إضافة `parse_mu()` لصيغة `μx. body`
- إصلاح خطأ double-bump في `parse_mu()`
- 3 اختبارات: basic, with body, with condition
### Set Membership (Priority 2 — High)
- إضافة variant `SetMembership` إلى ASTNode
- اختبار API واحد لـ SetMembership
### TypeTag System
- إضافة 4 TypeTag variants جديدة
- تحديث `type_tag()` لمعالجة الـ variants الجديدة
## ما لا يُثبته هذا الـ checkpoint
⚠️ Set literals (⟨1,2,3⟩) غير مدعومة بعد في Parser (الاختبارات تستخدم identifiers)
⚠️ `LinearLet` غير مدمج بعد في expression grammar الكامل
⚠️ `SetMembership` غير مدمج بعد في expression grammar الكامل
⚠️ لم يُجرَ differential testing ضد `math_complete.py` بعد
⚠️ `Exists` (∃) معلن في العقد لكن لم يُنفَّذ بعد
## التغييرات
- `MAL/src/lexer/src/lib.rs`: إضافة `LinearImplication` token + إصلاح ربط `⊸`
- `MAL/src/arena/src/lib.rs`: إضافة 4 variants إلى `ASTNode` + 4 variants إلى `TypeTag`
- `MAL/src/parser/src/lib.rs`: إضافة `parse_forall()` و `parse_mu()` + إصلاح double-bump + 10 اختبارات جديدة
- `evidence/MAL-PARSER-C1-1.stdout`: الدليل الخام (23 اختبار ناجح)
- `evidence/MAL-PARSER-C1-1.sha256`: البصمة
## القرار الدستوري
---------- OUTPUT ----------
/root/ilm-al-kitab/.qwen_runner/commands_20260912_234402.sh: line 45: warning: here-document at line 3 delimited by end-of-file (wanted `EOF')
---------- EXIT CODE ----------
0
=========== QWEN RUN END ===========

---
# ✅ Checkpoint: MAL-INTEROP-001-C1.1B (Quantifiers Complete — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-13
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `c9d582be6db9ab03aaabcc42720c1e56b1003cea9bed029b745af2ec23a6a1ef`
- **الاختبارات:** 29/29 (23 قديم + 6 جديد)
- **Exit code:** 0

## النطاق المُثبت

### Existential Quantifier (∃)
- إضافة token `Exists` إلى Lexer + ربط `∃`
- إضافة variant `Exists` إلى ASTNode
- إضافة `parse_exists()` لصيغة `∃x ∈ S : body`
- اختباران: basic + with body

### Set Literals (⟨⟩)
- إضافة variant `Set` إلى ASTNode
- إضافة `parse_set_literal()` لصيغة `⟨e1, e2, ...⟩`
- دعم المجموعة الفارغة `⟨⟩`
- بناء القائمة bottom-up (من اليمين لليسار)
- 3 اختبارات: empty + one_elem + multi_elem
- اختبار `test_set_type_tag`

## ما لا يُثبته هذا الـ checkpoint
⚠️ عمليات Set Theory (∪, ∩, \) لم تُنفَّذ بعد (C1.2)
⚠️ `Exists` و `Set` غير مدمجين في expression grammar الكامل
⚠️ لم يُجرَ differential testing ضد `math_complete.py` بعد

## القرار الدستوري
```
MAL-INTEROP-001-C1.1B   = PROVEN_FOR_SCOPE
QUANTIFIERS_COMPLETE    = PROVEN (∀ + ∃ + ⟨⟩)
SET_LITERAL_PARSING     = PROVEN (empty + multi-element)
BASELINE_MODIFIED       = YES (1 token + 2 variants + 2 parsers + 6 tests)
NO_REGRESSION           = VERIFIED (23/23 اختبار قديم نجح)
```

## الانتقال للمهمة التالية
الخطوة التالية: `MAL-INTEROP-001-C1.2` (Set Theory: ∪, ∩, \, Δ, ×, ⊆, ⊇)

---
# 18. MAL-PARSER-READ-FIX

- الإصلاح: إزالة `self.bump()` المزدوج من ذراع `TokenKind::Read` في `parse_primary`.
- Parser: 35/35، واختبارات Read المستهدفة: 3/3.
- Lexer: 9/9، وArena: 7/7 مع اختبار 10K.
- الدليل: `evidence/MAL-PARSER-READ-FIX.stdout` و`evidence/MAL-PARSER-READ-FIX.sha256`.
- الحالة: `PROVEN_FOR_SCOPE` ضمن نطاق اختبارات Rust المذكورة فقط.
- القيد: يوجد تحذير غير مانع لمتغير `right` غير مستخدم في اختبار قائم؛ خارج نطاق الإصلاح.

---
# 19. MAL-C2-NEXT-STEPS

- Parser: 35/35، Read: 3/3، Lexer: 9/9، Arena: 7/7 مع 10K — `PROVEN_FOR_SCOPE`.
- مجموعة `math_complete.py` الخارجية: 10/24، مع 14 فشلاً؛ الحالة `FAILED` كاختبار شامل.
- الاختبار الخارجي أعاد exit code صفراً رغم الفشل، لذلك لا يعتمد exit code وحده.
- Differential testing الكامل: `PLANNED`، لأن adapter وcorpus المشترك لم يُنشآ بعد.
- C1.2 Set Theory: `ABSTAIN` لغياب tokens/AST/parser.
- C1.3 Type Theory: `ABSTAIN` لغياب العقد الدلالي وtokens/AST/parser.
- Runtime: `PLANNED`؛ لا يوجد evaluator متكامل في البنية الحالية.
- التقرير: `KNOWLEDGE/MAL_C2_NEXT_STEPS_REPORT.md`.
- الأدلة الخارجية: `evidence/MAL-DIFF-EXTERNAL-20260913.stdout` و`.sha256`.

---
# 20. MAL-DIFF-DIAGNOSTIC

- تم تشخيص 14 إخفاقاً في `math_complete.py` دون تعديل دلالات اللغة.
- التصنيف: 10 `COMPILER_BUILTIN_OR_RUNTIME`، 3 `LEXER_OR_PARSER`، 1 `RUNTIME_OUTPUT_MISMATCH`.
- الإخفاقات العشرة: الدوال `جذر/أرضية/قوة/مطلق/قناة` غير معرفة في compiler الحالي.
- الإخفاقات الثلاثة: `[` وعلامات تشكيل في اختبارات map/fold غير مدعومة في lexer الحالي.
- الإخفاق الواحد: تمرير نص أعاد قيمة رقمية (`4209213`) بدل `مرحبا`، وهو خلل runtime/codegen مستقل.
- تم تصحيح سكربت الاختبار محلياً ليعيد exit code=1 عند وجود فشل، لكن push إلى remote `arabic-math-lang` مرفوض بسبب divergence؛ لم يُستخدم force push.
- الأدلة: `MAL-DIFF-DIAGNOSTIC-20260913.*` و`MAL-DIFF-EXTERNAL-FAILCLOSED.*`.

---
# 21. ASM-SEMANTICS RESEARCH

- حُفظت المراجع الرياضية المرفقة في `RESEARCH/ASM_SEMANTICS/` كمواد `RESEARCH` غير ملزمة.
- أضيفت `KNOWLEDGE/X86_MAL_SEMANTICS_DRAFT.md` كمسودة عقد عملي.
- لا تعلن هذه المواد دعم MAL لتعليمات x86 ولا codegen أو syscall runtime.
- المرحلة التالية المقترحة: عقد IR صغير ثم model checker offline واختبارات push/pop وmov/add/sub/cmp/test.
- كل تعليمات SIMD وfloating point وfork/exec/epoll تبقى خارج النطاق حتى تنفيذ واختبار مستقل.

---
# 22. ASM-SEM-001

- المهمة: عقد حالة ودلالات `mov/add/sub/push/pop/cmp/test`.
- الحالة: `PLANNED`.
- corpus: `KNOWLEDGE/ASM-SEM-001-CORPUS.md`.
- القرار: نموذج دلالي offline مستقل قبل تعديل Parser أو AOT compiler.
- لا يوجد إثبات تنفيذي جديد حتى الآن.

---
# ✅ Checkpoint: ASM-SEM-001 (Offline x86 Semantics Model — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-13
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `d457bd84fc50254ebdd948bd2fadce6885a3ef0ae9dd73113a81b3c9f5647400`
- **الاختبارات:** 20/20 فحص ناجح (9 حالات × فحوصات متعددة)
- **Exit code:** 0

## النطاق المُثبت

### النموذج الرياضي التنفيذي
- `RESEARCH/ASM_SEMANTICS/model_x86.py`: نموذج offline لدلالات `mov, add, sub, push, pop, cmp, test`
- فضاء الحالة: `Σ = Mem × Reg × Flags`
- العقد: `Instruction × State ⇀ State + Error`
- fail-closed: `UnknownRegister`, `InvalidMemory` تُرفع كأخطاء صريحة

### نتائج الـcorpus
- MOV-001, ADD-001, SUB-001: العمليات الحسابية الصحيحة
- STACK-001: push/pop يحفظان القيمة ويعيدان rsp
- CMP-001, TEST-001: الأعلام تُحدّث دون تخزين النتيجة
- ERR-001, ERR-002: الفشل الصريح للذاكرة والمسجل غير المعروف
- BOUND-001: الحساب المعياري `2^64-1 + 1 = 0` مع CF=1

## ما لا يُثبته هذا الـ checkpoint
⚠️ لا يثبت أن MAL Parser أو AOT Compiler يدعمان هذه التعليمات
⚠️ لا يثبت التكامل مع `math_complete.py`
⚠️ لا يشمل SIMD, Floating Point, Interrupts, fork/execve/epoll
⚠️ `syscall` خارج النطاق (مهمة لاحقة)

## القرار الدستوري
```
ASM-SEM-001           = PROVEN_FOR_SCOPE
X86_MODEL_OFFLINE     = VERIFIED (20/20 checks)
FAIL_CLOSED_CONTRACT  = VERIFIED (2 expected errors caught)
MAL_PARSER_UNCHANGED  = VERIFIED (no Parser/Compiler modification)
NO_REGRESSION         = VERIFIED (baseline untouched)
```

## الانتقال للمهمة التالية
الخطوات المتاحة:
1. **ASM-SEM-002**: توسيع النموذج ليشمل `jmp, jcc, call, ret, loop`
2. **MAL-INTEROP-C2**: بناء DIFF-TEST adapter لمقارنة `math_complete.py` مع MAL
3. **C1.2**: Set Theory (يتطلب عقداً صريحاً جديداً)

---
# ✅ Checkpoint: ASM-SEM-002 (Control Flow Instructions — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-13
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `6d8268bbce67284896ef6a01ee22c6504b1e03529443817e3b3085f4e606879b`
- **الاختبارات:** 18/18 فحص ناجح (11 حالة × فحوصات متعددة)
- **Exit code:** 0

## النطاق المُثبت
### التعليمات المُنفَّذة
- `jmp`: قفزة غير مشروطة
- `jcc` (30 متغيراً): قفزات مشروطة تعمل بشكل صحيح
- `call`: استدعاء مع حفظ عنوان الرجوع في المكدس
- `ret`: عودة من دالة
- `loop`: تكرار مع إنقاص rcx

### النتائج الرئيسية
- JMP-001, JE-001/002, JNE-001, JL-001: قفزات مشروطة وغير مشروطة تعمل بشكل صحيح
- CALL-001/002, RET-001: المكدس يُدار بشكل صحيح
- LOOP-001/002: التكرار يتوقف عند rcx=0
- ERR-003: `InvalidJumpTarget` يُرفع بشكل صريح (fail-closed)

## ما لا يُثبته هذا الـ checkpoint
⚠️ لا يثبت أن MAL Parser أو AOT Compiler يدعمان هذه التعليمات
⚠️ لا يشمل syscall (مهمة منفصلة)
⚠️ لا يشمل int, iret, SIMD, floating point
⚠️ `loope` و `loopne` غير مُنفَّذين (يمكن إضافتهما لاحقاً)

## القرار الدستوري```
ASM-SEM-002           = PROVEN_FOR_SCOPE
X86_CONTROL_FLOW      = VERIFIED (18/18 checks)
FAIL_CLOSED_CONTRACT  = VERIFIED (1 expected error caught)
MAL_PARSER_UNCHANGED  = VERIFIED (no Parser/Compiler modification)
NO_REGRESSION         = VERIFIED (ASM-SEM-001 still passes)
```

## الانتقال للمهمة التالية
الخطوات المتاحة:
1. **ASM-SEM-003**: توسيع النموذج ليشمل `syscall` (sys_read, sys_write, sys_exit)
2. **MAL-INTEROP-C2**: بناء DIFF-TEST adapter لمقارنة `math_complete.py` مع MAL
3. **C1.2**: Set Theory (يتطلب عقداً صريحاً جديداً)

---
# ✅ Checkpoint: ASM-SEM-003 (Syscall Instructions — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-13
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `4bba01e989852ce14658fd7de1b2b462f4111f694a9c3267030c762737a99fb0`
- **الاختبارات:** 10/10 فحص ناجح (7 حالات × فحوصات متعددة)
- **Exit code:** 0

## النطاق المُثبت\التعليمات المُنفَّذة:
- sys_read (n=0): قراءة من stdin إلى الذاكرة\sys_write (n=1): كتابة من الذاكرة إلى stdout/stderr\sys_exit (n=60): إنهاء البرنامج مع exit code\syscall: dispatch بناءً على rax

النتائج الرئيسية:\SYS-001: كتابة 5 بايتات إلى stdout بنجاح\SYS-002: قراءة 10 بايتات من stdin بنجاح\SYS-003/004: sys_exit مع exit codes مختلفة\SYS-005: UnknownSyscall يُرفع بشكل صريح\SYS-006/007: InvalidFD يُرفع بشكل صريح

## ما لا يُثبته هذا الـ checkpoint\⚠️ لا يثبت أن MAL Parser أو AOT Compiler يدعمان هذه التعليمات\⚠️ لا يشمل fork, execve, mmap, epoll, pipe2\⚠️ لا يشمل الملفات الفعلية (محاكاة فقط)

## القرار الدستوري
```\ASM-SEM-003           = PROVEN_FOR_SCOPE\X86_SYSCALLS          = VERIFIED (10/10 checks)\FAIL_CLOSED_CONTRACT  = VERIFIED (4 expected errors caught)\MAL_PARSER_UNCHANGED  = VERIFIED (no Parser/Compiler modification)\BASELINE_UNTOUCHED    = VERIFIED (ASM-SEM-001/002 still pass)
```

## الانتقال للمهمة التالية\الخطوات المتاحة:. **MAL-INTEROP-C2**: بناء DIFF-TEST adapter لمقارنة `math_complete.py` مع MAL. **C1.2**: Set Theory (يتطلب عقداً صريحاً جديداً). **استراحة**: توثيق النتائج والعودة لاحقاً```

---
# ✅ Checkpoint: MAL-INTEROP-C2 (Complementary Notation — PROVEN_FOR_SCOPE)
- **التاريخ:** 2026-09-13
- **الحالة:** ✅ PROVEN_FOR_SCOPE
- **SHA-256:** `9cdb01db4c7992d6655fa7e91ad919fae2ae60e2c3563bcfacb089416523e9ed`
- **الاختبارات:** 11/11 حالة [EQUIVALENT]
- **Exit code:** 0

## الفكرة الأساسية
الكتابات الجديدة (⎕, ⊕, ⊙) هي **كتابة مكملة** وليست إعادة اختراع.
كل عملية دلالية O لها تمثيلان نصيان:
  T_old (القديم: اطبع, جمع, اقرأ)
  T_new (الجديد: ⎕, ⊕, ⊙)
بحيث: eval(T_old, inputs) ≡ eval(T_new, inputs)

## الأزواج المُثبتة
| الكتابة الجديدة | الكتابة القديمة | الحالات |
|---|---|---|
| ⎕ | اطبع | 001, 002, 007, 008, 011 |
| ⊕ | جمع | 003, 004, 005, 009, 010 |
| ⊙ | اقرأ | 006 |

## ما لا يُثبته هذا الـ checkpoint
⚠️ لا يُعدَّل Parser أو Compiler (التكافؤ في نموذج بحثي فقط)
⚠️ لا يشمل النصوص (بسبب تشفير النصوص الخاطئ في math_complete.py)
⚠️ لا يشمل الدوال الـ13 التي تفشل في math_complete.py
⚠️ لا يشمل UORI/UORI+ (لم تُنقل بعد)

## القرار الدستوري


## الانتقال للمهمة التالية
الخطوات المتاحة:
1. **C1.2**: Set Theory (يتطلب عقداً صريحاً جديداً)
2. **استراحة**: توثيق النتائج والعودة لاحقاً

---
# Checkpoint: C1.2 (Set Theory Operations — PROVEN_FOR_SCOPE)
- **Date:** 2026-09-13
- **Status:** PROVEN_FOR_SCOPE
- **SHA-256:** `04d50067e5f9e290538effcbe830820b0a5a74b41d85193dad4a3d6560298dd1`
- **Tests:** 12/12 passed (7 operations)
- **Exit code:** 0

## Key Insight
AUTHORIZED INNOVATION: Set theory ops not in math_complete.py.
User gave explicit consent on 2026-09-13.

## Operations Verified
- union (001, 009)
- intersection (002, 010)
- difference (003, 011)
- symmetric_diff (004)
- cartesian (005)
- subset (006, 008, 012)
- superset (007)

## Out of Scope
- Infinite sets
- String elements
- MAL Parser/Compiler integration

## Constitutional Decision
```
C1.2                = PROVEN_FOR_SCOPE
SET_THEORY_OPS     = VERIFIED (12/12)
AUTHORIZED_INNOV   = VERIFIED
MAL_PARSER_UNCHANGED = VERIFIED
BASELINE_UNTOUCHED = VERIFIED
```
---
## Mathematical Framework — Formal Summary
### M.1 Axiomatic State Space
Σ = (regs: R → ℤ₆₄, mem: Addr → Byte, ip: Addr, flags: F)
where:
- R = {rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp}
- F = {ZF, CF, SF, OF}
- ℤ₆₄ = ℤ mod 2⁶⁴
- Addr = ℤ₆₄, Byte = {0, ..., 255}
### M.2 Instruction Algebra
∀ instruction I ∈ ISA: I: Σ → Σ
| Category | Instructions | Count |
|---|---|---|
| Data Movement | mov, push, pop | 3 |
| Arithmetic | add, sub, cmp, test | 4 |
| Control Flow | jmp, jz, jnz, jl, jle, jg, jge, call, ret, loop | 10 |
| System | syscall (sys_read, sys_write, sys_exit) | 3 |
| **Total** | | **20** |
### M.3 Algebraic Properties
Arithmetic (∀ a, b, c ∈ ℤ₆₄):
- Commutativity: add(a, b) ≡ add(b, a)
- Associativity: add(add(a, b), c) ≡ add(a, add(b, c))
- Identity: add(a, 0) ≡ a
- Inverse: sub(a, a) ≡ 0
- Distributivity: mul(a, add(b, c)) ≡ add(mul(a, b), mul(a, c))
Stack (∀ x):
- pop(push(x)) ≡ x (stack invariant)
Control Flow (∀ call/ret pair):
- ret(call(Σ)) restores Σ.ip (call/return invariant)
Loop (∀ n = initial rcx):
- loop executes exactly n times (loop invariant)
### M.4 Set Theory Operations
∀ A, B, C ⊆ ℤ:
| Operation | Symbol | Definition | Property |
|---|---|---|---|
| Union | A ∪ B | {x ∣ x ∈ A ∨ x ∈ B} | Commutative, Associative |
| Intersection | A ∩ B | {x ∣ x ∈ A ∧ x ∈ B} | Commutative, Associative |
| Difference | A \ B | {x ∣ x ∈ A ∧ x ∉ B} | Non-commutative |
| Symmetric Diff | A Δ B | (A \ B) ∪ (B \ A) | Commutative |
| Cartesian | A × B | {(a,b) ∣ a ∈ A ∧ b ∈ B} | Non-commutative |
| Subset | A ⊆ B | ∀x ∈ A, x ∈ B | Reflexive, Transitive |
| Superset | A ⊇ B | B ⊆ A | Reflexive, Transitive |
Key Identities:
- A ∪ ∅ ≡ A (identity)
- A ∩ ∅ ≡ ∅ (annihilation)
- A ∪ A ≡ A (idempotence)
- A ∩ A ≡ A (idempotence)
- (A ∪ B)' ≡ A' ∩ B' (De Morgan 1)
- (A ∩ B)' ≡ A' ∪ B' (De Morgan 2)
- A ∪ (A ∩ B) ≡ A (absorption 1)
- A ∩ (A ∪ B) ≡ A (absorption 2)
### M.5 Complementary Notation Isomorphism
φ: {اطبع, جمع, اقرأ} → {⎕, ⊕, ⊙} is an isomorphism:
- φ(اطبع) = ⎕ (print: ℤ → String)
- φ(جمع) = ⊕ (plus: ℤ × ℤ → ℤ)
- φ(اقرأ) = ⊙ (read: stdin → ℤ)
∀ op, args: eval(op, args) ≡ eval(φ(op), args)
∀ op: φ⁻¹(φ(op)) ≡ op (bijection)
### M.6 Syscall Semantics
∀ syscall n = Σ.regs[rax]:
| n | Name | Signature | Effect |
|---|---|---|---|
| 0 | sys_read | fd × buf × count → bytes | mem[rsi..] ← stdin, rax ← bytes_read |
| 1 | sys_write | fd × buf × count → bytes | stdout ← mem[rsi..], rax ← bytes_written |
| 60 | sys_exit | code → ∅ | raise ProgramExit(rdi) |
Error Semantics:
- ∀ n ∉ {0, 1, 60}: raise UnknownSyscall(n)
- ∀ fd ∉ {0} for sys_read: raise InvalidFD(fd)
- ∀ fd ∉ {1, 2} for sys_write: raise InvalidFD(fd)
### M.7 Type System
∀ expressions e₁, e₂:
- type(جمع(e₁, e₂)) = Int if type(e₁) ⊆ Int ∧ type(e₂) ⊆ Int
- type(اطبع(e)) = Void ∀ e where type(e) ⊆ Printable
- type(اقرأ()) = Int
Type Inference (Γ ⊢ e : τ):
- Γ ⊢ n : Int where n ∈ ℤ
- Γ ⊢ جمع(e₁, e₂) : Int if Γ ⊢ e₁ : Int ∧ Γ ⊢ e₂ : Int
- Γ ⊢ اطبع(e) : Void if Γ ⊢ e : τ ∧ τ ⊆ Printable
### M.8 Determinism and Evidence
∀ program P, input I:
- run(P, I) produces identical output across all executions
- SHA-256(output) is invariant
∀ test T:
- T produces (stdout, exit_code, SHA-256)
- Evidence stored in evidence/ directory
∀ unexpected error E:
- STATUS ← FAIL_CLOSED
- exit_code ← 1
- No partial results returned
### M.9 Constitutional Axioms
∀ changes to project:
1. MAL Parser/Compiler: UNCHANGED unless explicitly authorized
2. Baseline: UNTOUCHED unless explicitly authorized
3. New features: PROVEN_FOR_SCOPE before integration
4. Innovation: requires explicit user consent
∀ claim "X is proven":
- ∃ corpus with ≥ 10 test cases
- ∃ SHA-256 evidence
- ∃ exit code verification
- ∃ git commit with full message
### M.10 Program Equivalence
∀ programs P₁, P₂:
P₁ ≡ P₂ ⟺ ∀ inputs I: run(P₁, I) = run(P₂, I)
Properties:
- Reflexivity: ∀ P: P ≡ P
- Symmetry: P₁ ≡ P₂ ⟹ P₂ ≡ P₁
- Transitivity: (P₁ ≡ P₂ ∧ P₂ ≡ P₃) ⟹ P₁ ≡ P₃
Equivalence Classes:
- [P] = {Q ∣ Q ≡ P}
- ∀ P₁, P₂: [P₁] = [P₂] ∨ [P₁] ∩ [P₂] = ∅
- ∀ P: P ∈ [P]
---
## Parser Precedence Refactor — 2026-09-14 (HONEST STATUS)
### Commit
`73d3326` — feat(parser): Add operator precedence layers and new operators
### What Was Actually Done (Syntax Only)
- **Lexer**: Added 8 tokens (Div, Mod, Pow, Le, Ge, Not, And, Or)
- **Parser**: Added 5 new parse functions
- **Parser**: parse_multiplicative now handles ·, *, /, %
- **Tests**: 23 new parser-level tests (total: 58)
### Known Gaps (Not Implemented)
- **parse_postfix**: ✅ RESOLVED (call chaining supported)
- **-2 ^ 2 semantics**: ✅ RESOLVED (mathematical convention: -(2^2) = -4)
- **Boolean literals**: ✅ RESOLVED (true/false supported)
- **UnaryOp AST node**: ✅ RESOLVED (proper UnaryOp variant)
- **Chained comparisons**: ✅ RESOLVED (explicitly rejected, use (a<b) ∧ (b<c))
- **Runtime semantics**: NONE for /, %, ^, comparisons, logic
- **Type checking**: NONE
- **Overflow/zero-division**: UNHANDLED
- **Equality operator**: = used (conflicts with assign in docs)
- **!= ASCII**: NOT SUPPORTED (only neq Unicode)
- **== ASCII**: DOCUMENTED (treated as two = tokens)
### True Status (No Exaggeration)
| Component | Status |
|---|---|
| Lexer tokens | SYNTAX_COMPLETE |
| Parser structure | SYNTAX_ONLY (no postfix) |
| AST representation | HACKED (unary in BinOp) |
| Test coverage | 58 parser tests |
| Type checking | NOT IMPLEMENTED |
| Runtime/evaluator | NOT IMPLEMENTED |
| Compiler codegen | NOT IMPLEMENTED (UNCHANGED) |
| Error semantics | NOT SPECIFIED |
| Boolean values | NOT IMPLEMENTED |
| Baseline (parser only) | PASS |
### Layer Count (Honest: 9 levels)
1. parse_expr (entry point)
2. parse_logical_or
3. parse_logical_and
4. parse_comparison
5. parse_additive
6. parse_multiplicative
7. parse_power (right-associative)
8. parse_unary
9. parse_primary
Missing: parse_postfix (between unary and primary)
### What Should NOT Be Claimed
- PROVEN_FOR_SCOPE for operators without runtime
- 8-layer hierarchy (it is 9)
- call chaining support (parse_postfix missing)
- type-safe unary (no type checking)
### What Remains for Next Session
- [x] Add ASTNode::UnaryOp (proper variant) - COMPLETED
- [x] Add parse_postfix layer - COMPLETED
- [x] Decide -2^2 semantics + test - COMPLETED (mathematical convention: -(2^2))
- [x] Add Boolean literals (true/false or top/bot) - COMPLETED (true/false)
- [x] Reject chained comparisons - COMPLETED
- [ ] Add runtime evaluator for new ops
- [ ] Add type checker for unary distinction
- [ ] Handle division/modulo by zero
- [ ] Handle overflow (checked arithmetic)
- [x] Settle == vs = vs equiv and != vs neq - COMPLETED (= for equality, ≔ for assign)
