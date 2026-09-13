# سجل القرارات

## DEC-001: Kotlin + sshj لتطبيق SSH
**التاريخ:** 2026-09-09
**المبرر:** Foreground Service + أداء + دعم خطوط عربي أصلي
**البديل المرفوض:** Kivy + paramiko
**سبب الرفض:** خدمات خلفية معقدة + حجم APK

## DEC-002: Rust للـ Arena
**التاريخ:** 2026-08-25
**المبرر:** #![forbid(unsafe_code)] + أمان ذاكرة
**البديل المرفوض:** C++
**سبب الرفض:** لا ضمانات أمان مكافئة

## DEC-003: رفض mal_runner المقترح
**التاريخ:** 2026-09-09
**المبرر:** غلاف CLI بفحص نصي، لا تنفيذ دلالي
**الحكم:** ABSTAIN
---
## Mathematical Decision Framework
∀ decision D ∈ decisions:
- D.rationale: formal_justification(D) → {axioms, theorems, evidence}
- D.impact: Σ → Σ (state transition)
- D.reversibility ∈ {reversible, irreversible}
- D.evidence: SHA-256 + stdout + exit_code
∀ D₁, D₂: D₁.conflicts_with(D₂) ⟹ ¬(D₁ ∧ D₂) (mutual exclusion)
∀ D: D.approved ⟹ ∃ user_consent(D) (explicit authorization)
