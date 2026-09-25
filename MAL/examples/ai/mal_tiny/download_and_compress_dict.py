#!/usr/bin/env python3
"""
تحميل القاموس الصرفي العربي من مصادر مفتوحة المصدر وضغطه بذكاء
"""
import requests
import json
import os
import re
from typing import Dict, List
class ArabicDictionaryLoader:
    def __init__(self):
        self.sources = [
            # قاموس عربي شامل من GitHub
            "https://raw.githubusercontent.com/dyashar/Arabic-Dictionary/master/dictionary.json",
            # قاموس Buckwalter
            "https://raw.githubusercontent.com/KhaledAlshaer/Arabic-Dictionary/master/dictionary.json",
            # قاموس عربي-عربي
            "https://raw.githubusercontent.com/aamirbhat/arabic-dictionary/master/dict.json"
        ]
    def download_from_source(self, url: str) -> List[Dict]:
        """تحميل القاموس من مصدر واحد"""
        try:
            print(f"📥 تحميل من: {url}")
            response = requests.get(url, timeout=30)
            response.raise_for_status()
            data = response.json()
            print(f"✅ تم تحميل {len(data) if isinstance(data, list) else 'بيانات'} مدخل")
            return data if isinstance(data, list) else []
        except Exception as e:
            print(f"⚠️ فشل التحميل من {url}: {e}")
            return []
    def compress_dictionary(self, entries: List[Dict]) -> List[Dict]:
        """ضغط القاموس بذكاء"""
        compressed = []
        seen_roots = set()
        for entry in entries:
            if not isinstance(entry, dict):
                continue
            # استخراج المعلومات الأساسية
            word = entry.get('word', entry.get('term', entry.get('كلمة', '')))
            if not word:
                continue
            # ضغط الجذر (إزالة المسافات الزائدة)
            root = entry.get('root', entry.get('جذر', 'N/A'))
            if root and root != 'N/A':
                root = ' '.join(root.split())  # توحيد المسافات
            # تحديد المنطقة الرياضية
            math_region = self._determine_math_region(word, root)
            # إنشاء مدخل مضغوط
            compressed_entry = {
                "word": word,
                "root": root,
                "pattern": entry.get('pattern', entry.get('وزن', 'N/A')),
                "pos": entry.get('pos', entry.get('نوع', 'اسم')),
                "source": entry.get('source', entry.get('مصدر', root)),
                "math_region": math_region
            }
            compressed.append(compressed_entry)
        print(f"✅ تم ضغط {len(compressed)} مدخل")
        return compressed
    def _determine_math_region(self, word: str, root: str) -> str:
        """تحديد المنطقة الرياضية بناءً على الكلمة والجذر"""
        word_lower = word.lower()
        # خريطة شاملة للمناطق الرياضية
        region_map = {
            # جبر خطي
            'مصفوفة': 'MATRIX_TENSOR', 'متجه': 'VECTOR_TENSOR', 'محدد': 'DETERMINANT',
            'معكوس': 'INVERSE', 'منقول': 'TRANSPOSE', 'رتبة': 'RANK',
            # حساب التفاضل والتكامل
            'مشتق': 'DERIVATIVE', 'تكامل': 'INTEGRAL', 'نهاية': 'LIMIT',
            'متسلسلة': 'SERIES', 'تقارب': 'CONVERGENCE',
            # إحصاء واحتمالات
            'احتمال': 'PROBABILITY', 'توزيع': 'DISTRIBUTION', 'متوسط': 'MEAN',
            'تباين': 'VARIANCE', 'انحراف': 'DEVIATION', 'عينة': 'SAMPLE',
            # شبكات عصبية
            'شبكة': 'NEURAL_NETWORK', 'عصب': 'NEURAL', 'طبقة': 'LAYER',
            'تنشيط': 'ACTIVATION', 'وزن': 'WEIGHT', 'انحياز': 'BIAS',
            # تحسين
            'تدرج': 'GRADIENT', 'نزول': 'DESCENT', 'تحسين': 'OPTIMIZATION',
            'خسارة': 'LOSS', 'دقة': 'ACCURACY',
            # هندسة
            'نقطة': 'POINT', 'خط': 'LINE', 'مستوى': 'PLANE', 'زاوية': 'ANGLE',
            'دائرة': 'CIRCLE', 'كرة': 'SPHERE', 'مساحة': 'AREA', 'حجم': 'VOLUME',
            # أعداد
            'عدد': 'NUMBER', 'صحيح': 'INTEGER', 'كسري': 'FRACTION',
            'عشري': 'DECIMAL', 'نسبي': 'RATIONAL', 'جذر': 'ROOT',
            # دوال خاصة
            'دالة': 'FUNCTION', 'تحويل': 'TRANSFORM', 'لوغاريتم': 'LOGARITHM',
            'أسي': 'EXPONENTIAL', 'مثلث': 'TRIGONOMETRIC',
            # منطق
            'صحيح': 'TRUE', 'خطأ': 'FALSE', 'و': 'AND', 'أو': 'OR', 'لا': 'NOT',
            # عمليات أساسية
            'جمع': 'SUM', 'طرح': 'SUBTRACTION', 'ضرب': 'MULTIPLICATION',
            'قسمة': 'DIVISION', 'أس': 'POWER',
        }
        # البحث في الخريطة
        for key, region in region_map.items():
            if key in word:
                return region
        # افتراضي
        return 'GENERAL'
    def merge_with_existing(self, new_entries: List[Dict], existing_path: str) -> List[Dict]:
        """دمج القاموس الجديد مع الموجود"""
        if os.path.exists(existing_path):
            with open(existing_path, 'r', encoding='utf-8') as f:
                existing = json.load(f)
            # إزالة التكرارات
            existing_words = {e['word'] for e in existing}
            merged = existing.copy()
            for entry in new_entries:
                if entry['word'] not in existing_words:
                    merged.append(entry)
                    existing_words.add(entry['word'])
            print(f"✅ تم دمج {len(new_entries)} مدخل جديد (الإجمالي: {len(merged)})")
            return merged
        else:
            return new_entries
    def load_all_sources(self):
        """تحميل من جميع المصادر"""
        all_entries = []
        for source in self.sources:
            entries = self.download_from_source(source)
            if entries:
                all_entries.extend(entries)
        if not all_entries:
            print("⚠️ لم يتم تحميل أي بيانات من الإنترنت، استخدام القاموس المحلي فقط")
            return []
        # ضغط البيانات
        compressed = self.compress_dictionary(all_entries)
        return compressed
def main():
    loader = ArabicDictionaryLoader()
    # تحميل من الإنترنت
    print("\n🌐 بدء التحميل من مصادر الإنترنت...")
    new_entries = loader.load_all_sources()
    if new_entries:
        # دمج مع القاموس الحالي
        output_path = "/root/ilm-al-kitab/MAL/examples/ai/mal_tiny/arabic_morphology_dict.json"
        final_dict = loader.merge_with_existing(new_entries, output_path)
        # حفظ القاموس النهائي
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(final_dict, f, ensure_ascii=False, indent=2)
        print(f"\n✅ تم حفظ القاموس النهائي ({len(final_dict)} مدخل)")
        # عرض إحصائيات
        regions = {}
        for entry in final_dict:
            region = entry.get('math_region', 'GENERAL')
            regions[region] = regions.get(region, 0) + 1
        print("\n📊 توزيع المناطق الرياضية:")
        for region, count in sorted(regions.items(), key=lambda x: x[1], reverse=True)[:10]:
            print(f"  {region}: {count} كلمة")
    else:
        print("\n⚠️ لم يتم تحميل بيانات جديدة")
if __name__ == "__main__":
    main()
