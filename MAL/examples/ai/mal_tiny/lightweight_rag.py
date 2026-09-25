#!/usr/bin/env python3
"""
الاسترجاع المعزز الخفيف - حقن السياق الذكي بدلاً من تدريب النموذج على كل شيء
يبقي النموذج صغيراً لكن معرفته لانهائية
"""
import sqlite3
import re
from typing import List, Dict, Tuple
import math
class LightweightRAG:
    def __init__(self, db_path: str = "mal_knowledge_base.db"):
        self.db_path = db_path
        self._init_knowledge_base()
    def _init_knowledge_base(self):
        """تهيئة قاعدة المعرفة بالفهرس المتجهي الخفيف"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('''CREATE TABLE IF NOT EXISTS knowledge_base (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            root TEXT,
            domain TEXT,
            region TEXT,
            keywords TEXT,
            embedding TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )''')
        # إضافة الجذور الـ 2000+ إلى قاعدة المعرفة
        # (سنستخدم عينة تمثيلية هنا)
        roots_data = [
            ("ح س ب", "رياضي", "EVALUATE", "حساب,احسب,محسوب,تقييم"),
            ("ج م ع", "رياضي", "SUM_OPERATION", "جمع,مجموع,جامع,إضافة"),
            ("ض ر ب", "رياضي", "MATMUL", "ضرب,جداء,مصفوفة,ضرب مصفوفات"),
            ("ف ق ه", "شرعي", "ISLAMIC_JURISPRUDENCE", "فقه,فقيه,فتوى,حكم,اجتهاد"),
            ("ش ب ك", "حاسوبي", "NEURAL_NETWORK", "شبكة,عصبية,ذكاء,تعلم"),
            ("ط ب ق", "حاسوبي", "LAYER", "طبقة,مستوى,درجة"),
            ("ن ش ط", "حاسوبي", "ACTIVATION_FUNC", "تنشيط,دالة,ReLU,Sigmoid"),
            ("و ز ن", "حاسوبي", "WEIGHT", "وزن,أوزان,معامل"),
            ("م ر ض", "طبي", "DISEASE", "مرض,علاج,دواء,تشخيص"),
            ("خ ل ي", "طبي", "CELL", "خلية,نسيج,عضو"),
        ]
        for root, domain, region, keywords in roots_data:
            # إنشاء embedding بسيط (تمثيل متجهي خفيف)
            embedding = self._create_simple_embedding(keywords)
            cursor.execute('''INSERT OR IGNORE INTO knowledge_base 
                (root, domain, region, keywords, embedding) VALUES (?,?,?,?,?)''',
                (root, domain, region, keywords, embedding))
        conn.commit()
        conn.close()
        print(f"✅ تم تهيئة قاعدة المعرفة بـ {len(roots_data)} جذر")
    def _create_simple_embedding(self, text: str) -> str:
        """إنشاء embedding بسيط (bag of words)"""
        words = set(text.split(','))
        return ','.join(sorted(words))
    def _calculate_similarity(self, query: str, keywords: str) -> float:
        """حساب التشابه بين الاستعلام والكلمات المفتاحية"""
        query_words = set(query.lower().split())
        keyword_words = set(keywords.lower().split(','))
        intersection = query_words & keyword_words
        union = query_words | keyword_words
        if not union:
            return 0.0
        return len(intersection) / len(union)
    def retrieve_context(self, query: str, top_k: int = 3) -> List[Dict]:
        """استرجاع أفضل k جذور مطابقة للاستعلام"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT root, domain, region, keywords FROM knowledge_base')
        all_roots = cursor.fetchall()
        conn.close()
        # حساب التشابه مع كل جذر
        similarities = []
        for root, domain, region, keywords in all_roots:
            similarity = self._calculate_similarity(query, keywords)
            if similarity > 0:
                similarities.append({
                    'root': root,
                    'domain': domain,
                    'region': region,
                    'keywords': keywords,
                    'similarity': similarity
                })
        # ترتيب حسب التشابه
        similarities.sort(key=lambda x: x['similarity'], reverse=True)
        # إرجاع أفضل k
        return similarities[:top_k]
    def format_context_for_llm(self, retrieved: List[Dict]) -> str:
        """تنسيق السياق للحقن في نموذج الذكاء الاصطناعي"""
        if not retrieved:
            return "لا توجد معلومات ذات صلة في قاعدة المعرفة."
        context = "المعلومات ذات الصلة من قاعدة المعرفة:\n\n"
        for i, item in enumerate(retrieved, 1):
            context += f"{i}. الجذر: {item['root']}\n"
            context += f"   المجال: {item['domain']}\n"
            context += f"   المنطقة: {item['region']}\n"
            context += f"   الكلمات المفتاحية: {item['keywords']}\n"
            context += f"   درجة التطابق: {item['similarity']:.2f}\n\n"
        return context
if __name__ == "__main__":
    rag = LightweightRAG()
    test_queries = [
        "كيف أحسب مجموع مصفوفة؟",
        "ما هو حكم الصلاة؟",
        "اشرح مفهوم الشبكة العصبية",
        "ما هي أعراض المرض؟"
    ]
    print("\n🧪 اختبار RAG الخفيف:")
    for query in test_queries:
        print(f"\n📝 الاستعلام: {query}")
        retrieved = rag.retrieve_context(query, top_k=3)
        context = rag.format_context_for_llm(retrieved)
        print(context)
