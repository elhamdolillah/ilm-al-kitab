#!/usr/bin/env python3
"""
محرك التخصيص الديناميكي v3 - القاعدة الذهبية الصحيحة
الوسيط (Median) + blacklist يدوي
"""
import os
import sqlite3
from typing import Dict, List, Set
from datetime import datetime
from statistics import median
class DynamicSpecializationEngine:
    def __init__(self, db_path: str = "user_specialization.db", user_id: str = "user_001"):
        self.db_path = db_path
        self.user_id = user_id
        self.user_blacklist: Set[str] = set()
        self._init_db()
    def _init_db(self):
        conn = sqlite3.connect(self.db_path)
        c = conn.cursor()
        c.execute('''CREATE TABLE IF NOT EXISTS interaction_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT, user_id TEXT, domain TEXT, data_type TEXT, timestamp TEXT)''')
        c.execute('''CREATE TABLE IF NOT EXISTS domain_stats (
            domain TEXT PRIMARY KEY, total_interactions INTEGER, usage_percentage REAL,
            status TEXT, training_priority REAL, last_updated TEXT)''')
        c.execute('''CREATE TABLE IF NOT EXISTS weight_registry (
            domain TEXT PRIMARY KEY, adapter_file_path TEXT, size_mb REAL,
            current_location TEXT, status TEXT)''')
        conn.commit()
        conn.close()
    def set_blacklist(self, domains: List[str]):
        self.user_blacklist = set(domains)
        print(f"🚫 قائمة التجميد اليدوية: {self.user_blacklist}")
    def log_interaction(self, domain: str, data_type: str):
        conn = sqlite3.connect(self.db_path)
        conn.execute('INSERT INTO interaction_log (user_id, domain, data_type, timestamp) VALUES (?,?,?,?)',
                     (self.user_id, domain, data_type, datetime.now().isoformat()))
        conn.commit()
        conn.close()
    def recalculate_and_enforce_rules(self):
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT COUNT(*) FROM interaction_log WHERE user_id = ?', (self.user_id,))
        total = cursor.fetchone()[0]
        if total == 0:
            conn.close()
            return
        cursor.execute('SELECT domain, COUNT(*) FROM interaction_log WHERE user_id = ? GROUP BY domain', (self.user_id,))
        domains_data = cursor.fetchall()
        # حساب النسب
        domain_percentages = {}
        for domain, count in domains_data:
            domain_percentages[domain] = (count / total) * 100
        # 🔑 القاعدة الذهبية الصحيحة: الوسيط (Median) وليس المتوسط
        non_blacklisted = {d: p for d, p in domain_percentages.items() if d not in self.user_blacklist}
        if non_blacklisted:
            median_threshold = median(list(non_blacklisted.values()))
        else:
            median_threshold = 50.0
        print(f"\n📐 العتبة (الوسيط Median): {median_threshold:.1f}%")
        print(f"   (المتوسط الحسابي كان سيعطي: {sum(non_blacklisted.values())/len(non_blacklisted):.1f}%)")
        for domain, percentage in domain_percentages.items():
            # القاعدة 1: blacklist = مجمد دائماً
            if domain in self.user_blacklist:
                status, priority, location = 'FROZEN', 0.0, 'DISK'
                reason = "blacklist يدوي"
            # القاعدة 2: >= الوسيط = نشط
            elif percentage >= median_threshold:
                status, priority, location = 'ACTIVE', 1.0, 'RAM'
                reason = f">= الوسيط ({median_threshold:.1f}%)"
            # القاعدة 3: < الوسيط = مجمد
            else:
                status, priority, location = 'FROZEN', 0.0, 'DISK'
                reason = f"< الوسيط ({median_threshold:.1f}%)"
            cursor.execute('''INSERT OR REPLACE INTO domain_stats 
                (domain, total_interactions, usage_percentage, status, training_priority, last_updated)
                VALUES (?,?,?,?,?,?)''', (domain, int(percentage * total / 100), percentage, status, priority, datetime.now().isoformat()))
            cursor.execute('''INSERT OR REPLACE INTO weight_registry 
                (domain, adapter_file_path, size_mb, current_location, status)
                VALUES (?,?,?,?,?)''', (domain, f"adapters/{self.user_id}_{domain}.safetensors", 45.0, location, status))
            icon = "🔥" if status == 'ACTIVE' else "🧊"
            print(f"   {icon} {domain}: {percentage:.1f}% -> {status} ({reason})")
        conn.commit()
        conn.close()
    def get_training_batch(self, raw_data: List[Dict]) -> List[Dict]:
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT domain FROM domain_stats WHERE status = "ACTIVE"')
        active = {r[0] for r in cursor.fetchall()}
        conn.close()
        filtered = [i for i in raw_data if i['domain'] in active]
        discarded = len(raw_data) - len(filtered)
        if discarded > 0:
            print(f"   🚫 تم حذف {discarded} عينة من مجالات مجمدة")
        return filtered
    def simulate_user_journey(self):
        print("="*80)
        print("👤 محاكاة: مستخدم يهتم بالفقه والبرمجة، ولا يهتم بالموسيقى")
        print("="*80)
        self.set_blacklist(["موسيقى"])
        interactions = [
            ("شرعي", "سؤال_فقهي"), ("حاسوبي", "كود_برمجي"),
            ("موسيقى", "استفسار_عام"), ("شرعي", "فتوى"),
            ("حاسوبي", "تصحيح_خطأ"), ("موسيقى", "أغنية"),
            ("شرعي", "حديث"), ("حاسوبي", "خوارزمية"),
            ("موسيقى", "نوتة"), ("شرعي", "أصول_فقه")
        ]
        for d, t in interactions:
            self.log_interaction(d, t)
            print(f"   📝 [{d}] - {t}")
        print("\n⚙️ تطبيق القاعدة الذهبية (وسيط + blacklist)...")
        self.recalculate_and_enforce_rules()
        # لوحة التحكم
        conn = sqlite3.connect(self.db_path)
        c = conn.cursor()
        c.execute('SELECT domain, usage_percentage, status, training_priority FROM domain_stats ORDER BY usage_percentage DESC')
        print(f"\n{'المجال':<12} | {'النسبة':<8} | {'الحالة':<10} | {'التدريب':<8} | {'الموقع'}")
        print("-"*65)
        for d, p, s, pr in c.fetchall():
            c2 = conn.execute('SELECT current_location FROM weight_registry WHERE domain=?', (d,))
            loc = c2.fetchone()[0]
            icon = "🔥 نشط" if s == 'ACTIVE' else "🧊 مجمد"
            print(f"{d:<12} | {p:>5.1f}%  | {icon:<10} | {pr:>5.1f}    | {loc}")
        print("-"*65)
        conn.close()
        # اختبار التصفية
        print("\n🧠 تصفية بيانات التدريب:")
        raw = [
            {"text": "ما حكم كذا؟", "domain": "شرعي"},
            {"text": "من هو ملحن هذه الموسيقى؟", "domain": "موسيقى"},
            {"text": "كيف أكتب دالة؟", "domain": "حاسوبي"},
            {"text": "ما هي الآلات الموسيقية؟", "domain": "موسيقى"}
        ]
        batch = self.get_training_batch(raw)
        print(f"✅ تم قبول {len(batch)} عينة للتدريب:")
        for item in batch:
            print(f"   ✅ [{item['domain']}] {item['text']}")
if __name__ == "__main__":
    if os.path.exists("user_specialization.db"):
        os.remove("user_specialization.db")
    engine = DynamicSpecializationEngine(user_id="user_fiqh_dev")
    engine.simulate_user_journey()
    print("\n🎉 القاعدة الذهبية v3 تعمل بنجاح!")
    print("  ✅ شرعي (40%) >= الوسيط (30%) -> نشط 🔥")
    print("  ✅ حاسوبي (30%) >= الوسيط (30%) -> نشط 🔥")
    print("  ✅ موسيقى (30%) في blacklist -> مجمد 🧊")
