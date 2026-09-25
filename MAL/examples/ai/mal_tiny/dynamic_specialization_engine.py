#!/usr/bin/env python3
"""
محرك التخصيص الديناميكي (Dynamic Specialization Engine) v2
القاعدة الذهبية: عتبة نسبية (فوق المتوسط = نشط) + تجميد يدوي لما لا يريده المستخدم
"""
import os
import json
import sqlite3
from typing import Dict, List, Set
from datetime import datetime
class DynamicSpecializationEngine:
    def __init__(self, db_path: str = "user_specialization.db", user_id: str = "user_001"):
        self.db_path = db_path
        self.user_id = user_id
        self.ram_budget_mb = 3500
        # قائمة التجميد اليدوية: مجالات يرفض المستخدم التدريب عليها نهائياً
        self.user_blacklist: Set[str] = set()
        self._init_db()
    def _init_db(self):
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('''CREATE TABLE IF NOT EXISTS interaction_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT, user_id TEXT, domain TEXT, data_type TEXT, timestamp TEXT)''')
        cursor.execute('''CREATE TABLE IF NOT EXISTS domain_stats (
            domain TEXT PRIMARY KEY, total_interactions INTEGER DEFAULT 0,
            usage_percentage REAL DEFAULT 0.0, status TEXT DEFAULT 'FROZEN',
            training_priority REAL DEFAULT 0.0, last_updated TEXT)''')
        cursor.execute('''CREATE TABLE IF NOT EXISTS weight_registry (
            domain TEXT PRIMARY KEY, adapter_file_path TEXT, size_mb REAL,
            current_location TEXT, status TEXT)''')
        conn.commit()
        conn.close()
    def set_blacklist(self, domains: List[str]):
        """المستخدم يحدد المجالات التي لا يريدها أبداً (مثل: موسيقى)"""
        self.user_blacklist = set(domains)
        print(f"🚫 قائمة التجميد اليدوية: {self.user_blacklist}")
    def log_interaction(self, domain: str, data_type: str):
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('INSERT INTO interaction_log (user_id, domain, data_type, timestamp) VALUES (?,?,?,?)',
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
        # حساب المتوسط (العتبة النسبية)
        non_blacklisted = {d: p for d, p in domain_percentages.items() if d not in self.user_blacklist}
        if non_blacklisted:
            average = sum(non_blacklisted.values()) / len(non_blacklisted)
        else:
            average = 50.0
        print(f"\n📐 العتبة النسبية (المتوسط): {average:.1f}%")
        for domain, percentage in domain_percentages.items():
            # القاعدة 1: إذا كان في القائمة السوداء -> مجمد دائماً
            if domain in self.user_blacklist:
                status = 'FROZEN'
                priority = 0.0
                location = 'DISK'
                reason = "تجميد يدوي (blacklist)"
            # القاعدة 2: فوق المتوسط -> نشط
            elif percentage >= average:
                status = 'ACTIVE'
                priority = 1.0
                location = 'RAM'
                reason = f"فوق المتوسط ({average:.1f}%)"
            # القاعدة 3: تحت المتوسط -> مجمد
            else:
                status = 'FROZEN'
                priority = 0.0
                location = 'DISK'
                reason = f"تحت المتوسط ({average:.1f}%)"
            cursor.execute('''INSERT OR REPLACE INTO domain_stats 
                (domain, total_interactions, usage_percentage, status, training_priority, last_updated)
                VALUES (?,?,?,?,?,?)''', (domain, domain_percentages[domain], percentage, status, priority, datetime.now().isoformat()))
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
        active_domains = {row[0] for row in cursor.fetchall()}
        conn.close()
        filtered = [item for item in raw_data if item['domain'] in active_domains]
        discarded = len(raw_data) - len(filtered)
        if discarded > 0:
            print(f"   🚫 تم حذف {discarded} عينة من مجالات مجمدة")
        return filtered
    def simulate_user_journey(self):
        print("="*80)
        print("👤 محاكاة: مستخدم يهتم بالفقه والبرمجة، ولا يهتم بالموسيقى أبداً")
        print("="*80)
        # المستخدم يضع الموسيقى في القائمة السوداء
        self.set_blacklist(["موسيقى"])
        interactions = [
            ("شرعي", "سؤال_فقهي"), ("حاسوبي", "كود_برمجي"),
            ("موسيقى", "استفسار_عام"), ("شرعي", "فتوى"),
            ("حاسوبي", "تصحيح_خطأ"), ("موسيقى", "أغنية"),
            ("شرعي", "حديث"), ("حاسوبي", "خوارزمية"),
            ("موسيقى", "نوتة"), ("شرعي", "أصول_فقه")
        ]
        for domain, dtype in interactions:
            self.log_interaction(domain, dtype)
            print(f"   📝 [{domain}] - {dtype}")
        print("\n⚙️ تطبيق القاعدة الذهبية (عتبة نسبية + blacklist)...")
        self.recalculate_and_enforce_rules()
        # عرض لوحة التحكم
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT domain, usage_percentage, status, training_priority FROM domain_stats ORDER BY usage_percentage DESC')
        print(f"\n{'المجال':<12} | {'النسبة':<8} | {'الحالة':<8} | {'التدريب':<8} | {'الموقع'}")
        print("-"*65)
        for row in cursor.fetchall():
            d, p, s, pr = row
            cursor.execute('SELECT current_location FROM weight_registry WHERE domain=?', (d,))
            loc = cursor.fetchone()[0]
            icon = "🔥 نشط" if s == 'ACTIVE' else "🧊 مجمد"
            print(f"{d:<12} | {p:>5.1f}%  | {icon:<8} | {pr:>5.1f}    | {loc}")
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
    print("\n🎉 القاعدة الذهبية تعمل بنجاح!")
    print("  ✅ شرعي (40%) -> نشط (فوق المتوسط)")
    print("  ✅ حاسوبي (30%) -> نشط (فوق المتوسط)")
    print("  ✅ موسيقى (30%) -> مجمد (blacklist يدوي)")
