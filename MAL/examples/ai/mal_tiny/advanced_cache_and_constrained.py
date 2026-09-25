#!/usr/bin/env python3
import sqlite3
import re
from typing import Dict, Optional, List
class AdvancedIntelligenceEngine:
    def __init__(self, db_path: str = "advanced_intelligence_cache.db"):
        self.db_path = db_path
        self._init_system()
    def _init_system(self):
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('''CREATE TABLE IF NOT EXISTS semantic_cache (
            query_pattern TEXT PRIMARY KEY, mal_code TEXT, region TEXT, domain TEXT, hit_count INTEGER DEFAULT 0
        )''')
        # 🔑 الإصلاح الجذري 1: [^\d]* تمنع الجشع وتلتقط الأرقام الكاملة فقط (15 و 25)
        # 🔑 الإصلاح الجذري 2: {{ }} تُستخدم فقط في القوالب الديناميكية، و {} في الثابتة
        templates = [
            (r"احسب[^\d]*(\d+)[^\d]*\+[^\d]*(\d+)", "EVALUATE", "رياضي", 
             "fn main() -> i32 {{\n    let a: i32 = {0};\n    let b: i32 = {1};\n    let result: i32 = a + b;\n    printf(\"النتيجة: %d\\n\", result);\n    return 0;\n}}"),
            (r"مجموع[^\d]*(\d+)[^\d]*و[^\d]*(\d+)", "SUM_OPERATION", "رياضي",
             "fn main() -> i32 {{\n    let sum: i32 = {0} + {1};\n    printf(\"المجموع: %d\\n\", sum);\n    return 0;\n}}"),
            (r"حكم.*الصلاة", "ISLAMIC_JURISPRUDENCE", "شرعي",
             "fn main() -> i32 {\n    printf(\"الصلاة واجبة في أوقاتها المحددة\\n\");\n    printf(\"شروط الصحة: الطهارة، استقبال القبلة، ستر العورة\\n\");\n    return 0;\n}"),
            (r"مفهوم.*الاجتهاد", "ISLAMIC_JURISPRUDENCE", "شرعي",
             "fn main() -> i32 {\n    printf(\"الاجتهاد: بذل الفقيه وسعه في استنباط الأحكام الشرعية من الأدلة\\n\");\n    return 0;\n}"),
            (r"شبكة.*عصبية", "NEURAL_NETWORK", "حاسوبي",
             "fn relu(x: i32) -> i32 {\n    if x > 0 { return x; } else { return 0; }\n}\nfn main() -> i32 {\n    let z: i32 = 5 * 2 + (-3);\n    printf(\"مخرج الشبكة: %d\\n\", relu(z));\n    return 0;\n}")
        ]
        for pattern, region, domain, code in templates:
            cursor.execute('INSERT OR REPLACE INTO semantic_cache (query_pattern, mal_code, region, domain) VALUES (?,?,?,?)',
                (pattern, code, region, domain))
        conn.commit()
        conn.close()
        print(f"✅ تم تهيئة النظام بـ {len(templates)} قالب مصحح جذرياً")
    def try_semantic_cache(self, text: str) -> Optional[Dict]:
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT query_pattern, mal_code, region, domain FROM semantic_cache')
        for pattern, code_template, region, domain in cursor.fetchall():
            match = re.search(pattern, text, re.IGNORECASE)
            if match:
                args = match.groups()
                # 🔑 الإصلاح الجذري: format() يحول {{ إلى { دائماً، حتى لو كانت args فارغة
                try:
                    mal_code = code_template.format(*args)
                except:
                    mal_code = code_template
                cursor.execute('UPDATE semantic_cache SET hit_count = hit_count + 1 WHERE query_pattern = ?', (pattern,))
                conn.commit()
                conn.close()
                return {'mal_code': mal_code, 'region': region, 'domain': domain, 'certainty': 1.0, 'is_deterministic': True, 'from_cache': True}
        conn.close()
        return None
