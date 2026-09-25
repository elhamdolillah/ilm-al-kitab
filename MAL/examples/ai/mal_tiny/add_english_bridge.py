#!/usr/bin/env python3
"""
إضافة قاموس عربي-إنجليزي كجسر داخل قاعدة البيانات
يربط كل كلمة عربية بترجمتها الإنجليزية والمنطقة الرياضية
"""
import json
import os
# قاموس الترجمات الإنجليزية الشامل
english_translations = {
    # عمليات حسابية
    'احسب': 'calculate', 'حساب': 'calculation', 'حاسب': 'computer',
    'محسوب': 'computed', 'حاسبة': 'calculator', 'حسابات': 'calculations',
    'اجمع': 'add', 'جمع': 'sum', 'جامع': 'union', 'مجموع': 'total',
    'مجاميع': 'sums', 'طرح': 'subtraction', 'اطرح': 'subtract',
    'اضرب': 'multiply', 'ضرب': 'multiplication', 'ضارب': 'multiplier',
    'مضروب': 'multiplicand', 'اقسم': 'divide', 'قسمة': 'division',
    'قاسم': 'divisor', 'مقسوم': 'dividend', 'أقسام': 'sections',
    # كائنات رياضية
    'مصفوفة': 'matrix', 'مصفوفات': 'matrices', 'صف': 'row', 'صفوف': 'rows',
    'متجه': 'vector', 'متجهات': 'vectors', 'وجه': 'face', 'أوجه': 'faces',
    'محدد': 'determinant', 'معكوس': 'inverse', 'منقول': 'transpose',
    'رتبة': 'rank', 'رتب': 'ranks', 'ترتيب': 'order',
    # دوال وتحويلات
    'دالة': 'function', 'دوال': 'functions', 'تحويل': 'transform',
    'تحويلات': 'transforms', 'حوّل': 'transform', 'محول': 'converter',
    'تكامل': 'integral', 'كامل': 'complete', 'اشتقاق': 'derivation',
    'اشتق': 'derive', 'مشتق': 'derivative',
    # شبكات عصبية
    'شبكة': 'network', 'شبكات': 'networks', 'شبكة عصبية': 'neural network',
    'عصبية': 'neural', 'عصب': 'nerve', 'طبقة': 'layer', 'طبقات': 'layers',
    'طبقة مخفية': 'hidden layer', 'تنشيط': 'activation', 'نشّط': 'activate',
    'نشط': 'active', 'نشاط': 'activity', 'وزن': 'weight', 'أوزان': 'weights',
    'انحياز': 'bias',
    # إحصاء واحتمالات
    'قيمة': 'value', 'قيم': 'values', 'تقييم': 'evaluation',
    'فرق': 'difference', 'فروق': 'differences', 'افرق': 'differentiate',
    'حاصل': 'result', 'حصول': 'obtaining', 'نسبة': 'ratio', 'نسب': 'ratios',
    'نسبي': 'relative', 'مئوية': 'percentage', 'احتمال': 'probability',
    'توزيع': 'distribution', 'متوسط': 'mean', 'تباين': 'variance',
    'عينة': 'sample',
    # تحسين وتدريب
    'تدريب': 'training', 'درب': 'train', 'مدرب': 'trainer',
    'تدريبات': 'trainings', 'تحسين': 'optimization', 'حسن': 'improve',
    'محسن': 'optimizer', 'خطأ': 'error', 'أخطاء': 'errors',
    'دالة خطأ': 'loss function', 'تعلم': 'learning',
    # جبر خطي
    'فضاء': 'space', 'فضاءات': 'spaces', 'فضاء متجهي': 'vector space',
    'أساس': 'basis', 'أسس': 'bases', 'أساسي': 'fundamental',
    'بُعد': 'dimension', 'أبعاد': 'dimensions', 'بعدي': 'dimensional',
    'نواة': 'kernel', 'أنوية': 'kernels', 'نوي': 'nuclear',
    # هندسة
    'نقطة': 'point', 'خط': 'line', 'مستوى': 'plane', 'زاوية': 'angle',
    'دائرة': 'circle', 'كرة': 'sphere', 'مساحة': 'area', 'حجم': 'volume',
    'سطح': 'surface',
    # أعداد
    'عدد': 'number', 'صحيح': 'integer', 'كسري': 'fractional',
    'كسر': 'fraction', 'جذر': 'root', 'ثابت': 'constant',
    'متغير': 'variable', 'معامل': 'coefficient',
    # فيزياء
    'طاقة': 'energy', 'قوة': 'force', 'كتلة': 'mass', 'سرعة': 'velocity',
    'تردد': 'frequency',
    # كيمياء
    'ذرة': 'atom', 'جزيء': 'molecule', 'تكافؤ': 'equivalence',
    # حاسوب
    'خوارزمية': 'algorithm', 'بيانات': 'data', 'نموذج': 'model',
    # تحليل رياضي
    'نهاية': 'limit', 'متسلسلة': 'series', 'تقارب': 'convergence',
    # دوال خاصة
    'لوغاريتم': 'logarithm', 'أس': 'exponent',
    # مصطلحات تقنية
    'ReLU': 'ReLU', 'Sigmoid': 'Sigmoid', 'Tanh': 'Tanh', 'Softmax': 'Softmax',
    'PCA': 'PCA', 'SVD': 'SVD', 'CNN': 'CNN', 'RNN': 'RNN', 'GAN': 'GAN',
    'LSTM': 'LSTM', 'BERT': 'BERT', 'GPT': 'GPT', 'Adam': 'Adam',
    'SGD': 'SGD', 'MSE': 'MSE', 'MAE': 'MAE', 'NLP': 'NLP',
    'API': 'API', 'GPU': 'GPU', 'CPU': 'CPU', 'RAM': 'RAM',
    'SSD': 'SSD', 'HTTP': 'HTTP', 'JSON': 'JSON', 'XML': 'XML',
    'SQL': 'SQL', 'NoSQL': 'NoSQL', 'REST': 'REST', 'GraphQL': 'GraphQL',
    'Docker': 'Docker', 'Kubernetes': 'Kubernetes', 'Linux': 'Linux',
    'Windows': 'Windows', 'macOS': 'macOS', 'Python': 'Python',
    'Rust': 'Rust', 'C++': 'C++', 'Java': 'Java', 'JavaScript': 'JavaScript',
    'TypeScript': 'TypeScript', 'Go': 'Go', 'Swift': 'Swift',
    'Kotlin': 'Kotlin', 'Ruby': 'Ruby', 'PHP': 'PHP', 'Scala': 'Scala',
    'Haskell': 'Haskell', 'Elixir': 'Elixir', 'Erlang': 'Erlang',
    'Julia': 'Julia', 'MATLAB': 'MATLAB', 'R': 'R',
}
def add_english_translations():
    """إضافة الترجمات الإنجليزية للقاموس الموجود"""
    dict_path = "/root/ilm-al-kitab/MAL/examples/ai/mal_tiny/arabic_morphology_dict.json"
    if not os.path.exists(dict_path):
        print("❌ ملف القاموس غير موجود")
        return
    with open(dict_path, 'r', encoding='utf-8') as f:
        dictionary = json.load(f)
    print(f"📚 تحميل {len(dictionary)} مدخل من القاموس")
    # إضافة الترجمة الإنجليزية لكل مدخل
    updated_count = 0
    for entry in dictionary:
        word = entry.get('word', '')
        if word in english_translations:
            entry['en'] = english_translations[word]
            updated_count += 1
        else:
            # محاولة ترجمة تلقائية بناءً على المنطقة الرياضية
            region = entry.get('math_region', '')
            entry['en'] = get_auto_translation(word, region)
            updated_count += 1
    print(f"✅ تمت إضافة {updated_count} ترجمة إنجليزية")
    # حفظ القاموس المحدث
    with open(dict_path, 'w', encoding='utf-8') as f:
        json.dump(dictionary, f, ensure_ascii=False, indent=2)
    print(f"✅ تم حفظ القاموس المحدث في {dict_path}")
    # إنشاء قاموس عكسي (إنجليزي -> عربي)
    reverse_dict = {}
    for entry in dictionary:
        en_term = entry.get('en', '')
        ar_word = entry.get('word', '')
        if en_term and ar_word:
            if en_term not in reverse_dict:
                reverse_dict[en_term] = []
            reverse_dict[en_term].append({
                'ar': ar_word,
                'root': entry.get('root', 'N/A'),
                'math_region': entry.get('math_region', 'GENERAL')
            })
    # حفظ القاموس العكسي
    reverse_path = "/root/ilm-al-kitab/MAL/examples/ai/mal_tiny/english_to_arabic_bridge.json"
    with open(reverse_path, 'w', encoding='utf-8') as f:
        json.dump(reverse_dict, f, ensure_ascii=False, indent=2)
    print(f"✅ تم إنشاء القاموس العكسي (إنجليزي -> عربي) في {reverse_path}")
    print(f"📊 يحتوي على {len(reverse_dict)} مصطلح إنجليزي")
    # إحصائيات
    regions = {}
    for entry in dictionary:
        region = entry.get('math_region', 'GENERAL')
        regions[region] = regions.get(region, 0) + 1
    print("\n📊 توزيع المناطق الرياضية:")
    for region, count in sorted(regions.items(), key=lambda x: x[1], reverse=True)[:10]:
        print(f"  {region}: {count} كلمة")
    # عينة من القاموس العكسي
    print("\n📋 عينة من القاموس العكسي (إنجليزي -> عربي):")
    sample_terms = ['matrix', 'vector', 'neural network', 'activation', 'gradient', 'loss']
    for term in sample_terms:
        if term in reverse_dict:
            arabic_words = [item['ar'] for item in reverse_dict[term][:3]]
            print(f"  {term} -> {', '.join(arabic_words)}")
def get_auto_translation(word: str, region: str) -> str:
    """ترجمة تلقائية بناءً على المنطقة الرياضية"""
    region_translations = {
        'EVALUATE': 'evaluate',
        'SUM_OPERATION': 'sum',
        'SUB_OPERATION': 'subtract',
        'MATMUL': 'matrix multiply',
        'DIV_OPERATION': 'divide',
        'MATRIX_TENSOR': 'matrix',
        'VECTOR_TENSOR': 'vector',
        'DETERMINANT': 'determinant',
        'INVERSE': 'inverse',
        'TRANSPOSE': 'transpose',
        'FUNCTION': 'function',
        'TRANSFORM': 'transform',
        'INTEGRAL': 'integral',
        'DERIVATIVE': 'derivative',
        'NEURAL_NETWORK': 'neural network',
        'NEURAL_WEIGHTS': 'neural weights',
        'LAYER_DIMENSION': 'layer',
        'ACTIVATION_FUNC': 'activation',
        'WEIGHT_MATRIX': 'weight',
        'BIAS_VECTOR': 'bias',
        'SCALAR_VALUE': 'scalar',
        'RATIO': 'ratio',
        'PERCENTAGE': 'percentage',
        'PROBABILITY': 'probability',
        'DISTRIBUTION': 'distribution',
        'MEAN': 'mean',
        'VARIANCE': 'variance',
        'SAMPLE': 'sample',
        'OPTIMIZATION_STEP': 'optimization step',
        'OPTIMIZATION': 'optimization',
        'LOSS_FUNCTION': 'loss function',
        'LEARNING': 'learning',
        'VECTOR_SPACE': 'vector space',
        'BASIS': 'basis',
        'DIMENSION': 'dimension',
        'RANK': 'rank',
        'KERNEL': 'kernel',
        'POINT': 'point',
        'LINE': 'line',
        'PLANE': 'plane',
        'ANGLE': 'angle',
        'CIRCLE': 'circle',
        'SPHERE': 'sphere',
        'AREA': 'area',
        'VOLUME': 'volume',
        'SURFACE': 'surface',
        'NUMBER': 'number',
        'INTEGER': 'integer',
        'FRACTION': 'fraction',
        'ROOT': 'root',
        'CONSTANT': 'constant',
        'VARIABLE': 'variable',
        'COEFFICIENT': 'coefficient',
        'ENERGY': 'energy',
        'FORCE': 'force',
        'MASS': 'mass',
        'VELOCITY': 'velocity',
        'FREQUENCY': 'frequency',
        'ATOM': 'atom',
        'MOLECULE': 'molecule',
        'EQUIVALENCE': 'equivalence',
        'ALGORITHM': 'algorithm',
        'DATA': 'data',
        'MODEL': 'model',
        'LIMIT': 'limit',
        'SERIES': 'series',
        'CONVERGENCE': 'convergence',
        'LOGARITHM': 'logarithm',
        'EXPONENT': 'exponent',
    }
    return region_translations.get(region, word)
if __name__ == "__main__":
    add_english_translations()
