#!/usr/bin/env python3
"""
وحدة البحث التلقائي في الإنترنت للمصطلحات الجديدة
تبحث عن المصطلح في مصادر موثوقة وتضيفه لقاعدة البيانات
"""
import requests
import json
import re
from typing import Dict, Optional, List
from bs4 import BeautifulSoup
import pyarabic.araby as araby
class AutoLookup:
    def __init__(self):
        self.sources = [
            'https://www.almaany.com/ar/dict/ar-ar/',  # قاموس المعاني
            'https://ar.wiktionary.org/wiki/',  # ويكي قاموس العربي
        ]
        # خريطة المناطق الرياضية (دستور MAL)
        self.math_region_rules = {
            # عمليات حسابية
            'ح س ب': 'EVALUATE',
            'ج م ع': 'SUM_OPERATION',
            'ط ر ح': 'SUB_OPERATION',
            'ض ر ب': 'MATMUL',
            'ق س م': 'DIV_OPERATION',
            # كائنات رياضية
            'ص ف ف': 'MATRIX_TENSOR',
            'و ج ه': 'VECTOR_TENSOR',
            'ح د د': 'DETERMINANT',
            'ع ك س': 'INVERSE',
            'ن ق ل': 'TRANSPOSE',
            # دوال
            'د و ل': 'FUNCTION',
            'ح و ل': 'TRANSFORM',
            'ك م ل': 'INTEGRAL',
            'ش ت ق': 'DERIVATIVE',
            # شبكات عصبية
            'ش ب ك': 'NEURAL_NETWORK',
            'ع ص ب': 'NEURAL_WEIGHTS',
            'ط ب ق': 'LAYER_DIMENSION',
            'ن ش ط': 'ACTIVATION_FUNC',
            'و ز ن': 'WEIGHT_MATRIX',
            'ن ح ي ز': 'BIAS_VECTOR',
            # إحصاء
            'ق ي م': 'SCALAR_VALUE',
            'ن س ب': 'RATIO',
            'ا ح ت م': 'PROBABILITY',
            # تحسين
            'د ر ب': 'OPTIMIZATION_STEP',
            'ح س ن': 'OPTIMIZATION',
            'خ ط أ': 'LOSS_FUNCTION',
            # جبر خطي
            'ف ض ي': 'VECTOR_SPACE',
            'أ س س': 'BASIS',
            'ب ع د': 'DIMENSION',
            'ر ت ب': 'RANK',
            'ن و ي': 'KERNEL',
            # هندسة
            'ن ق ط': 'POINT',
            'خ ط ط': 'LINE',
            'س ط ح': 'SURFACE',
            'ز و ي': 'ANGLE',
            'د و ر': 'CIRCLE',
            # فيزياء
            'ط ا ق': 'ENERGY',
            'ق و ي': 'FORCE',
            'ك ت ل': 'MASS',
            'س ر ع': 'VELOCITY',
            'ت ر د د': 'FREQUENCY',
        }
    def extract_root(self, word: str) -> Optional[str]:
        """استخراج الجذر الثلاثي من الكلمة باستخدام قواعد صرفية"""
        # إزالة التشكيل
        word = araby.strip_tashkeel(word)
        # إزالة أل التعريف
        word = re.sub(r'^ال', '', word)
        # إزالة اللواحق الشائعة
        suffixes = ['ة', 'ات', 'ين', 'ون', 'ان', 'ها', 'هم', 'هن', 'نا', 'كم', 'كن']
        for suffix in suffixes:
            if word.endswith(suffix):
                word = word[:-len(suffix)]
                break
        # إزالة البوادئ الشائعة
        prefixes = ['م', 'ت', 'ي', 'ن', 'ا', 'س']
        for prefix in prefixes:
            if word.startswith(prefix) and len(word) > 3:
                word = word[len(prefix):]
                break
        # محاولة استخراج الجذر الثلاثي
        if len(word) >= 3:
            # افتراض أن الحروف 1، 3، 5 (أو 1، 2، 3) هي الجذر
            if len(word) == 3:
                return f"{word[0]} {word[1]} {word[2]}"
            elif len(word) == 4:
                # قد يكون الجذر هو الحروف 1، 2، 4
                return f"{word[0]} {word[1]} {word[3]}"
            elif len(word) >= 5:
                # الحروف 1، 3، 5 غالباً
                return f"{word[0]} {word[2]} {word[4]}"
        return None
    def determine_math_region(self, word: str, root: Optional[str]) -> str:
        """تحديد المنطقة الرياضية بناءً على الجذر والقواعد"""
        if root and root in self.math_region_rules:
            return self.math_region_rules[root]
        # قواعد إضافية بناءً على نمط الكلمة
        if any(kw in word for kw in ['مصفوفة', 'متجه', 'موتر']):
            return 'MATRIX_TENSOR'
        elif any(kw in word for kw in ['دالة', 'تحويل', 'تكامل', 'مشتق']):
            return 'FUNCTION'
        elif any(kw in word for kw in ['شبكة', 'عصب', 'طبقة', 'تنشيط']):
            return 'NEURAL_NETWORK'
        elif any(kw in word for kw in ['احتمال', 'توزيع', 'متوسط', 'تباين']):
            return 'STATISTICS'
        elif any(kw in word for kw in ['فضاء', 'أساس', 'بُعد', 'رتبة', 'نواة']):
            return 'LINEAR_ALGEBRA'
        elif any(kw in word for kw in ['نقطة', 'خط', 'مستوى', 'زاوية', 'دائرة']):
            return 'GEOMETRY'
        elif any(kw in word for kw in ['طاقة', 'قوة', 'كتلة', 'سرعة']):
            return 'PHYSICS'
        return 'GENERAL'
    def search_almaany(self, word: str) -> Optional[Dict]:
        """البحث في قاموس المعاني"""
        try:
            url = f"https://www.almaany.com/ar/dict/ar-ar/{word}/"
            headers = {
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
            }
            response = requests.get(url, headers=headers, timeout=10)
            if response.status_code == 200:
                soup = BeautifulSoup(response.content, 'html.parser')
                # البحث عن المعنى
                meaning_div = soup.find('div', class_='meaning-block')
                if meaning_div:
                    meaning_text = meaning_div.get_text(strip=True)
                    # استخراج الجذر إذا وجد
                    root_elem = soup.find('span', class_='root-word')
                    root = root_elem.get_text(strip=True) if root_elem else None
                    return {
                        'word': word,
                        'meaning': meaning_text[:200],  # أول 200 حرف
                        'root': root,
                        'source': 'almaany'
                    }
        except Exception as e:
            print(f"⚠️ خطأ في البحث في المعاني: {e}")
        return None
    def search_wiktionary(self, word: str) -> Optional[Dict]:
        """البحث في ويكي قاموس العربي"""
        try:
            url = f"https://ar.wiktionary.org/wiki/{word}"
            headers = {
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
            }
            response = requests.get(url, headers=headers, timeout=10)
            if response.status_code == 200:
                soup = BeautifulSoup(response.content, 'html.parser')
                # البحث عن قسم الجذر
                root_section = soup.find('span', id='الجذر')
                if root_section:
                    root_text = root_section.find_next('p')
                    if root_text:
                        root = root_text.get_text(strip=True)
                        return {
                            'word': word,
                            'root': root,
                            'source': 'wiktionary'
                        }
        except Exception as e:
            print(f"⚠️ خطأ في البحث في ويكي قاموس: {e}")
        return None
    def lookup_and_add(self, word: str, dict_path: str) -> Optional[Dict]:
        """البحث عن المصطلح وإضافته للقاموس"""
        print(f"🔍 البحث عن: {word}")
        # محاولة البحث في المصادر
        result = self.search_almaany(word)
        if not result:
            result = self.search_wiktionary(word)
        if not result:
            print(f"⚠️ لم يتم العثور على: {word}")
            return None
        # استخراج الجذر
        root = result.get('root') or self.extract_root(word)
        # تحديد المنطقة الرياضية
        math_region = self.determine_math_region(word, root)
        # إنشاء المدخل
        entry = {
            'word': word,
            'root': root or 'N/A',
            'pattern': 'مشتق تلقائياً',
            'pos': 'اسم',  # افتراضي
            'source': result.get('source', 'auto'),
            'math_region': math_region,
            'meaning': result.get('meaning', ''),
            'auto_generated': True
        }
        # إضافة للقاموس
        if os.path.exists(dict_path):
            with open(dict_path, 'r', encoding='utf-8') as f:
                dictionary = json.load(f)
        else:
            dictionary = []
        # التحقق من عدم التكرار
        existing_words = {e['word'] for e in dictionary}
        if word not in existing_words:
            dictionary.append(entry)
            with open(dict_path, 'w', encoding='utf-8') as f:
                json.dump(dictionary, f, ensure_ascii=False, indent=2)
            print(f"✅ تمت إضافة: {word} -> {math_region}")
            return entry
        else:
            print(f"⚠️ الكلمة موجودة بالفعل: {word}")
            return None
    def batch_lookup(self, words: List[str], dict_path: str) -> List[Dict]:
        """البحث عن مجموعة من المصطلحات"""
        results = []
        for word in words:
            result = self.lookup_and_add(word, dict_path)
            if result:
                results.append(result)
            import time
            time.sleep(1)  # تأخير لتجنب الحظر
        return results
import os
if __name__ == "__main__":
    lookup = AutoLookup()
    # اختبار مع مصطلحات جديدة
    test_words = ['تدرج', 'انتشار', 'خلفي', 'تحسين', 'تكراري']
    print("🤖 اختبار البحث التلقائي:")
    results = lookup.batch_lookup(test_words, 'arabic_morphology_dict.json')
    print(f"\n✅ تمت إضافة {len(results)} مصطلح جديد")
