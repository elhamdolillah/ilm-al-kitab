#!/usr/bin/env python3
"""
وحدة البحث التلقائي الذكي مع الاكتشاف الديناميكي للمجالات (Dynamic Domain Discovery)
وضمان عدم التكرار بنسبة 100%
"""
import requests
import json
import re
import os
import sqlite3
from typing import Dict, Optional, List, Set
from bs4 import BeautifulSoup
import pyarabic.araby as araby
class AutoLookup:
    def __init__(self, db_path: str = "grammar_math_rules.db"):
        self.db_path = db_path
        self._init_db()
        # المجالات الأساسية الثابتة (دستور MAL الأساسي)
        self.core_domains = {
            'رياضي': ['مصفوفة', 'متجه', 'تكامل', 'مشتق', 'احتمال', 'تباين', 'فضاء', 'بُعد', 'نواة', 'دالة', 'معادلة'],
            'حاسوبي': ['خوارزمية', 'بيانات', 'نموذج', 'شبكة', 'خادم', 'برمجية', 'نظام', 'ذكاء'],
            'طبي': ['مريض', 'علاج', 'دواء', 'مرض', 'تشخيص', 'جراحة'],
            'زراعي': ['محصول', 'تربة', 'ري', 'سماد', 'حصاد', 'بذر'],
            'قانوني': ['عقد', 'حكم', 'محكمة', 'دعوى', 'حق', 'قانون', 'تشريع'],
            'جيولوجي': ['صخر', 'معدن', 'زلزال', 'بركان', 'طبقة', 'أحفورة'],
            'بيولوجي': ['خلية', 'جين', 'كائن', 'تكاثر', 'وراثة', 'تطور', 'نسيج'],
            'كيميائي': ['عنصر', 'مركب', 'تفاعل', 'ذرة', 'جزيء', 'رابطة', 'حمض'],
            'فيزيائي': ['طاقة', 'قوة', 'كتلة', 'سرعة', 'تردد', 'ضوء', 'موجة', 'جاذبية']
        }
        # مجموعة المجالات المكتشفة ديناميكياً
        self.dynamic_domains: Set[str] = set()
        # خريطة الجذور للمناطق الرياضية الأساسية
        self.math_region_rules = {
            'ح س ب': 'EVALUATE', 'ج م ع': 'SUM_OPERATION', 'ط ر ح': 'SUB_OPERATION',
            'ض ر ب': 'MATMUL', 'ق س م': 'DIV_OPERATION', 'ص ف ف': 'MATRIX_TENSOR',
            'و ج ه': 'VECTOR_TENSOR', 'ح د د': 'DETERMINANT', 'ع ك س': 'INVERSE',
            'ن ق ل': 'TRANSPOSE', 'د و ل': 'FUNCTION', 'ح و ل': 'TRANSFORM',
            'ك م ل': 'INTEGRAL', 'ش ت ق': 'DERIVATIVE', 'ش ب ك': 'NEURAL_NETWORK',
            'ع ص ب': 'NEURAL_WEIGHTS', 'ط ب ق': 'LAYER_DIMENSION', 'ن ش ط': 'ACTIVATION_FUNC',
            'و ز ن': 'WEIGHT_MATRIX', 'ن ح ي ز': 'BIAS_VECTOR', 'ق ي م': 'SCALAR_VALUE',
            'ن س ب': 'RATIO', 'د ر ب': 'OPTIMIZATION_STEP', 'ح س ن': 'OPTIMIZATION',
            'خ ط أ': 'LOSS_FUNCTION', 'ف ض ي': 'VECTOR_SPACE', 'أ س س': 'BASIS',
            'ب ع د': 'DIMENSION', 'ر ت ب': 'RANK', 'ن و ي': 'KERNEL', 'ن ق ط': 'POINT',
            'خ ط ط': 'LINE', 'س ط ح': 'SURFACE', 'ز و ي': 'ANGLE', 'د و ر': 'CIRCLE',
            'ط ا ق': 'ENERGY', 'ق و ي': 'FORCE', 'ك ت ل': 'MASS', 'س ر ع': 'VELOCITY',
            'ت ر د د': 'FREQUENCY'
        }
    def _init_db(self):
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS grammar_math_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                original_word TEXT UNIQUE,
                root TEXT,
                morph_pattern TEXT,
                pos TEXT,
                source TEXT,
                math_region TEXT,
                domain TEXT,
                compression_token TEXT,
                certainty_score REAL DEFAULT 1.0,
                auto_generated INTEGER DEFAULT 0
            )
        ''')
        for col in ['domain', 'auto_generated']:
            try:
                cursor.execute(f'ALTER TABLE grammar_math_rules ADD COLUMN {col} TEXT')
            except sqlite3.OperationalError:
                pass
        conn.commit()
        conn.close()
    def extract_root(self, word: str) -> Optional[str]:
        word = araby.strip_tashkeel(word)
        word = re.sub(r'^ال', '', word)
        suffixes = ['ة', 'ات', 'ين', 'ون', 'ان', 'ها', 'هم', 'هن', 'نا', 'كم', 'كن']
        for suffix in suffixes:
            if word.endswith(suffix) and len(word) > len(suffix) + 2:
                word = word[:-len(suffix)]
                break
        prefixes = ['م', 'ت', 'ي', 'ن', 'ا', 'س', 'است']
        for prefix in prefixes:
            if word.startswith(prefix) and len(word) > len(prefix) + 2:
                word = word[len(prefix):]
                break
        if len(word) >= 3:
            if len(word) == 3: return f"{word[0]} {word[1]} {word[2]}"
            elif len(word) == 4: return f"{word[0]} {word[1]} {word[3]}"
            elif len(word) >= 5: return f"{word[0]} {word[2]} {word[4]}"
        return None
    def infer_domain_dynamically(self, word: str, meaning_text: str) -> str:
        """استنتاج المجال ديناميكياً من سياق التعريف أو الكلمة"""
        # 1. التحقق من المجالات الأساسية الثابتة أولاً
        for dom, keywords in self.core_domains.items():
            if any(kw in word for kw in keywords):
                return dom
        # 2. تحليل نص التعريف المستخرج من الإنترنت للبحث عن مؤشرات المجال
        if meaning_text:
            # البحث عن النصوص بين أقواس، وهي الطريقة الشائعة في القواميس العربية لتحديد المجال
            parentheses_matches = re.findall(r'\((.*?)\)', meaning_text)
            for match in parentheses_matches:
                match = match.strip()
                # تطبيع أسماء المجالات الشائعة
                if any(kw in match for kw in ['فقه', 'شريعة', 'دين', 'إسلام', 'حديث']): return 'شرعي'
                if any(kw in match for kw in ['كهرباء', 'كهربائي', 'تيار', 'جهد']): return 'كهرباء'
                if any(kw in match for kw in ['إلكترونيات', 'إلكترونيك', 'دوائر', 'ترانزستور']): return 'إلكترونيك'
                if any(kw in match for kw in ['فضاء', 'فلك', 'نجوم', 'كواكب', 'مدار']): return 'فضاء'
                if any(kw in match for kw in ['نبات', 'زراعة', 'محصول', 'تربة']): return 'زراعي'
                if any(kw in match for kw in ['طب', 'مرض', 'جسم', 'تشريح']): return 'طبي'
                if any(kw in match for kw in ['قانون', 'محكمة', 'جنائي', 'مدني']): return 'قانوني'
                # 3. الاكتشاف الديناميكي الحقيقي: إذا كان النص بين القوسين مجالاً جديداً وغير معروف
                if len(match) > 2 and len(match) < 40 and match not in self.core_domains:
                    self.dynamic_domains.add(match)
                    return match # إرجاع المجال المكتشف حديثاً
        # 4. fallback: إذا لم يتم العثور على أي مؤشر، نعود للجذر أو "عام"
        return 'عام'
    def determine_region(self, root: Optional[str], domain: str) -> str:
        """تحديد المنطقة بناءً على الجذر أو المجال"""
        if root and root in self.math_region_rules:
            return self.math_region_rules[root]
        # تعيين منطقة عامة بناءً على المجال المكتشف
        domain_to_region = {
            'شرعي': 'ISLAMIC_JURISPRUDENCE',
            'كهرباء': 'ELECTRICAL_ENGINEERING',
            'إلكترونيك': 'ELECTRONICS',
            'فضاء': 'ASTRONOMY_SPACE',
            'زراعي': 'AGRICULTURE',
            'طبي': 'MEDICAL',
            'قانوني': 'LEGAL',
            'جيولوجي': 'GEOLOGY',
            'بيولوجي': 'BIOLOGY',
            'كيميائي': 'CHEMISTRY',
            'فيزيائي': 'PHYSICS',
            'حاسوبي': 'COMPUTER_SCIENCE',
            'رياضي': 'MATHEMATICS'
        }
        return domain_to_region.get(domain, 'GENERAL')
    def search_online(self, word: str) -> Optional[Dict]:
        try:
            url = f"https://www.almaany.com/ar/dict/ar-ar/{word}/"
            headers = {'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'}
            response = requests.get(url, headers=headers, timeout=10)
            if response.status_code == 200:
                soup = BeautifulSoup(response.content, 'html.parser')
                meaning_div = soup.find('div', class_='meaning-block')
                if meaning_div:
                    root_elem = soup.find('span', class_='root-word')
                    return {
                        'meaning': meaning_div.get_text(strip=True)[:300],
                        'root': root_elem.get_text(strip=True) if root_elem else None,
                        'source': 'almaany'
                    }
        except Exception:
            pass
        return None
    def lookup_and_add(self, word: str, dict_path: str) -> Optional[Dict]:
        # 1. فحص صارم لعدم التكرار في قاعدة البيانات
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT original_word FROM grammar_math_rules WHERE original_word = ?', (word,))
        if cursor.fetchone():
            conn.close()
            return None 
        # 2. فحص صارم لعدم التكرار في JSON
        if os.path.exists(dict_path):
            with open(dict_path, 'r', encoding='utf-8') as f:
                dictionary = json.load(f)
            if any(e['word'] == word for e in dictionary):
                conn.close()
                return None 
        else:
            dictionary = []
        print(f"🔍 بحث تلقائي واكتشاف مجال لـ: '{word}'")
        result = self.search_online(word)
        root = result['root'] if result and result.get('root') else self.extract_root(word)
        meaning = result['meaning'] if result else ''
        # الاستنتاج الديناميكي للمجال
        domain = self.infer_domain_dynamically(word, meaning)
        region = self.determine_region(root, domain)
        entry = {
            'word': word,
            'root': root or 'N/A',
            'pattern': 'مشتق',
            'pos': 'اسم',
            'source': result['source'] if result else 'auto',
            'math_region': region,
            'domain': domain,
            'meaning': meaning,
            'auto_generated': True
        }
        # 3. الحفظ في JSON
        dictionary.append(entry)
        with open(dict_path, 'w', encoding='utf-8') as f:
            json.dump(dictionary, f, ensure_ascii=False, indent=2)
        # 4. الحفظ في قاعدة البيانات
        cursor.execute('''
            INSERT INTO grammar_math_rules 
            (original_word, root, morph_pattern, pos, source, math_region, domain, compression_token, certainty_score, auto_generated)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ''', (word, entry['root'], entry['pattern'], entry['pos'], entry['source'], 
              entry['math_region'], entry['domain'], entry['math_region'], 0.95, 1))
        conn.commit()
        conn.close()
        domain_note = " (مجال مكتشف ديناميكياً!)" if domain in self.dynamic_domains else ""
        print(f"✅ تمت الإضافة: {word} -> {region} [{domain}]{domain_note} (حتمية 95%)")
        return entry
    def batch_lookup(self, words: List[str], dict_path: str) -> List[Dict]:
        results = []
        for word in words:
            res = self.lookup_and_add(word, dict_path)
            if res: results.append(res)
            import time; time.sleep(0.5)
        return results
if __name__ == "__main__":
    lookup = AutoLookup()
    test_words = ['اجتهاد', 'مدار', 'مكثف', 'ترانزستور', 'فقه', 'جاذبية']
    print("🤖 اختبار الاكتشاف الديناميكي للمجالات:")
    lookup.batch_lookup(test_words, 'arabic_morphology_dict.json')
    print(f"\n🌟 المجالات الجديدة التي اكتشفها المحرك تلقائياً: {lookup.dynamic_domains}")
