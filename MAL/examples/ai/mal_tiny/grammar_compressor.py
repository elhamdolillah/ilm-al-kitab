#!/usr/bin/env python3
"""
المحرك اللغوي-الرياضي الحتمي (Deterministic Neuro-Grammatical Engine) v4
يدعم: الاكتشاف الديناميكي للمجالات + عدم التكرار + البحث التلقائي عبر الإنترنت
"""
import re
import json
import sqlite3
import pyarabic.araby as araby
from typing import Dict, List, Optional
import os
import requests
from bs4 import BeautifulSoup
class DeterministicGrammarEngine:
    def __init__(self, db_path: str = "grammar_math_rules.db", 
                 dict_path: str = "arabic_morphology_dict.json",
                 auto_lookup_enabled: bool = True):
        self.db_path = db_path
        self.dict_path = dict_path
        self.auto_lookup_enabled = auto_lookup_enabled
        self.lexicon = {}
        self.dynamic_domains = set()
        # المجالات الأساسية الثابتة
        self.core_domains = {
            'رياضي': ['مصفوفة', 'متجه', 'تكامل', 'مشتق', 'احتمال', 'تباين', 'فضاء', 'بُعد', 'نواة', 'دالة', 'معادلة'],
            'حاسوبي': ['خوارزمية', 'بيانات', 'نموذج', 'شبكة', 'خادم', 'برمجية', 'نظام', 'ذكاء'],
            'طبي': ['مريض', 'علاج', 'دواء', 'مرض', 'تشخيص', 'جراحة'],
            'زراعي': ['محصول', 'تربة', 'ري', 'سماد', 'حصاد', 'بذر'],
            'قانوني': ['عقد', 'حكم', 'محكمة', 'دعوى', 'حق', 'قانون', 'تشريع'],
            'جيولوجي': ['صخر', 'معدن', 'زلزال', 'بركان', 'طبقة', 'أحفورة'],
            'بيولوجي': ['خلية', 'جين', 'كائن', 'تكاثر', 'وراثة', 'تطور', 'نسيج'],
            'كيميائي': ['عنصر', 'مركب', 'تفاعل', 'ذرة', 'جزيء', 'رابطة', 'حمض'],
            'فيزيائي': ['طاقة', 'قوة', 'كتلة', 'سرعة', 'تردد', 'ضوء', 'موجة', 'جاذبية'],
            'شرعي': ['فقه', 'شريعة', 'دين', 'إسلام', 'حديث', 'عبادة', 'صلاة'],
            'كهرباء': ['كهرباء', 'تيار', 'جهد', 'مكثف', 'مقاومة'],
            'إلكترونيك': ['إلكترونيات', 'ترانزستور', 'دوائر', 'معالج'],
            'فضاء': ['فضاء', 'فلك', 'نجوم', 'كواكب', 'مدار', 'قمر']
        }
        # خريطة الجذور للمناطق الرياضية
        self.math_region_rules = {
            'ح س ب': 'EVALUATE', 'ج م ع': 'SUM_OPERATION', 'ط ر ح': 'SUB_OPERATION',
            'ض ر ب': 'MATMUL', 'ق س م': 'DIV_OPERATION', 'ص ف ف': 'MATRIX_TENSOR',
            'و ج ه': 'VECTOR_TENSOR', 'ح د د': 'DETERMINANT', 'ع ك س': 'INVERSE',
            'ن ق ل': 'TRANSPOSE', 'د و ل': 'FUNCTION', 'ح و ل': 'TRANSFORM',
            'ك م ل': 'INTEGRAL', 'ش ت ق': 'DERIVATIVE', 'ش ب ك': 'NEURAL_NETWORK',
            'ع ص ب': 'NEURAL_WEIGHTS', 'ط ب ق': 'LAYER_DIMENSION', 'ن ش ط': 'ACTIVATION_FUNC',
            'و ز ن': 'WEIGHT_MATRIX', 'ن ح ي ز': 'BIAS_VECTOR', 'ق ي م': 'SCALAR_VALUE',
            'ف ق ه': 'ISLAMIC_JURISPRUDENCE', 'م د ر': 'ASTRONOMY_SPACE', 'ك ه ر ب': 'ELECTRICAL_ENGINEERING'
        }
        self._load_morphology_dictionary()
        self._init_db()
    def _load_morphology_dictionary(self):
        """تحميل القاموس الصرفي"""
        if os.path.exists(self.dict_path):
            with open(self.dict_path, 'r', encoding='utf-8') as f:
                entries = json.load(f)
                for entry in entries:
                    self.lexicon[entry['word']] = {
                        "root": entry.get('root', 'N/A'),
                        "pattern": entry.get('pattern', 'N/A'),
                        "pos": entry.get('pos', 'اسم'),
                        "source": entry.get('source', 'N/A'),
                        "math_region": entry.get('math_region', 'GENERAL'),
                        "domain": entry.get('domain', 'عام')
                    }
            print(f"✅ تم تحميل {len(self.lexicon)} مدخل من القاموس.")
        else:
            print("⚠️ لم يتم العثور على ملف القاموس.")
    def _init_db(self):
        """تهيئة قاعدة البيانات"""
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
        conn.commit()
        conn.close()
    def extract_root(self, word: str) -> Optional[str]:
        """استخراج الجذر الصرفي"""
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
        """استنتاج المجال ديناميكياً"""
        # 1. التحقق من المجالات الأساسية
        for dom, keywords in self.core_domains.items():
            if any(kw in word for kw in keywords):
                return dom
        # 2. تحليل نص التعريف (البحث عن النصوص بين الأقواس)
        if meaning_text:
            matches = re.findall(r'\((.*?)\)', meaning_text)
            for match in matches:
                match = match.strip()
                if any(kw in match for kw in ['فقه', 'شريعة', 'دين', 'إسلام']): return 'شرعي'
                if any(kw in match for kw in ['كهرباء', 'كهربائي']): return 'كهرباء'
                if any(kw in match for kw in ['إلكترونيات', 'إلكترونيك']): return 'إلكترونيك'
                if any(kw in match for kw in ['فضاء', 'فلك']): return 'فضاء'
                if any(kw in match for kw in ['طب', 'مرض']): return 'طبي'
                if any(kw in match for kw in ['قانون', 'محكمة']): return 'قانوني'
                # اكتشاف ديناميكي حقيقي للمجالات الجديدة
                if len(match) > 2 and len(match) < 40 and match not in self.core_domains:
                    self.dynamic_domains.add(match)
                    return match
        return 'عام'
    def determine_region(self, root: Optional[str], domain: str) -> str:
        """تحديد المنطقة الرياضية/المعرفية"""
        if root and root in self.math_region_rules:
            return self.math_region_rules[root]
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
        """البحث عن المصطلح في الإنترنت"""
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
                        'meaning': meaning_div.get_text(strip=True)[:300],
                        'root': root_elem.get_text(strip=True) if root_elem else None,
                        'source': 'almaany'
                    }
        except Exception:
            pass
        return None
    def auto_lookup_and_add(self, word: str) -> Optional[Dict]:
        """البحث التلقائي وإضافة المصطلح"""
        # فحص عدم التكرار
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute('SELECT original_word FROM grammar_math_rules WHERE original_word = ?', (word,))
        if cursor.fetchone():
            conn.close()
            return None
        dictionary = []
        if os.path.exists(self.dict_path):
            with open(self.dict_path, 'r', encoding='utf-8') as f:
                dictionary = json.load(f)
            if any(e['word'] == word for e in dictionary):
                conn.close()
                return None
        print(f"🤖 بحث تلقائي عن: '{word}'")
        result = self.search_online(word)
        root = result['root'] if result and result.get('root') else self.extract_root(word)
        meaning = result['meaning'] if result else ''
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
        # حفظ في JSON
        dictionary.append(entry)
        with open(self.dict_path, 'w', encoding='utf-8') as f:
            json.dump(dictionary, f, ensure_ascii=False, indent=2)
        # حفظ في DB
        cursor.execute('''
            INSERT INTO grammar_math_rules 
            (original_word, root, morph_pattern, pos, source, math_region, domain, compression_token, certainty_score, auto_generated)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ''', (word, entry['root'], entry['pattern'], entry['pos'], entry['source'], 
              entry['math_region'], entry['domain'], entry['math_region'], 0.95, 1))
        conn.commit()
        conn.close()
        # إعادة تحميل القاموس
        self._load_morphology_dictionary()
        domain_note = " (مجال مكتشف ديناميكياً!)" if domain in self.dynamic_domains else ""
        print(f"✅ تمت الإضافة: {word} -> {region} [{domain}]{domain_note}")
        return entry
    def analyze_and_compress(self, text: str) -> Dict:
        """تحليل النص وضغطه"""
        clean_text = araby.strip_tashkeel(text)
        clean_text = araby.normalize_hamza(clean_text)
        clean_text = araby.normalize_alef(clean_text)
        words = re.findall(r'\b\w+\b', clean_text)
        compressed_tokens = []
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        for word in words:
            # 1. البحث في القاموس المحلي
            if word in self.lexicon:
                info = self.lexicon[word]
                token = info['math_region']
                cursor.execute('''
                    INSERT OR IGNORE INTO grammar_math_rules 
                    (original_word, root, morph_pattern, pos, source, math_region, domain, compression_token, certainty_score, auto_generated)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ''', (word, info['root'], info['pattern'], info['pos'], info['source'], 
                      info['math_region'], info.get('domain', 'عام'), token, 1.0, 0))
                compressed_tokens.append(token)
            # 2. البحث التلقائي في الإنترنت
            elif self.auto_lookup_enabled and not word.isdigit():
                result = self.auto_lookup_and_add(word)
                if result:
                    if word in self.lexicon:
                        info = self.lexicon[word]
                        token = info['math_region']
                        cursor.execute('''
                            INSERT OR IGNORE INTO grammar_math_rules 
                            (original_word, root, morph_pattern, pos, source, math_region, domain, compression_token, certainty_score, auto_generated)
                            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                        ''', (word, info['root'], info['pattern'], info['pos'], info['source'], 
                              info['math_region'], info.get('domain', 'عام'), token, 0.95, 1))
                        compressed_tokens.append(token)
        conn.commit()
        conn.close()
        # إزالة التكرار
        seen = set()
        unique_tokens = [t for t in compressed_tokens if not (t in seen or seen.add(t))]
        return {
            "original": text,
            "clean": clean_text,
            "compressed_sequence": " -> ".join(unique_tokens),
            "tokens": unique_tokens,
            "certainty": 1.0,
            "auto_lookup_used": self.auto_lookup_enabled,
            "discovered_domains": list(self.dynamic_domains)
        }
    def export_db_to_json(self, output_path: str):
        """تصدير قاعدة البيانات كـ JSON"""
        conn = sqlite3.connect(self.db_path)
        conn.row_factory = sqlite3.Row
        cursor = conn.cursor()
        cursor.execute("SELECT * FROM grammar_math_rules ORDER BY id")
        rows = [dict(row) for row in cursor.fetchall()]
        conn.close()
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(rows, f, ensure_ascii=False, indent=2)
        print(f"✅ تم تصدير {len(rows)} قاعدة إلى {output_path}")
if __name__ == "__main__":
    engine = DeterministicGrammarEngine(auto_lookup_enabled=True)
    test_texts = [
        "احسب قيمة مجموع المصفوفة",
        "هذا اجتهاد فقهي في مسألة شرعية",
        "يدور القمر في مدار حول الأرض في الفضاء",
        "يمر التيار الكهربائي عبر المكثف والترانزستور في الدائرة"
    ]
    for text in test_texts:
        print(f"\n📝 النص: {text}")
        result = engine.analyze_and_compress(text)
        print(f"✅ التسلسل: {result['compressed_sequence']}")
        print(f"✅ المجالات المكتشفة: {result['discovered_domains']}")
    engine.export_db_to_json("grammar_math_rules_export.json")
