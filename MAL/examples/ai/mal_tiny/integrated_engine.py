#!/usr/bin/env python3
"""
المحرك المتكامل: يربط DynamicSpecializationEngine مع DeterministicGrammarEngine
"""
import sys
sys.path.insert(0, '/root/ilm-al-kitab/MAL/examples/ai/mal_tiny')
from dynamic_specialization_engine import DynamicSpecializationEngine
from grammar_compressor import DeterministicGrammarEngine
from expanded_arabic_roots import EXPANDED_ROOTS, get_total_roots_count
class IntegratedEngine:
    def __init__(self, user_id: str = "user_001"):
        self.user_id = user_id
        self.specialization = DynamicSpecializationEngine(user_id=user_id)
        self.grammar = DeterministicGrammarEngine(auto_lookup_enabled=True)
        # تحميل الجذور الموسعة في المحرك اللغوي
        self._load_expanded_roots()
    def _load_expanded_roots(self):
        """تحميل الجذور الموسعة في المحرك اللغوي"""
        print(f"🌳 تحميل {get_total_roots_count()} جذر موسع في المحرك اللغوي...")
        # إضافة الجذور إلى قاموس المحرك اللغوي
        for root, data in EXPANDED_ROOTS.items():
            for word in data['words']:
                if word not in self.grammar.lexicon:
                    self.grammar.lexicon[word] = {
                        'root': root,
                        'pattern': 'مشتق',
                        'pos': 'اسم',
                        'source': 'expanded_roots',
                        'math_region': data['region'],
                        'domain': data['domain']
                    }
        print(f"✅ تم تحميل {len(self.grammar.lexicon)} كلمة في المحرك اللغوي")
    def process_query(self, text: str) -> dict:
        """معالجة استعلام المستخدم مع تطبيق القاعدة الذهبية"""
        print(f"\n{'='*80}")
        print(f"📝 الاستعلام: {text}")
        print(f"{'='*80}")
        # 1. تحليل النص بالمحرك اللغوي
        result = self.grammar.analyze_and_compress_with_structure(text)
        # 2. تسجيل التفاعل مع محرك التخصيص
        detected_domain = result.get('discovered_domains', ['عام'])[0] if result.get('discovered_domains') else 'عام'
        self.specialization.log_interaction(detected_domain, 'query')
        # 3. تطبيق القاعدة الذهبية
        self.specialization.recalculate_and_enforce_rules()
        # 4. عرض النتائج
        print(f"\n✅ التسلسل المضغوط: {result['compressed_sequence']}")
        print(f"✅ نسبة الحتمية: {result['certainty_percentage']:.1%}")
        print(f"✅ المجالات المكتشفة: {result['discovered_domains']}")
        if result['hierarchical_responses']:
            resp = result['hierarchical_responses'][0]
            print(f"\n📚 التصنيف الهيكلي:")
            print(f"   📂 القسم: {resp['hierarchical_classification']['library_section']}")
            print(f"   📌 المحور: {resp['hierarchical_classification']['main_axis']}")
        return result
    def simulate_user_session(self, queries: list, blacklist: list = None):
        """محاكاة جلسة مستخدم كاملة"""
        if blacklist:
            self.specialization.set_blacklist(blacklist)
        for query in queries:
            self.process_query(query)
        # عرض إحصائيات التخصيص
        print(f"\n{'='*80}")
        print("📊 إحصائيات التخصيص بعد الجلسة:")
        print(f"{'='*80}")
        import sqlite3
        conn = sqlite3.connect(self.specialization.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT domain, usage_percentage, status FROM domain_stats ORDER BY usage_percentage DESC')
        print(f"{'المجال':<15} | {'النسبة':<10} | {'الحالة':<10}")
        print("-"*45)
        for domain, percentage, status in cursor.fetchall():
            icon = "🔥 نشط" if status == 'ACTIVE' else "🧊 مجمد"
            print(f"{domain:<15} | {percentage:>7.1f}%  | {icon}")
        conn.close()
if __name__ == "__main__":
    # تنظيف قواعد البيانات القديمة
    import os
    for db in ["user_specialization.db", "grammar_math_rules.db"]:
        if os.path.exists(db):
            os.remove(db)
    # إنشاء المحرك المتكامل
    engine = IntegratedEngine(user_id="user_fiqh_dev")
    # محاكاة جلسة مستخدم
    queries = [
        "ما حكم الصلاة في وقتها؟",
        "كيف أكتب دالة في Python؟",
        "ما هي آلات الموسيقى؟",
        "اشرح لي مفهوم الاجتهاد",
        "كيف أحسب قيمة مجموع المصفوفة؟",
        "من هو ملحن هذه الموسيقى؟",
    ]
    engine.simulate_user_session(queries, blacklist=["موسيقى"])
    print("\n🎉 تم بنجاح! المحرك المتكامل يعمل:")
    print(f"  ✅ {get_total_roots_count()} جذر عربي موسع")
    print(f"  ✅ ربط محرك التخصيص مع المحرك اللغوي")
    print(f"  ✅ تطبيق القاعدة الذهبية (الوسيط + blacklist)")
