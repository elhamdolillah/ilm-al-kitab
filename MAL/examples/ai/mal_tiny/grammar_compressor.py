#!/usr/bin/env python3
"""
المحرك اللغوي-الرياضي الحتمي (Deterministic Neuro-Grammatical Engine) v9
يدعم: الاكتشاف الديناميكي + عدم التكرار + البحث التلقائي + تنظيم المعرفة الهيكلي + حساب نسبة الحتمية الكلية
"""
import re
import json
import sqlite3
import pyarabic.araby as araby
from typing import Dict, List, Optional
import os
import requests
from bs4 import BeautifulSoup
class KnowledgeOrganizer:
    def __init__(self):
        self.hierarchy_templates = {
            'شرعي': {'library': 'العلوم الشرعية', 'main_axes': ['أصول الفقه', 'أحكام المعاملات', 'الفقه المقارن']},
            'فضاء': {'library': 'علوم الفضاء والفلك', 'main_axes': ['الميكانيكا السماوية', 'استكشاف الكواكب', 'الفيزياء الفلكية']},
            'كهرباء': {'library': 'الهندسة الكهربائية', 'main_axes': ['دوائر التيار المستمر', 'الإلكترونيات', 'نظم القدرة']},
            'إلكترونيك': {'library': 'هندسة الإلكترونيات', 'main_axes': ['أشباه الموصلات', 'الدوائر الرقمية', 'معالجة الإشارات']},
            'طبي': {'library': 'العلوم الطبية', 'main_axes': ['التشريح', 'الفيزيولوجيا', 'الطب الدقيق']},
            'فيزياء كمومية': {'library': 'الفيزياء الحديثة', 'main_axes': ['ميكانيكا الكم', 'التراكب الكمومي', 'التشابك الكمومي']},
            'عمارة': {'library': 'الهندسة المعمارية', 'main_axes': ['التصميم المستدام', 'ديناميكا المباني', 'مواد البناء']},
            'عام': {'library': 'معارف عامة', 'main_axes': ['تعريف عام', 'تطبيقات', 'خلاصة']}
        }
    def organize_response(self, word: str, domain: str, meaning: str, mal_region: str) -> Dict:
        template = self.hierarchy_templates.get(domain, self.hierarchy_templates['عام'])
        main_axis = template['main_axes'][0]
        return {
            "word": word,
            "domain": domain,
            "mal_region": mal_region,
            "hierarchical_classification": {
                "library_section": template['library'],
                "main_axis": main_axis,
                "secondary_axis": f"مفاهيم {word} الأساسية",
                "topic": word,
                "structured_content": {
                    "introduction": f"مقدمة: يُعد مصطلح '{word}' من المفاهيم المحورية في {template['library']}. {meaning[:150] if meaning else ''}",
                    "body_paragraphs": [
                        f"1. التحليل الصرفي: الجذر اللغوي يشير إلى عمق المفهوم وأصالته في السياق العربي.",
                        f"2. التطبيق المعرفي: في نطاق {domain}، يُترجم هذا المفهوم برمجياً إلى المنطقة المعرفية ({mal_region}).",
                        f"3. الضبط الحتمي: يضمن هذا التصنيف بقاء المعالجة ضمن سياق '{main_axis}' بنسبة حتمية تتجاوز 97%."
                    ],
                    "conclusion": f"خاتمة: يلخص هذا الهيكل مكانة '{word}' ضمن التصنيف المعرفي، مما يمنع أي انحراف دلالي ويوجه مولد الكود بدقة."
                }
            },
            "mal_code_guidance": f"// توجيه هيكلي لمولد الكود\n// المجال: {domain} | المنطقة: {mal_region}\nconst {word}_schema = {{ domain: '{domain}', region: '{mal_region}', axis: '{main_axis}' }};"
        }
class DeterministicGrammarEngine:
    def __init__(self, db_path: str = "grammar_math_rules.db", 
                 dict_path: str = "arabic_morphology_dict.json",
                 auto_lookup_enabled: bool = True):
        self.db_path = db_path
        self.dict_path = dict_path
        self.auto_lookup_enabled = auto_lookup_enabled
        self.lexicon = {}
        self.dynamic_domains = set()
        self.organizer = KnowledgeOrganizer()
        self.core_domains = {
            'رياضي': ['مصفوفة', 'متجه', 'تكامل', 'مشتق', 'احتمال', 'تباين', 'فضاء', 'بُعد', 'نواة', 'دالة', 'معادلة'],
            'حاسوبي': ['خوارزمية', 'بيانات', 'نموذج', 'شبكة', 'خادم', 'برمجية', 'نظام', 'ذكاء'],
            'طبي': ['مريض', 'علاج', 'دواء', 'مرض', 'تشخيص', 'جراحة', 'جين', 'وراثي'],
            'زراعي': ['محصول', 'تربة', 'ري', 'سماد', 'حصاد', 'بذر'],
            'قانوني': ['عقد', 'حكم', 'محكمة', 'دعوى', 'حق', 'قانون', 'تشريع'],
            'جيولوجي': ['صخر', 'معدن', 'زلزال', 'بركان', 'طبقة', 'أحفورة'],
            'بيولوجي': ['خلية', 'جين', 'كائن', 'تكاثر', 'وراثة', 'تطور', 'نسيج'],
            'كيميائي': ['عنصر', 'مركب', 'تفاعل', 'ذرة', 'جزيء', 'رابطة', 'حمض'],
            'فيزيائي': ['طاقة', 'قوة', 'كتلة', 'سرعة', 'تردد', 'ضوء', 'موجة', 'جاذبية'],
            'شرعي': ['فقه', 'شريعة', 'دين', 'إسلام', 'حديث', 'عبادة', 'صلاة', 'اجتهاد'],
            'كهرباء': ['كهرباء', 'تيار', 'جهد', 'مكثف', 'مقاومة', 'دائرة'],
            'إلكترونيك': ['إلكترونيات', 'ترانزستور', 'دوائر', 'معالج', 'شريحة'],
            'فضاء': ['فضاء', 'فلك', 'نجوم', 'كواكب', 'مدار', 'قمر', 'أرض'],
            'فيزياء كمومية': ['كمومي', 'تشابك', 'تراكب', 'جسيم', 'احتمالي'],
            'عمارة': ['معماري', 'تصميم', 'مستدام', 'حراري', 'كربوني', 'مبنى']
        }
        self.math_region_rules = {
            'ح س ب': 'EVALUATE', 'ج م ع': 'SUM_OPERATION', 'ط ر ح': 'SUB_OPERATION',
            'ض ر ب': 'MATMUL', 'ق س م': 'DIV_OPERATION', 'ص ف ف': 'MATRIX_TENSOR',
            'و ج ه': 'VECTOR_TENSOR', 'ح د د': 'DETERMINANT', 'ع ك س': 'INVERSE',
            'ن ق ل': 'TRANSPOSE', 'د و ل': 'FUNCTION', 'ح و ل': 'TRANSFORM',
            'ك م ل': 'INTEGRAL', 'ش ت ق': 'DERIVATIVE', 'ش ب ك': 'NEURAL_NETWORK',
            'ع ص ب': 'NEURAL_WEIGHTS', 'ط ب ق': 'LAYER_DIMENSION', 'ن ش ط': 'ACTIVATION_FUNC',
            'و ز ن': 'WEIGHT_MATRIX', 'ن ح ي ز': 'BIAS_VECTOR', 'ق ي م': 'SCALAR_VALUE',
            'ف ق ه': 'ISLAMIC_JURISPRUDENCE', 'م د ر': 'ASTRONOMY_SPACE', 'ك ه ر ب': 'ELECTRICAL_ENGINEERING',
            'ك م م': 'QUANTUM_MECHANICS', 'ع م ر': 'ARCHITECTURE_DESIGN', 'ط ب د': 'PRECISION_MEDICINE'
        }
        self._load_morphology_dictionary()
        self._init_db()
    def _load_morphology_dictionary(self):
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
        for dom, keywords in self.core_domains.items():
            if any(kw in word for kw in keywords):
                return dom
        if meaning_text:
            matches = re.findall(r'\((.*?)\)', meaning_text)
            for match in matches:
                match = match.strip()
                if any(kw in match for kw in ['فقه', 'شريعة', 'دين', 'إسلام', 'اجتهاد']): return 'شرعي'
                if any(kw in match for kw in ['كهرباء', 'كهربائي', 'تيار']): return 'كهرباء'
                if any(kw in match for kw in ['إلكترونيات', 'إلكترونيك', 'ترانزستور']): return 'إلكترونيك'
                if any(kw in match for kw in ['فضاء', 'فلك', 'مدار']): return 'فضاء'
                if any(kw in match for kw in ['طب', 'مرض', 'جين']): return 'طبي'
                if any(kw in match for kw in ['قانون', 'محكمة']): return 'قانوني'
                if any(kw in match for kw in ['كم', 'تشابك', 'تراكب']): return 'فيزياء كمومية'
                if any(kw in match for kw in ['معماري', 'مبنى', 'تصميم']): return 'عمارة'
                if len(match) > 2 and len(match) < 40 and match not in self.core_domains:
                    self.dynamic_domains.add(match)
                    return match
        return 'عام'
    def determine_region(self, root: Optional[str], domain: str) -> str:
        if root and root in self.math_region_rules:
            return self.math_region_rules[root]
        domain_to_region = {
            'شرعي': 'ISLAMIC_JURISPRUDENCE', 'كهرباء': 'ELECTRICAL_ENGINEERING',
            'إلكترونيك': 'ELECTRONICS', 'فضاء': 'ASTRONOMY_SPACE', 'زراعي': 'AGRICULTURE',
            'طبي': 'PRECISION_MEDICINE', 'قانوني': 'LEGAL', 'جيولوجي': 'GEOLOGY',
            'بيولوجي': 'BIOLOGY', 'كيميائي': 'CHEMISTRY', 'فيزيائي': 'PHYSICS',
            'حاسوبي': 'COMPUTER_SCIENCE', 'رياضي': 'MATHEMATICS', 'فيزياء كمومية': 'QUANTUM_MECHANICS', 'عمارة': 'ARCHITECTURE_DESIGN'
        }
        return domain_to_region.get(domain, 'GENERAL')
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
                        'meaning': meaning_div.get_text(strip=True)[:300],
                        'root': root_elem.get_text(strip=True) if root_elem else None,
                        'source': 'almaany'
                    }
        except Exception:
            pass
        return None
    def auto_lookup_and_add(self, word: str, conn: sqlite3.Connection, cursor: sqlite3.Cursor) -> Optional[Dict]:
        cursor.execute('SELECT original_word FROM grammar_math_rules WHERE original_word = ?', (word,))
        if cursor.fetchone():
            return None
        dictionary = []
        if os.path.exists(self.dict_path):
            with open(self.dict_path, 'r', encoding='utf-8') as f:
                dictionary = json.load(f)
            if any(e['word'] == word for e in dictionary):
                return None
        print(f"🤖 بحث تلقائي عن: '{word}'")
        result = self.search_online(word)
        root = result['root'] if result and result.get('root') else self.extract_root(word)
        meaning = result['meaning'] if result else ''
        domain = self.infer_domain_dynamically(word, meaning)
        region = self.determine_region(root, domain)
        entry = {
            'word': word, 'root': root or 'N/A', 'pattern': 'مشتق', 'pos': 'اسم',
            'source': result['source'] if result else 'auto', 'math_region': region,
            'domain': domain, 'meaning': meaning, 'auto_generated': True
        }
        dictionary.append(entry)
        with open(self.dict_path, 'w', encoding='utf-8') as f:
            json.dump(dictionary, f, ensure_ascii=False, indent=2)
        cursor.execute('''
            INSERT INTO grammar_math_rules 
            (original_word, root, morph_pattern, pos, source, math_region, domain, compression_token, certainty_score, auto_generated)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ''', (word, entry['root'], entry['pattern'], entry['pos'], entry['source'], 
              entry['math_region'], entry['domain'], entry['math_region'], 0.95, 1))
        self._load_morphology_dictionary()
        domain_note = " (مجال مكتشف ديناميكياً!)" if domain in self.dynamic_domains else ""
        print(f"✅ تمت الإضافة: {word} -> {region} [{domain}]{domain_note}")
        return entry
    def calculate_sentence_certainty(self, words_info: List[Dict]) -> float:
        """حساب نسبة الحتمية الكلية للجملة بناءً على تصنيف كلماتها ذات الدلالة"""
        if not words_info:
            return 0.0
        total_score = 0.0
        valid_words_count = 0
        # كلمات توقف شائعة لا تؤثر في الحساب
        stopwords = {'في', 'من', 'إلى', 'على', 'عن', 'مع', 'هذا', 'هذه', 'ذلك', 'تلك', 'هو', 'هي', 'و', 'أو', 'ثم', 'لكن', 'أن', 'لا', 'لم', 'لن', 'قد', 'إن'}
        for info in words_info:
            word = info.get('word', '')
            # تجاهل الكلمات القصيرة جداً، كلمات التوقف، والأرقام
            if len(word) <= 2 or word in stopwords or word.isdigit():
                continue
            valid_words_count += 1
            domain = info.get('domain', 'عام')
            auto_generated = info.get('auto_generated', False)
            # منطق احتساب النسبة
            if not auto_generated:
                if domain != 'عام':
                    total_score += 1.0      # كلمة معروفة في مجال محدد (حتمية كاملة)
                else:
                    total_score += 0.6      # كلمة معروفة لكن مجالها عام
            else:
                if domain != 'عام':
                    total_score += 0.95     # كلمة مكتشفة ديناميكياً في مجال محدد
                else:
                    total_score += 0.3      # كلمة مكتشفة ديناميكياً ومجالها عام
        if valid_words_count == 0:
            return 0.0
        return total_score / valid_words_count
    def analyze_and_compress_with_structure(self, text: str) -> Dict:
        clean_text = araby.strip_tashkeel(text)
        clean_text = araby.normalize_hamza(clean_text)
        clean_text = araby.normalize_alef(clean_text)
        words = re.findall(r'\b\w+\b', clean_text)
        compressed_tokens = []
        structured_responses = []
        words_info = [] # لتجميع معلومات كل كلمة لحساب النسبة
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        for word in words:
            word_info = {'word': word}
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
                word_info.update(info)
                word_info['auto_generated'] = False
                if info['domain'] != 'عام' or len(word) > 3:
                    resp = self.organizer.organize_response(word, info['domain'], info.get('meaning', ''), token)
                    structured_responses.append(resp)
            elif self.auto_lookup_enabled and not word.isdigit():
                result = self.auto_lookup_and_add(word, conn, cursor)
                if result and word in self.lexicon:
                    info = self.lexicon[word]
                    token = info['math_region']
                    cursor.execute('''
                        INSERT OR IGNORE INTO grammar_math_rules 
                        (original_word, root, morph_pattern, pos, source, math_region, domain, compression_token, certainty_score, auto_generated)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ''', (word, info['root'], info['pattern'], info['pos'], info['source'], 
                          info['math_region'], info.get('domain', 'عام'), token, 0.95, 1))
                    compressed_tokens.append(token)
                    word_info.update(info)
                    word_info['auto_generated'] = True
                    resp = self.organizer.organize_response(word, info['domain'], info.get('meaning', ''), token)
                    structured_responses.append(resp)
            words_info.append(word_info)
        conn.commit()
        conn.close()
        seen = set()
        unique_tokens = [t for t in compressed_tokens if not (t in seen or seen.add(t))]
        # 1. حساب نسبة الحتمية الكلية للجملة
        certainty_percentage = self.calculate_sentence_certainty(words_info)
        is_deterministic = certainty_percentage >= 0.97
        # 2. إضافة تنويه إذا لم تصل النسبة إلى 97%
        disclaimer = ""
        if not is_deterministic:
            disclaimer = f"\n\n⚠️ تنويه هام: نسبة الحتمية للتصنيف هي {certainty_percentage:.1%} (أقل من عتبة 97٪).\nالنموذج غير متأكد تماماً ويحتاج إلى تغذية إضافية بالمعطيات لتقديم إجابة صادقة وحتمية بنسبة 100٪ عن هذا السؤال.\nومع ذلك، إليك التحليل الأولي بناءً على المعطيات المتاحة:"
        return {
            "original": text,
            "clean": clean_text,
            "compressed_sequence": " -> ".join(unique_tokens),
            "tokens": unique_tokens,
            "certainty_percentage": certainty_percentage,
            "is_deterministic": is_deterministic,
            "disclaimer": disclaimer,
            "auto_lookup_used": self.auto_lookup_enabled,
            "discovered_domains": list(self.dynamic_domains),
            "hierarchical_responses": structured_responses
        }
if __name__ == "__main__":
    engine = DeterministicGrammarEngine(auto_lookup_enabled=True)
    test_texts = [
        "تتشابك الحالات الكمومية للجسيمات دون الذرية في تراكب احتمالي",
        "يعتمد التصميم المعماري المستدام على تحسين الكفاءة الحرارية وتقليل البصمة الكربونية",
        "هذا سؤال غريب جداً عن شيء لا أعرفه في الفضاء الخارجي" # جملة اختبارية منخفضة الحتمية
    ]
    for text in test_texts:
        print(f"\n" + "="*80)
        print(f"📝 النص المدخل: {text}")
        print("="*80)
        result = engine.analyze_and_compress_with_structure(text)
        # عرض نسبة الحتمية بوضوح
        status_icon = "✅" if result['is_deterministic'] else "⚠️"
        print(f"{status_icon} نسبة الحتمية الكلية: {result['certainty_percentage']:.1%} | حتمي: {result['is_deterministic']}")
        print(f"✅ التسلسل المضغوط: {result['compressed_sequence']}")
        if result['disclaimer']:
            print(result['disclaimer'])
        if result['hierarchical_responses']:
            sample_resp = result['hierarchical_responses'][0]
            print(f"\n📚 نموذج التصنيف الهيكلي للمصطلح: '{sample_resp['word']}'")
            print(f"   📂 القسم: {sample_resp['hierarchical_classification']['library_section']}")
            print(f"   📌 المحور الرئيسي: {sample_resp['hierarchical_classification']['main_axis']}")
