#!/usr/bin/env python3
"""
الذاكرة المؤقتة الحتمية - تعالج العبارات الشائعة فوراً بدون استدعاء النموذج
توفر 100% من موارد CPU/RAM للأنماط المتكررة
"""
import sqlite3
import re
from typing import Dict, Optional, List
class DeterministicCache:
    def __init__(self, db_path: str = "mal_deterministic_cache.db"):
        self.db_path = db_path
        self._init_cache()
    def _init_cache(self):
        """تهيئة الذاكرة المؤقتة بأنماط حتمية شائعة"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('''CREATE TABLE IF NOT EXISTS mal_cache (
            pattern TEXT PRIMARY KEY,
            mal_code TEXT,
            region TEXT,
            domain TEXT,
            hit_count INTEGER DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )''')
        # أنماط حتمية لا تحتاج لذكاء اصطناعي
        patterns = [
            # أنماط رياضية
            (r"احسب.*(\d+)\s*\+\s*(\d+)", "EVALUATE", "رياضي", 
             "fn main() -> i32 {{\n    let a: i32 = {0};\n    let b: i32 = {1};\n    let result: i32 = a + b;\n    printf(\"النتيجة: %d\\n\", result);\n    return 0;\n}}"),
            (r"مجموع.*(\d+).*و.*(\d+)", "SUM_OPERATION", "رياضي",
             "fn main() -> i32 {{\n    let sum: i32 = {0} + {1};\n    printf(\"المجموع: %d\\n\", sum);\n    return 0;\n}}"),
            (r"ضرب.*(\d+).*في.*(\d+)", "MATMUL", "رياضي",
             "fn main() -> i32 {{\n    let product: i32 = {0} * {1};\n    printf(\"الجداء: %d\\n\", product);\n    return 0;\n}}"),
            # أنماط فقهية
            (r"حكم.*الصلاة", "ISLAMIC_JURISPRUDENCE", "شرعي",
             "fn main() -> i32 {{\n    printf(\"الصلاة واجبة في أوقاتها المحددة\\n\");\n    printf(\"شروط الصحة: الطهارة، استقبال القبلة، ستر العورة\\n\");\n    return 0;\n}}"),
            (r"مفهوم.*الاجتهاد", "ISLAMIC_JURISPRUDENCE", "شرعي",
             "fn main() -> i32 {{\n    printf(\"الاجتهاد: بذل الفقيه وسعه في استنباط الأحكام الشرعية\\n\");\n    printf(\"شروط المجتهد: العلم بالكتاب والسنة، معرفة أصول الفقه\\n\");\n    return 0;\n}}"),
            # أنماط برمجية
            (r"دالة.*جمع", "FUNCTION", "حاسوبي",
             "fn add(a: i32, b: i32) -> i32 {{\n    return a + b;\n}}\n\nfn main() -> i32 {{\n    let result: i32 = add(5, 7);\n    printf(\"النتيجة: %d\\n\", result);\n    return 0;\n}}"),
            (r"شبكة.*عصبية", "NEURAL_NETWORK", "حاسوبي",
             "fn relu(x: i32) -> i32 {{\n    if x > 0 {{ return x; }} else {{ return 0; }}\n}}\n\nfn main() -> i32 {{\n    let input: i32 = 5;\n    let weight: i32 = 2;\n    let bias: i32 = -3;\n    let z: i32 = input * weight + bias;\n    let output: i32 = relu(z);\n    printf(\"مخرج الشبكة: %d\\n\", output);\n    return 0;\n}}"),
        ]
        for pattern, region, domain, code_template in patterns:
            cursor.execute('''INSERT OR IGNORE INTO mal_cache 
                (pattern, mal_code, region, domain) VALUES (?,?,?,?)''',
                (pattern, code_template, region, domain))
        conn.commit()
        conn.close()
        print(f"✅ تم تهيئة الذاكرة المؤقتة بـ {len(patterns)} نمط حتمي")
    def try_cache(self, text: str) -> Optional[Dict]:
        """محاولة الاسترجاع من الذاكرة المؤقتة"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT pattern, mal_code, region, domain FROM mal_cache')
        patterns = cursor.fetchall()
        for pattern, code_template, region, domain in patterns:
            match = re.search(pattern, text, re.IGNORECASE)
            if match:
                # استخراج المعاملات من النص
                args = match.groups()
                # ملء القالب بالمعاملات
                if args:
                    try:
                        mal_code = code_template.format(*args)
                    except:
                        mal_code = code_template
                else:
                    mal_code = code_template
                # زيادة عداد الاستخدام
                cursor.execute('UPDATE mal_cache SET hit_count = hit_count + 1 WHERE pattern = ?', (pattern,))
                conn.commit()
                conn.close()
                return {
                    'mal_code': mal_code,
                    'region': region,
                    'domain': domain,
                    'certainty': 1.0,
                    'is_deterministic': True,
                    'from_cache': True
                }
        conn.close()
        return None
    def get_cache_stats(self) -> Dict:
        """إحصائيات الذاكرة المؤقتة"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT COUNT(*), SUM(hit_count) FROM mal_cache')
        total_patterns, total_hits = cursor.fetchone()
        cursor.execute('SELECT pattern, hit_count FROM mal_cache ORDER BY hit_count DESC LIMIT 5')
        top_patterns = cursor.fetchall()
        conn.close()
        return {
            'total_patterns': total_patterns,
            'total_hits': total_hits,
            'top_patterns': top_patterns
        }
if __name__ == "__main__":
    cache = DeterministicCache()
    test_queries = [
        "احسب قيمة 15 + 25",
        "ما حكم الصلاة في وقتها؟",
        "مجموع 10 و 20",
        "اشرح لي مفهوم الاجتهاد",
        "كيف أكتب دالة جمع؟"
    ]
    print("\n🧪 اختبار الذاكرة المؤقتة:")
    for query in test_queries:
        result = cache.try_cache(query)
        if result:
            print(f"\n✅ '{query}'")
            print(f"   المنطقة: {result['region']}")
            print(f"   من الذاكرة المؤقتة: نعم")
        else:
            print(f"\n❌ '{query}'")
            print(f"   من الذاكرة المؤقتة: لا")
    stats = cache.get_cache_stats()
    print(f"\n📊 إحصائيات الذاكرة المؤقتة:")
    print(f"   إجمالي الأنماط: {stats['total_patterns']}")
    print(f"   إجمالي الاستخدامات: {stats['total_hits']}")
    print(f"   أكثر الأنماط استخداماً:")
    for pattern, hits in stats['top_patterns']:
        print(f"      - {pattern}: {hits} مرة")
