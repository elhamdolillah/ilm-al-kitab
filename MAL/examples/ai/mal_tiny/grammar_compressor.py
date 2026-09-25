#!/usr/bin/env python3
"""
ضاغط البيانات النحوي-الصرفي (Grammar Compressor)
يحول النصوص العربية إلى تمثيلات دلالية-رياضية مضغوطة لتسريع وتحسين تدريب الشبكات العصبية.
"""
import re
import pyarabic.araby as araby
import json
class GrammarCompressor:
    def __init__(self):
        # قاموس الضغط الدلالي-النحوي (Semantic-Grammatical Compression Dictionary)
        self.action_map = {
            r'(أنشئ|بناء|تكوين|صمم)': 'ACT:CREATE',
            r'(أضف|اجمع)': 'ACT:ADD',
            r'(اضرب|ضرب|مصفوفة)': 'ACT:MUL',
            r'(نشّط|تفعيل|تنشيط)': 'ACT:ACTIVATE',
            r'(درب|تدريب|تعلم)': 'ACT:TRAIN',
            r'(احسب|قيم|استخرج)': 'ACT:EVAL'
        }
        self.entity_map = {
            r'(شبكة عصبية|نموذج)': 'ENT:MODEL',
            r'(طبقة مخفية)': 'ENT:HIDDEN',
            r'(طبقة إدخال)': 'ENT:INPUT',
            r'(طبقة مخرجات|مخرج)': 'ENT:OUTPUT',
            r'(دالة تنشيط|تنشيط)': 'ENT:ACT_FUNC',
            r'(وزن|أوزان)': 'ENT:WEIGHTS',
            r'(تحيز|انحياز)': 'ENT:BIAS'
        }
        self.func_map = {
            r'(ReLU|ريلو)': 'FUNC:RELU',
            r'(Sigmoid|سيجمويد)': 'FUNC:SIGMOID',
            r'(Tanh|تان)': 'FUNC:TANH',
            r'(Softmax|سوفت ماكس)': 'FUNC:SOFTMAX'
        }
    def compress(self, text: str) -> dict:
        """
        ضغط النص العربي إلى تمثيل دلالي-رياضي مكثف.
        """
        # 1. تنظيف صرفي (إزالة التشكيل، توحيد الهمزات والألف)
        clean_text = araby.strip_tashkeel(text)
        clean_text = araby.normalize_hamza(clean_text)
        clean_text = araby.normalize_alef(clean_text)
        compressed_tokens = []
        metadata = {"original_length": len(text), "clean_length": len(clean_text)}
        # 2. استخراج الأبعاد والأرقام (ضغط كمي)
        dimensions = re.findall(r'بحجم\s+(\d+)|حجمه\s+(\d+)|(\d+)\s+عصبون|(\d+)\s+طبقة', clean_text)
        for d in dimensions:
            val = next((x for x in d if x), None)
            if val:
                compressed_tokens.append(f"DIM:{val}")
        # 3. استخراج الأفعال (العمليات)
        for pattern, token in self.action_map.items():
            if re.search(pattern, clean_text):
                compressed_tokens.append(token)
        # 4. استخراج الكيانات (المعاملات)
        for pattern, token in self.entity_map.items():
            if re.search(pattern, clean_text):
                compressed_tokens.append(token)
        # 5. استخراج الدوال المحددة
        for pattern, token in self.func_map.items():
            if re.search(pattern, clean_text, re.IGNORECASE):
                compressed_tokens.append(token)
        # إزالة التكرار مع الحفاظ على الترتيب النسبي التقريبي
        seen = set()
        unique_tokens = []
        for token in compressed_tokens:
            if token not in seen:
                seen.add(token)
                unique_tokens.append(token)
        metadata["compressed_length"] = len(unique_tokens)
        metadata["compression_ratio"] = round(len(clean_text) / max(len(unique_tokens), 1), 2)
        return {
            "original": text,
            "clean": clean_text,
            "compressed_tokens": unique_tokens,
            "metadata": metadata
        }
    def compress_dataset(self, dataset_path: str, output_path: str):
        """ضغط مجموعة بيانات كاملة وحفظها بصيغة JSONL للتدريب الفعال."""
        compressed_data = []
        with open(dataset_path, 'r', encoding='utf-8') as f:
            for line in f:
                if not line.strip(): continue
                try:
                    item = json.loads(line)
                    text = item.get('text', item.get('instruction', ''))
                    if text:
                        compressed = self.compress(text)
                        # إضافة المخرجات المستهدفة (labels) إذا كانت موجودة
                        if 'output' in item or 'response' in item:
                            compressed['target'] = item.get('output', item.get('response'))
                        compressed_data.append(compressed)
                except json.JSONDecodeError:
                    continue
        with open(output_path, 'w', encoding='utf-8') as f:
            for item in compressed_data:
                f.write(json.dumps(item, ensure_ascii=False) + '\n')
        print(f"✅ تم ضغط المجموعة بنجاح: {len(compressed_data)} عينة.")
        print(f"📊 متوسط نسبة الضغط: {sum(d['metadata']['compression_ratio'] for d in compressed_data)/len(compressed_data):.2f}x")
# اختبار الوحدة
if __name__ == "__main__":
    compressor = GrammarCompressor()
    test_cases = [
        "أنشئ شبكة عصبية تتكون من طبقة مخفية بحجم 8 مع دالة تنشيط ReLU",
        "اضرب مصفوفة الإدخال في الأوزان ثم نشّط النتيجة بدالة Sigmoid",
        "درب النموذج على 1000 عصبون باستخدام خوارزمية الانتشار الخلفي"
    ]
    print("🧪 اختبار ضاغط البيانات النحوي:\n" + "="*50)
    for text in test_cases:
        result = compressor.compress(text)
        print(f"الأصلي: {result['original']}")
        print(f"المضغوط: {' | '.join(result['compressed_tokens'])}")
        print(f"نسبة الضغط: {result['metadata']['compression_ratio']}x\n")
