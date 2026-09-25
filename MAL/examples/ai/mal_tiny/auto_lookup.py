#!/usr/bin/env python3
"""
وحدة البحث التلقائي الذكي مع تصنيف المجالات وضمان عدم التكرار
"""
import requests
import json
import re
import os
import sqlite3
from typing import Dict, Optional, List
from bs4 import BeautifulSoup
import pyarabic.araby as araby
class AutoLookup:
    def __init__(self, db_path: str = "grammar_math_rules.db"):
        self.db_path = db_path
        self._init_db()
        # خريطة تصنيف المجالات (دستور MAL الموسع)
        self.domain_keywords = {
            'رياضي': ['مصفوفة', 'متجه', 'تكامل', 'مشتق', 'احتمال', 'تباين', 'فضاء', 'بُعد', 'نواة', 'دالة', 'معادلة'],
            'طبي': ['مريض', 'علاج', 'دواء', 'مرض', 'تشخيص', 'عرض', 'جراحة', 'صيدلة'],
            'زراعي': ['محصول', 'تربة', 'ري', 'سماد', 'حصاد', 'بذر', 'آفة', 'نبات'],
            'قانوني': ['عقد', 'حكم', 'محكمة', 'دعوى', 'حق', 'قانون', 'تشريع', 'عقوبة'],
            'جيولوجي': ['صخر', 'معدن', 'زلزال', 'بركان', 'طبقة', 'أحفورة', 'تضاريس'],
            'بيولوجي': ['خلية', 'جين', 'كائن', 'تكاثر', 'وراثة', 'تطور', 'نسيج', 'عضو'],
            'كيميائي': ['عنصر', 'مركب', 'تفاعل', 'ذرة', 'جزيء', 'رابطة', 'حمض', 'قاعدة'],
            'فيزيائي': ['طاقة', 'قوة', 'كتلة', 'سرعة', 'تردد', 'ضوء', 'موجة', 'جاذبية'],
            'حاسوبي': ['خوارزمية', 'بيانات', 'نموذج', 'شبكة', 'خادم', 'برمجية', 'نظام']
        }
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
        # إضافة الأعمدة الجديدة إذا لم تكن موجودة
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
    def determine_domain_and_region(self, word: str, root: Optional[str]) -> tuple:
        domain = 'عام'
        for dom, keywords in self.domain_keywords.items():
            if any(kw in word for kw in keywords):
                domain = dom
                break
        region = 'GENERAL'
        if root and root in self.math_region_rules:
            region = self.math_region_rules[root]
        elif domain == 'رياضي':
            region = 'MATHEMATICS'
        elif domain == 'طبي':
            region = 'MEDICAL'
        elif domain == 'زراعي':
            region = 'AGRICULTURE'
        elif domain == 'قانوني':
            region = 'LEGAL'
        elif domain == 'جيولوجي':
            region = 'GEOLOGY'
        elif domain == 'بيولوجي':
            region = 'BIOLOGY'
        elif domain == 'كيميائي':
            region = 'CHEMISTRY'
        elif domain == 'فيزيائي':
            region = 'PHYSICS'
        elif domain == 'حاسوبي':
            region = 'COMPUTER_SCIENCE'
        return domain, region
    def search_online(self, word: str) -> Optional[Dict]:
        try:
            url = f"https://www.almaany.com/ar/dict/ar-ar/{word}/"
            headers = {'User-Agent': 'Mozilla/5.0'}
            response = requests.get(url, headers=headers, timeout=10)
            if response.status_code == 200:
                soup = BeautifulSoup(response.content, 'html.parser')
                meaning_div = soup.find('div', class_='meaning-block')
                if meaning_div:
                    root_elem = soup.find('span', class_='root-word')
                    return {
                        'meaning': meaning_div.get_text(strip=True)[:200],
                        'root': root_elem.get_text(strip=True) if root_elem else None,
                        'source': 'almaany'
                    }
        except Exception:
            pass
        return None
    def lookup_and_add(self, word: str, dict_path: str) -> Optional[Dict]:
        # 1. فحص عدم التكرار في قاعدة البيانات
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT original_word FROM grammar_math_rules WHERE original_word = ?', (word,))
        if cursor.fetchone():
            conn.close()
            return None # موجود مسبقاً
        # 2. فحص عدم التكرار في JSON
        if os.path.exists(dict_path):
            with open(dict_path, 'r', encoding='utf-8') as f:
                dictionary = json.load(f)
            if any(e['word'] == word for e in dictionary):
                conn.close()
                return None # موجود مسبقاً
        else:
            dictionary = []
        print(f"🔍 بحث تلقائي عن: {word}")
        result = self.search_online(word)
        root = result['root'] if result and result.get('root') else self.extract_root(word)
        domain, region = self.determine_domain_and_region(word, root)
        entry = {
            'word': word,
            'root': root or 'N/A',
            'pattern': 'مشتق',
            'pos': 'اسم',
            'source': result['source'] if result else 'auto',
            'math_region': region,
            'domain': domain,
            'meaning': result['meaning'] if result else '',
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
        print(f"✅ تمت الإضافة: {word} -> {region} [{domain}] (حتمية 95%)")
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
    test_words = ['تدرج', 'انتشار', 'خلفي', 'تحسين', 'تكراري', 'محصول', 'عقد']
    print("🤖 اختبار البحث التلقائي:")
    lookup.batch_lookup(test_words, 'arabic_morphology_dict.json')
