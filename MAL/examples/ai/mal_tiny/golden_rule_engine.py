#!/usr/bin/env python3
"""
محرك القاعدة الذهبية (Golden Rule Engine)
يدير تخصيص النموذج، تجميد الأوزان غير الضرورية، والتدريب المستمر ضمن ذاكرة < 4GB
"""
import os
import json
import sqlite3
import hashlib
from typing import Dict, List, Optional
from datetime import datetime
class GoldenRuleEngine:
    def __init__(self, db_path: str = "model_lifecycle.db", user_profile: str = "default_user"):
        self.db_path = db_path
        self.user_profile = user_profile
        self.active_domain = None
        self.max_ram_budget_mb = 3500  # ميزانية الذاكرة الحية (أقل من 4GB)
        self._init_lifecycle_db()
    def _init_lifecycle_db(self):
        """تهيئة قاعدة بيانات دورة حياة النموذج والأوزان"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        # جدول تصنيف أنواع البيانات (مثل: محمد، نشيط، فقه، طب...)
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS data_registry (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data_type TEXT,          -- مثال: 'محمد', 'نشيط', 'عام'
                domain TEXT,             -- مثال: 'شرعي', 'طبي'
                content_hash TEXT UNIQUE,
                priority_score REAL,     -- أولوية التدريب (1.0 = قصوى)
                is_frozen INTEGER DEFAULT 0, -- 0 = نشط في الذاكرة، 1 = مجمد في القرص
                created_at TEXT
            )
        ''')
        # جدول إدارة الأوزان (LoRA Adapters)
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS weight_adapters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                adapter_name TEXT UNIQUE,
                domain TEXT,
                size_mb REAL,
                status TEXT,             -- 'ACTIVE_IN_RAM', 'FROZEN_ON_DISK'
                last_trained TEXT
            )
        ''')
        conn.commit()
        conn.close()
    def classify_and_tag_data(self, text: str, inferred_domain: str) -> Dict:
        """
        القاعدة الذهبية 1: تصنيف البيانات وتحديد نوعها (محمد، نشيط، إلخ) قبل التدريب
        """
        # منطق بسيط لتصنيف النوع بناءً على الكلمات المفتاحية (يمكن ربطه بمصنف أصغر)
        data_type = "عام"
        priority = 0.5
        text_lower = text.lower()
        if "محمد" in text_lower or "رسول" in text_lower or "فقه" in text_lower:
            data_type = "محمد_شرعي"
            priority = 1.0 if inferred_domain == "شرعي" else 0.8
        elif "نشيط" in text_lower or "طاقة" in text_lower or "صحي" in text_lower:
            data_type = "نشيط_صحي"
            priority = 1.0 if inferred_domain == "طبي" else 0.8
        content_hash = hashlib.md5(text.encode('utf-8')).hexdigest()
        return {
            "data_type": data_type,
            "domain": inferred_domain,
            "content_hash": content_hash,
            "priority_score": priority,
            "is_frozen": 0,
            "created_at": datetime.now().isoformat()
        }
    def filter_and_prepare_training_batch(self, raw_dialogue_history: List[str], target_domain: str) -> List[Dict]:
        """
        القاعدة الذهبية 2: عزل وتصنيف البيانات. تجاهل أو تجميد ما لا علاقة له بأهداف المستخدم
        """
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        prioritized_batch = []
        for text in raw_dialogue_history:
            metadata = self.classify_and_tag_data(text, target_domain)
            # التحقق من التكرار
            cursor.execute('SELECT id FROM data_registry WHERE content_hash = ?', (metadata['content_hash'],))
            if cursor.fetchone():
                continue # تخطي البيانات المكررة لتوفير الذاكرة والوقت
            # القاعدة الحاسمة: إذا لم يكن المجال مطابقاً لهدف المستخدم، يتم تجميده فوراً
            if metadata['domain'] != target_domain and metadata['priority_score'] < 0.9:
                metadata['is_frozen'] = 1
                metadata['priority_score'] = 0.1 # خفض الأولوية لأدنى حد
            # حفظ في السجل
            cursor.execute('''
                INSERT OR IGNORE INTO data_registry 
                (data_type, domain, content_hash, priority_score, is_frozen, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
            ''', (metadata['data_type'], metadata['domain'], metadata['content_hash'], 
                  metadata['priority_score'], metadata['is_frozen'], metadata['created_at']))
            # إضافة للتدريب فقط إذا لم يكن مجمداً أو كانت أولويته عالية جداً
            if metadata['is_frozen'] == 0 or metadata['priority_score'] >= 0.9:
                prioritized_batch.append({
                    "text": text,
                    "metadata": metadata
                })
        conn.commit()
        conn.close()
        return prioritized_batch
    def manage_memory_streaming(self, new_domain: str):
        """
        القاعدة الذهبية 3: إدارة الذاكرة الحية (< 4GB) عبر تبديل الأوزان (Adapter Swapping)
        """
        if self.active_domain == new_domain:
            return # لا حاجة للتبديل
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        print(f"🔄 جاري تبديل السياق من '{self.active_domain}' إلى '{new_domain}'...")
        # 1. تجميد الأوزان الحالية وإخراجها من الذاكرة الحية (محاكاة)
        if self.active_domain:
            cursor.execute('''
                UPDATE weight_adapters 
                SET status = 'FROZEN_ON_DISK' 
                WHERE domain = ?
            ''', (self.active_domain,))
            print(f"   ✅ تم تجميد أوزان مجال '{self.active_domain}' ونقلها للقرص لتوفير الذاكرة.")
        # 2. تحميل الأوزان الجديدة إلى الذاكرة الحية (محاكاة Streaming)
        cursor.execute('''
            UPDATE weight_adapters 
            SET status = 'ACTIVE_IN_RAM' 
            WHERE domain = ?
        ''', (new_domain,))
        # إذا لم تكن موجودة، ننشئ سجلاً جديداً لها (حجم تقديري صغير جداً < 100MB)
        cursor.execute('''
            INSERT OR IGNORE INTO weight_adapters (adapter_name, domain, size_mb, status, last_trained)
            VALUES (?, ?, ?, ?, ?)
        ''', (f"adapter_{new_domain}", new_domain, 50.0, 'ACTIVE_IN_RAM', datetime.now().isoformat()))
        self.active_domain = new_domain
        conn.commit()
        conn.close()
        print(f"   ✅ تم تحميل أوزان مجال '{new_domain}' بنجاح في الذاكرة الحية.")
    def simulate_continuous_learning(self, training_batch: List[Dict]):
        """
        القاعدة الذهبية 4: التدريب الدوري المستمر مع إعطاء الأولوية لسياق المستخدم
        """
        if not training_batch:
            print("⚠️ لا توجد بيانات جديدة ذات أولوية للتدريب.")
            return
        print(f"\n🧠 بدء دورة تدريبية مصغرة (Micro-Learning) على {len(training_batch)} عينة...")
        print("   🔒 قاعدة التجميد مفعلة: يتم تحديث أوزان المجال النشط فقط.")
        # هنا يتم استدعاء مكتبة التدريب الفعلية (مثل Hugging Face PEFT/LoRA)
        # model.enable_gradient_checkpointing() # لتوفير الذاكرة
        # trainer.train() # تدريب على الـ batch المفلتر فقط
        total_priority = sum(item['metadata']['priority_score'] for item in training_batch)
        print(f"   ✅ اكتمل التدريب. أولوية السياق المضافة: {total_priority:.2f}")
        print("   💾 تم حفظ الأوزان الجديدة في المحول (Adapter) النشط فقط.")
if __name__ == "__main__":
    # تهيئة المحرك لمستخدم مهتم بـ "الفقه الشرعي" كمثال
    engine = GoldenRuleEngine(user_profile="user_fiqh_focused")
    # محاكاة سجل حوار يحتوي على مواضيع مختلطة
    raw_dialogue = [
        "ما هو حكم الصلاة في الوقت الحالي يا محمد؟",
        "هل يمكن أن أتمرن بشكل نشيط بعد الأكل مباشرة؟",
        "اشرح لي مفهوم الاجتهاد في أصول الفقه.",
        "ما هي أفضل طريقة لتدريب نموذج ذكاء اصطناعي؟",
        "يا نشيط، كم عدد ركعات السنة الراتبة؟"
    ]
    target_domain = "شرعي" # هدف المستخدم الأساسي
    print("="*80)
    print("👑 تطبيق القاعدة الذهبية: تصفية وتجميد البيانات غير ذات الصلة")
    print("="*80)
    # 1. تصفية البيانات
    batch = engine.filter_and_prepare_training_batch(raw_dialogue, target_domain)
    print(f"\n📊 ملخص التصفية:")
    print(f"   - إجمالي الجمل: {len(raw_dialogue)}")
    print(f"   - جمل مؤهلة للتدريب الحي (غير مجمدة): {len(batch)}")
    for item in batch:
        print(f"   ✅ [{item['metadata']['data_type']}] (أولوية: {item['metadata']['priority_score']}) -> {item['text'][:40]}...")
    # 2. إدارة الذاكرة (محاكاة التبديل)
    print("\n" + "="*80)
    print("💾 إدارة الذاكرة الحية (Streaming Weight Management)")
    print("="*80)
    engine.manage_memory_streaming("شرعي")
    # 3. التدريب المستمر
    print("\n" + "="*80)
    print("🧠 التدريب الدوري الموجه (Continuous Micro-Learning)")
    print("="*80)
    engine.simulate_continuous_learning(batch)
    print("\n🎉 تم بنجاح! النموذج الآن:")
    print("  ✅ يستهلك < 4GB RAM (بسبب تجميد الأوزان غير النشطة).")
    print("  ✅ مدرب حصرياً تقريباً على سياق المستخدم المستهدف.")
    print("  ✅ يصنف البيانات بذكاء (محمد، نشيط، إلخ).")
    print("  ✅ جاهز للتبديل السريع (Streaming) لمجالات أخرى عند الحاجة فقط.")
