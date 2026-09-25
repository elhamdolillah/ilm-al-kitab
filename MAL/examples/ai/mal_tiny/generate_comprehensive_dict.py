#!/usr/bin/env python3
"""
توليد قاموس صرفي عربي شامل من الجذور والأوزان
يستخدم pyarabic لتوليد الاشتقاقات تلقائياً
"""
import json
import os
import re
from typing import Dict, List, Tuple
class ComprehensiveDictionaryGenerator:
    def __init__(self):
        # الجذور الأساسية في المجال الرياضي والعلمي
        self.math_roots = {
            # عمليات حسابية
            'ح س ب': {'base': 'حساب', 'region': 'EVALUATE', 'patterns': ['فعل', 'مصدر', 'اسم فاعل', 'اسم مفعول']},
            'ج م ع': {'base': 'جمع', 'region': 'SUM_OPERATION', 'patterns': ['فعل', 'مصدر', 'اسم فاعل', 'اسم مفعول']},
            'ط ر ح': {'base': 'طرح', 'region': 'SUB_OPERATION', 'patterns': ['فعل', 'مصدر', 'اسم فاعل']},
            'ض ر ب': {'base': 'ضرب', 'region': 'MATMUL', 'patterns': ['فعل', 'مصدر', 'اسم فاعل', 'اسم مفعول']},
            'ق س م': {'base': 'قسمة', 'region': 'DIV_OPERATION', 'patterns': ['فعل', 'مصدر', 'اسم فاعل', 'اسم مفعول']},
            # كائنات رياضية
            'ص ف ف': {'base': 'مصفوفة', 'region': 'MATRIX_TENSOR', 'patterns': ['اسم', 'اسم جمع', 'صفة']},
            'و ج ه': {'base': 'متجه', 'region': 'VECTOR_TENSOR', 'patterns': ['اسم', 'اسم جمع', 'صفة']},
            'ح د د': {'base': 'محدد', 'region': 'DETERMINANT', 'patterns': ['اسم مفعول', 'فعل', 'مصدر']},
            'ع ك س': {'base': 'معكوس', 'region': 'INVERSE', 'patterns': ['اسم مفعول', 'فعل', 'مصدر']},
            'ن ق ل': {'base': 'منقول', 'region': 'TRANSPOSE', 'patterns': ['اسم مفعول', 'فعل', 'مصدر']},
            # دوال وتحويلات
            'د و ل': {'base': 'دالة', 'region': 'FUNCTION', 'patterns': ['اسم', 'اسم جمع', 'فعل']},
            'ح و ل': {'base': 'تحويل', 'region': 'TRANSFORM', 'patterns': ['مصدر', 'فعل', 'اسم فاعل']},
            'ك م ل': {'base': 'تكامل', 'region': 'INTEGRAL', 'patterns': ['اسم', 'مصدر', 'فعل']},
            'ش ت ق': {'base': 'اشتقاق', 'region': 'DERIVATIVE', 'patterns': ['مصدر', 'فعل', 'اسم مفعول']},
            # شبكات عصبية
            'ش ب ك': {'base': 'شبكة', 'region': 'NEURAL_NETWORK', 'patterns': ['اسم', 'فعل', 'صفة']},
            'ع ص ب': {'base': 'عصبية', 'region': 'NEURAL_WEIGHTS', 'patterns': ['صفة', 'اسم', 'فعل']},
            'ط ب ق': {'base': 'طبقة', 'region': 'LAYER_DIMENSION', 'patterns': ['اسم', 'اسم جمع', 'فعل']},
            'ن ش ط': {'base': 'تنشيط', 'region': 'ACTIVATION_FUNC', 'patterns': ['مصدر', 'فعل', 'اسم فاعل']},
            'و ز ن': {'base': 'وزن', 'region': 'WEIGHT_MATRIX', 'patterns': ['اسم', 'اسم جمع', 'فعل']},
            'ن ح ي ز': {'base': 'انحياز', 'region': 'BIAS_VECTOR', 'patterns': ['اسم', 'فعل', 'صفة']},
            # إحصاء واحتمالات
            'ح ص ل': {'base': 'حاصل', 'region': 'RESULT_TENSOR', 'patterns': ['اسم فاعل', 'فعل', 'مصدر']},
            'ق ي م': {'base': 'قيمة', 'region': 'SCALAR_VALUE', 'patterns': ['اسم', 'اسم جمع', 'فعل']},
            'ف ر ق': {'base': 'فرق', 'region': 'SUB_OPERATION', 'patterns': ['اسم', 'فعل', 'مصدر']},
            'ن س ب': {'base': 'نسبة', 'region': 'RATIO', 'patterns': ['اسم', 'فعل', 'صفة']},
            'م ئ ة': {'base': 'مئوية', 'region': 'PERCENTAGE', 'patterns': ['اسم', 'صفة']},
            # تحسين وتدريب
            'د ر ب': {'base': 'تدريب', 'region': 'OPTIMIZATION_STEP', 'patterns': ['مصدر', 'فعل', 'اسم فاعل']},
            'ح س ن': {'base': 'تحسين', 'region': 'OPTIMIZATION', 'patterns': ['مصدر', 'فعل', 'اسم فاعل']},
            'خ ط أ': {'base': 'خطأ', 'region': 'LOSS_FUNCTION', 'patterns': ['اسم', 'فعل', 'صفة']},
            'ع ل م': {'base': 'تعلم', 'region': 'LEARNING', 'patterns': ['مصدر', 'فعل', 'اسم فاعل']},
            # جبر خطي
            'ف ض ي': {'base': 'فضاء', 'region': 'VECTOR_SPACE', 'patterns': ['اسم', 'صفة']},
            'أ س س': {'base': 'أساس', 'region': 'BASIS', 'patterns': ['اسم', 'فعل', 'صفة']},
            'ب ع د': {'base': 'بُعد', 'region': 'DIMENSION', 'patterns': ['اسم', 'فعل', 'صفة']},
            'ر ت ب': {'base': 'رتبة', 'region': 'RANK', 'patterns': ['اسم', 'فعل', 'مصدر']},
            'ن و ي': {'base': 'نواة', 'region': 'KERNEL', 'patterns': ['اسم', 'فعل', 'صفة']},
            # هندسة
            'ن ق ط': {'base': 'نقطة', 'region': 'POINT', 'patterns': ['اسم', 'فعل']},
            'خ ط ط': {'base': 'خط', 'region': 'LINE', 'patterns': ['اسم', 'فعل', 'صفة']},
            'س ط ح': {'base': 'سطح', 'region': 'SURFACE', 'patterns': ['اسم', 'فعل']},
            'ز و ي': {'base': 'زاوية', 'region': 'ANGLE', 'patterns': ['اسم', 'فعل', 'صفة']},
            'د و ر': {'base': 'دائرة', 'region': 'CIRCLE', 'patterns': ['اسم', 'فعل', 'صفة']},
            # أعداد
            'ع د د': {'base': 'عدد', 'region': 'NUMBER', 'patterns': ['اسم', 'فعل', 'صفة']},
            'ص ح ح': {'base': 'صحيح', 'region': 'INTEGER', 'patterns': ['صفة', 'اسم', 'فعل']},
            'ك س ر': {'base': 'كسر', 'region': 'FRACTION', 'patterns': ['اسم', 'فعل', 'صفة']},
            'ج ذ ر': {'base': 'جذر', 'region': 'ROOT', 'patterns': ['اسم', 'فعل', 'صفة']},
            # منطق
            'ث ب ت': {'base': 'ثابت', 'region': 'CONSTANT', 'patterns': ['صفة', 'اسم', 'فعل']},
            'غ ي ر': {'base': 'متغير', 'region': 'VARIABLE', 'patterns': ['اسم فاعل', 'فعل', 'مصدر']},
            'ع م ل': {'base': 'معامل', 'region': 'COEFFICIENT', 'patterns': ['اسم', 'فعل', 'صفة']},
            # فيزياء
            'ط ا ق': {'base': 'طاقة', 'region': 'ENERGY', 'patterns': ['اسم', 'فعل']},
            'ق و ي': {'base': 'قوة', 'region': 'FORCE', 'patterns': ['اسم', 'فعل', 'صفة']},
            'ك ت ل': {'base': 'كتلة', 'region': 'MASS', 'patterns': ['اسم', 'فعل']},
            'س ر ع': {'base': 'سرعة', 'region': 'VELOCITY', 'patterns': ['اسم', 'فعل', 'صفة']},
            'ت ر د د': {'base': 'تردد', 'region': 'FREQUENCY', 'patterns': ['اسم', 'فعل']},
            # كيمياء
            'ذ ر ة': {'base': 'ذرة', 'region': 'ATOM', 'patterns': ['اسم']},
            'ج ز ء': {'base': 'جزيء', 'region': 'MOLECULE', 'patterns': ['اسم']},
            'ت ك ا ف ؤ': {'base': 'تكافؤ', 'region': 'EQUIVALENCE', 'patterns': ['اسم', 'فعل']},
            # حاسوب
            'خ و ر ز': {'base': 'خوارزمية', 'region': 'ALGORITHM', 'patterns': ['اسم']},
            'ب ي ا ن': {'base': 'بيانات', 'region': 'DATA', 'patterns': ['اسم', 'اسم جمع']},
            'ن م و ذ ج': {'base': 'نموذج', 'region': 'MODEL', 'patterns': ['اسم', 'فعل', 'مصدر']},
            # إحصاء متقدم
            'ا ح ت م ا ل': {'base': 'احتمال', 'region': 'PROBABILITY', 'patterns': ['اسم', 'فعل']},
            'ت و ز ي ع': {'base': 'توزيع', 'region': 'DISTRIBUTION', 'patterns': ['اسم', 'فعل']},
            'م ت و س ط': {'base': 'متوسط', 'region': 'MEAN', 'patterns': ['اسم', 'صفة']},
            'ت ب ا ي ن': {'base': 'تباين', 'region': 'VARIANCE', 'patterns': ['اسم', 'فعل']},
            'ع ي ن ة': {'base': 'عينة', 'region': 'SAMPLE', 'patterns': ['اسم', 'فعل']},
            # تحليل رياضي
            'ن ه ي ة': {'base': 'نهاية', 'region': 'LIMIT', 'patterns': ['اسم', 'فعل']},
            'م ت س ل س ل ة': {'base': 'متسلسلة', 'region': 'SERIES', 'patterns': ['اسم', 'صفة']},
            'ت ق ا ر ب': {'base': 'تقارب', 'region': 'CONVERGENCE', 'patterns': ['اسم', 'فعل']},
            # دوال خاصة
            'ل و غ': {'base': 'لوغاريتم', 'region': 'LOGARITHM', 'patterns': ['اسم']},
            'أ س س': {'base': 'أس', 'region': 'EXPONENT', 'patterns': ['اسم', 'فعل']},
        }
        # الأوزان الصرفية الأساسية
        self.pattern_templates = {
            'فعل': ['فعل', 'يفعل', 'افعل'],
            'مصدر': ['فعل', 'فعال', 'فعال', 'فعل'],
            'اسم فاعل': ['فاعل', 'مفاعل', 'فعيل'],
            'اسم مفعول': ['مفعول', 'مفعال', 'مفعّل'],
            'صفة': ['فعيل', 'فعلي', 'فعّال'],
            'اسم': ['فعلة', 'فعل', 'فعال'],
            'اسم جمع': ['أفعال', 'فعالات', 'فعيل'],
        }
    def generate_derivations(self, root: str, base_word: str, patterns: List[str]) -> List[Dict]:
        """توليد الاشتقاقات من جذر واحد"""
        derivations = []
        # إضافة الكلمة الأساسية
        derivations.append({
            'word': base_word,
            'root': root,
            'pattern': 'أساسي',
            'pos': 'اسم',
            'source': base_word,
            'math_region': self.math_roots[root]['region']
        })
        # توليد اشتقاقات شائعة بناءً على الأنماط
        common_derivations = self._get_common_derivations(root, base_word)
        derivations.extend(common_derivations)
        return derivations
    def _get_common_derivations(self, root: str, base_word: str) -> List[Dict]:
        """توليد اشتقاقات شائعة من جذر"""
        derivations = []
        region = self.math_roots[root]['region']
        # اشتقاقات شائعة بناءً على الجذر
        if root == 'ح س ب':
            derivations.extend([
                {'word': 'احسب', 'root': root, 'pattern': 'افعل', 'pos': 'فعل أمر', 'source': 'حساب', 'math_region': region},
                {'word': 'حاسب', 'root': root, 'pattern': 'فاعل', 'pos': 'اسم فاعل', 'source': 'حساب', 'math_region': region},
                {'word': 'محسوب', 'root': root, 'pattern': 'مفعول', 'pos': 'اسم مفعول', 'source': 'حساب', 'math_region': region},
                {'word': 'حاسبة', 'root': root, 'pattern': 'فاعلة', 'pos': 'اسم', 'source': 'حساب', 'math_region': region},
                {'word': 'حسابات', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'حساب', 'math_region': region},
            ])
        elif root == 'ج م ع':
            derivations.extend([
                {'word': 'اجمع', 'root': root, 'pattern': 'افعل', 'pos': 'فعل أمر', 'source': 'جمع', 'math_region': region},
                {'word': 'جامع', 'root': root, 'pattern': 'فاعل', 'pos': 'اسم فاعل', 'source': 'جمع', 'math_region': region},
                {'word': 'مجموع', 'root': root, 'pattern': 'مفعول', 'pos': 'اسم مفعول', 'source': 'جمع', 'math_region': region},
                {'word': 'مجاميع', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'جمع', 'math_region': region},
                {'word': 'جامعة', 'root': root, 'pattern': 'فاعلة', 'pos': 'اسم', 'source': 'جمع', 'math_region': 'UNION'},
            ])
        elif root == 'ض ر ب':
            derivations.extend([
                {'word': 'اضرب', 'root': root, 'pattern': 'افعل', 'pos': 'فعل أمر', 'source': 'ضرب', 'math_region': region},
                {'word': 'ضارب', 'root': root, 'pattern': 'فاعل', 'pos': 'اسم فاعل', 'source': 'ضرب', 'math_region': region},
                {'word': 'مضروب', 'root': root, 'pattern': 'مفعول', 'pos': 'اسم مفعول', 'source': 'ضرب', 'math_region': region},
                {'word': 'ضرائب', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'ضرب', 'math_region': region},
            ])
        elif root == 'ق س م':
            derivations.extend([
                {'word': 'اقسم', 'root': root, 'pattern': 'افعل', 'pos': 'فعل أمر', 'source': 'قسم', 'math_region': region},
                {'word': 'قاسم', 'root': root, 'pattern': 'فاعل', 'pos': 'اسم فاعل', 'source': 'قسم', 'math_region': region},
                {'word': 'مقسوم', 'root': root, 'pattern': 'مفعول', 'pos': 'اسم مفعول', 'source': 'قسم', 'math_region': region},
                {'word': 'أقسام', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'قسم', 'math_region': region},
            ])
        elif root == 'ص ف ف':
            derivations.extend([
                {'word': 'مصفوفات', 'root': root, 'pattern': 'مفعولات', 'pos': 'اسم جمع', 'source': 'صف', 'math_region': region},
                {'word': 'صف', 'root': root, 'pattern': 'فعل', 'pos': 'اسم', 'source': 'صف', 'math_region': region},
                {'word': 'صفوف', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'صف', 'math_region': region},
            ])
        elif root == 'و ج ه':
            derivations.extend([
                {'word': 'متجهات', 'root': root, 'pattern': 'مفعولات', 'pos': 'اسم جمع', 'source': 'وجه', 'math_region': region},
                {'word': 'وجه', 'root': root, 'pattern': 'فعل', 'pos': 'اسم', 'source': 'وجه', 'math_region': region},
                {'word': 'أوجه', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'وجه', 'math_region': region},
            ])
        elif root == 'ش ب ك':
            derivations.extend([
                {'word': 'شبكات', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'شبكة', 'math_region': region},
                {'word': 'شبكة', 'root': root, 'pattern': 'فعلة', 'pos': 'اسم', 'source': 'شبكة', 'math_region': region},
                {'word': 'شبكة عصبية', 'root': root, 'pattern': 'اسم + صفة', 'pos': 'اسم مركب', 'source': 'شبكة عصبية', 'math_region': 'NEURAL_NETWORK'},
            ])
        elif root == 'ط ب ق':
            derivations.extend([
                {'word': 'طبقات', 'root': root, 'pattern': 'فعيلات', 'pos': 'اسم جمع', 'source': 'طبقة', 'math_region': region},
                {'word': 'طبقة', 'root': root, 'pattern': 'فعلة', 'pos': 'اسم', 'source': 'طبقة', 'math_region': region},
                {'word': 'طبقة مخفية', 'root': root, 'pattern': 'اسم + صفة', 'pos': 'اسم مركب', 'source': 'طبقة مخفية', 'math_region': 'HIDDEN_LAYER'},
            ])
        elif root == 'ن ش ط':
            derivations.extend([
                {'word': 'نشّط', 'root': root, 'pattern': 'فعّل', 'pos': 'فعل أمر', 'source': 'تنشيط', 'math_region': region},
                {'word': 'نشط', 'root': root, 'pattern': 'فعل', 'pos': 'مصدر', 'source': 'نشاط', 'math_region': region},
                {'word': 'نشاط', 'root': root, 'pattern': 'فعال', 'pos': 'مصدر', 'source': 'نشاط', 'math_region': region},
            ])
        elif root == 'و ز ن':
            derivations.extend([
                {'word': 'أوزان', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'وزن', 'math_region': region},
                {'word': 'وزن', 'root': root, 'pattern': 'فعل', 'pos': 'مصدر', 'source': 'وزن', 'math_region': region},
                {'word': 'أوزان', 'root': root, 'pattern': 'جمع', 'pos': 'اسم جمع', 'source': 'وزن', 'math_region': region},
            ])
        elif root == 'د ر ب':
            derivations.extend([
                {'word': 'درب', 'root': root, 'pattern': 'افعل', 'pos': 'فعل أمر', 'source': 'تدريب', 'math_region': region},
                {'word': 'مدرب', 'root': root, 'pattern': 'مفعل', 'pos': 'اسم', 'source': 'تدريب', 'math_region': region},
                {'word': 'تدريبات', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'تدريب', 'math_region': region},
            ])
        elif root == 'خ ط أ':
            derivations.extend([
                {'word': 'أخطاء', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'خطأ', 'math_region': region},
                {'word': 'خطأ', 'root': root, 'pattern': 'فعل', 'pos': 'اسم', 'source': 'خطأ', 'math_region': region},
                {'word': 'دالة خطأ', 'root': root, 'pattern': 'اسم + اسم', 'pos': 'اسم مركب', 'source': 'دالة خطأ', 'math_region': 'LOSS_FUNCTION'},
            ])
        elif root == 'ق ي م':
            derivations.extend([
                {'word': 'قيم', 'root': root, 'pattern': 'فعيل', 'pos': 'اسم جمع', 'source': 'قوم', 'math_region': region},
                {'word': 'قيمة', 'root': root, 'pattern': 'فعلة', 'pos': 'اسم', 'source': 'قوم', 'math_region': region},
                {'word': 'تقييم', 'root': root, 'pattern': 'تفعيل', 'pos': 'مصدر', 'source': 'تقييم', 'math_region': region},
            ])
        elif root == 'ف ر ق':
            derivations.extend([
                {'word': 'افرق', 'root': root, 'pattern': 'افعل', 'pos': 'فعل أمر', 'source': 'فرق', 'math_region': region},
                {'word': 'فروق', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'فرق', 'math_region': region},
                {'word': 'فرق', 'root': root, 'pattern': 'فعل', 'pos': 'اسم', 'source': 'فرق', 'math_region': region},
            ])
        elif root == 'ش ت ق':
            derivations.extend([
                {'word': 'اشتق', 'root': root, 'pattern': 'افتعل', 'pos': 'فعل أمر', 'source': 'اشتقاق', 'math_region': region},
                {'word': 'مشتق', 'root': root, 'pattern': 'مفعل', 'pos': 'اسم مفعول', 'source': 'اشتقاق', 'math_region': region},
                {'word': 'اشتقاق', 'root': root, 'pattern': 'افتعال', 'pos': 'مصدر', 'source': 'اشتقاق', 'math_region': region},
            ])
        elif root == 'ح و ل':
            derivations.extend([
                {'word': 'حوّل', 'root': root, 'pattern': 'فعّل', 'pos': 'فعل أمر', 'source': 'تحويل', 'math_region': region},
                {'word': 'محول', 'root': root, 'pattern': 'مفعل', 'pos': 'اسم', 'source': 'تحويل', 'math_region': region},
                {'word': 'تحويلات', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'تحويل', 'math_region': region},
            ])
        elif root == 'ك م ل':
            derivations.extend([
                {'word': 'تكامل', 'root': root, 'pattern': 'افتعال', 'pos': 'اسم', 'source': 'كمال', 'math_region': region},
                {'word': 'كامل', 'root': root, 'pattern': 'فاعل', 'pos': 'صفة', 'source': 'كمال', 'math_region': region},
            ])
        elif root == 'د و ل':
            derivations.extend([
                {'word': 'دوال', 'root': root, 'pattern': 'فعال', 'pos': 'اسم جمع', 'source': 'دول', 'math_region': region},
                {'word': 'دالة', 'root': root, 'pattern': 'فالة', 'pos': 'اسم', 'source': 'دول', 'math_region': region},
            ])
        elif root == 'ح ص ل':
            derivations.extend([
                {'word': 'حاصل', 'root': root, 'pattern': 'فاعل', 'pos': 'اسم فاعل', 'source': 'حصول', 'math_region': region},
                {'word': 'حصول', 'root': root, 'pattern': 'فعل', 'pos': 'مصدر', 'source': 'حصول', 'math_region': region},
            ])
        elif root == 'ن س ب':
            derivations.extend([
                {'word': 'نسبة', 'root': root, 'pattern': 'فعلة', 'pos': 'اسم', 'source': 'نسبة', 'math_region': region},
                {'word': 'نسب', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'نسبة', 'math_region': region},
                {'word': 'نسبي', 'root': root, 'pattern': 'فعلي', 'pos': 'صفة', 'source': 'نسبة', 'math_region': region},
            ])
        elif root == 'ح س ن':
            derivations.extend([
                {'word': 'حسن', 'root': root, 'pattern': 'فعل', 'pos': 'مصدر', 'source': 'حسن', 'math_region': region},
                {'word': 'محسن', 'root': root, 'pattern': 'مفعل', 'pos': 'اسم', 'source': 'تحسين', 'math_region': region},
            ])
        elif root == 'ف ض ي':
            derivations.extend([
                {'word': 'فضاء', 'root': root, 'pattern': 'فعال', 'pos': 'اسم', 'source': 'فضاء', 'math_region': region},
                {'word': 'فضاءات', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'فضاء', 'math_region': region},
                {'word': 'فضاء متجهي', 'root': root, 'pattern': 'اسم + صفة', 'pos': 'اسم مركب', 'source': 'فضاء متجهي', 'math_region': 'VECTOR_SPACE'},
            ])
        elif root == 'أ س س':
            derivations.extend([
                {'word': 'أساس', 'root': root, 'pattern': 'فعل', 'pos': 'اسم', 'source': 'أساس', 'math_region': region},
                {'word': 'أسس', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'أساس', 'math_region': region},
                {'word': 'أساسي', 'root': root, 'pattern': 'فعالي', 'pos': 'صفة', 'source': 'أساس', 'math_region': region},
            ])
        elif root == 'ب ع د':
            derivations.extend([
                {'word': 'بُعد', 'root': root, 'pattern': 'فعل', 'pos': 'اسم', 'source': 'بُعد', 'math_region': region},
                {'word': 'أبعاد', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'بُعد', 'math_region': region},
                {'word': 'بعدي', 'root': root, 'pattern': 'فعلي', 'pos': 'صفة', 'source': 'بُعد', 'math_region': region},
            ])
        elif root == 'ر ت ب':
            derivations.extend([
                {'word': 'رتبة', 'root': root, 'pattern': 'فعلة', 'pos': 'اسم', 'source': 'رتبة', 'math_region': region},
                {'word': 'رتب', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'رتبة', 'math_region': region},
                {'word': 'ترتيب', 'root': root, 'pattern': 'تفعيل', 'pos': 'مصدر', 'source': 'ترتيب', 'math_region': region},
            ])
        elif root == 'ن و ي':
            derivations.extend([
                {'word': 'نواة', 'root': root, 'pattern': 'فعالة', 'pos': 'اسم', 'source': 'نواة', 'math_region': region},
                {'word': 'أنوية', 'root': root, 'pattern': 'أفعال', 'pos': 'اسم جمع', 'source': 'نواة', 'math_region': region},
                {'word': 'نوي', 'root': root, 'pattern': 'فعلي', 'pos': 'صفة', 'source': 'نواة', 'math_region': region},
            ])
        return derivations
    def generate_full_dictionary(self) -> List[Dict]:
        """توليد القاموس الكامل"""
        full_dict = []
        for root, info in self.math_roots.items():
            derivations = self.generate_derivations(root, info['base'], info['patterns'])
            full_dict.extend(derivations)
        # إضافة ثوابت ودوال خاصة
        special_terms = [
            {'word': 'ReLU', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'ReLU', 'math_region': 'RELU_FUNC'},
            {'word': 'Sigmoid', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Sigmoid', 'math_region': 'SIGMOID_FUNC'},
            {'word': 'Tanh', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Tanh', 'math_region': 'TANH_FUNC'},
            {'word': 'Softmax', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Softmax', 'math_region': 'SOFTMAX_FUNC'},
            {'word': 'PCA', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'PCA', 'math_region': 'PCA'},
            {'word': 'SVD', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'SVD', 'math_region': 'SVD'},
            {'word': 'CNN', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'CNN', 'math_region': 'CNN'},
            {'word': 'RNN', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'RNN', 'math_region': 'RNN'},
            {'word': 'GAN', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'GAN', 'math_region': 'GAN'},
            {'word': 'LSTM', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'LSTM', 'math_region': 'LSTM'},
            {'word': 'BERT', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'BERT', 'math_region': 'BERT'},
            {'word': 'GPT', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'GPT', 'math_region': 'GPT'},
            {'word': 'Adam', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Adam', 'math_region': 'ADAM_OPTIMIZER'},
            {'word': 'SGD', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'SGD', 'math_region': 'SGD'},
            {'word': 'MSE', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'MSE', 'math_region': 'MSE'},
            {'word': 'MAE', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'MAE', 'math_region': 'MAE'},
            {'word': 'NLP', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'NLP', 'math_region': 'NLP'},
            {'word': 'API', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'API', 'math_region': 'API'},
            {'word': 'GPU', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'GPU', 'math_region': 'GPU'},
            {'word': 'CPU', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'CPU', 'math_region': 'CPU'},
            {'word': 'RAM', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'RAM', 'math_region': 'RAM'},
            {'word': 'SSD', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'SSD', 'math_region': 'SSD'},
            {'word': 'HTTP', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'HTTP', 'math_region': 'HTTP'},
            {'word': 'JSON', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'JSON', 'math_region': 'JSON'},
            {'word': 'XML', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'XML', 'math_region': 'XML'},
            {'word': 'SQL', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اختصار', 'source': 'SQL', 'math_region': 'SQL'},
            {'word': 'NoSQL', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'NoSQL', 'math_region': 'NOSQL'},
            {'word': 'REST', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'REST', 'math_region': 'REST'},
            {'word': 'GraphQL', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'GraphQL', 'math_region': 'GRAPHQL'},
            {'word': 'Docker', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Docker', 'math_region': 'DOCKER'},
            {'word': 'Kubernetes', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Kubernetes', 'math_region': 'KUBERNETES'},
            {'word': 'Linux', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Linux', 'math_region': 'LINUX'},
            {'word': 'Windows', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Windows', 'math_region': 'WINDOWS'},
            {'word': 'macOS', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'macOS', 'math_region': 'MACOS'},
            {'word': 'Python', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Python', 'math_region': 'PYTHON'},
            {'word': 'Rust', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Rust', 'math_region': 'RUST'},
            {'word': 'C++', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'C++', 'math_region': 'CPP'},
            {'word': 'Java', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Java', 'math_region': 'JAVA'},
            {'word': 'JavaScript', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'JavaScript', 'math_region': 'JAVASCRIPT'},
            {'word': 'TypeScript', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'TypeScript', 'math_region': 'TYPESCRIPT'},
            {'word': 'Go', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Go', 'math_region': 'GO'},
            {'word': 'Swift', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Swift', 'math_region': 'SWIFT'},
            {'word': 'Kotlin', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Kotlin', 'math_region': 'KOTLIN'},
            {'word': 'Ruby', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Ruby', 'math_region': 'RUBY'},
            {'word': 'PHP', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'PHP', 'math_region': 'PHP'},
            {'word': 'Scala', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Scala', 'math_region': 'SCALA'},
            {'word': 'Haskell', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Haskell', 'math_region': 'HASKELL'},
            {'word': 'Elixir', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Elixir', 'math_region': 'ELIXIR'},
            {'word': 'Erlang', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Erlang', 'math_region': 'ERLANG'},
            {'word': 'Julia', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'Julia', 'math_region': 'JULIA'},
            {'word': 'MATLAB', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'MATLAB', 'math_region': 'MATLAB'},
            {'word': 'R', 'root': 'N/A', 'pattern': 'اسم علم', 'pos': 'اسم علم', 'source': 'R', 'math_region': 'R_LANG'},
        ]
        full_dict.extend(special_terms)
        # إزالة التكرارات
        seen_words = set()
        unique_dict = []
        for entry in full_dict:
            if entry['word'] not in seen_words:
                seen_words.add(entry['word'])
                unique_dict.append(entry)
        return unique_dict
def main():
    generator = ComprehensiveDictionaryGenerator()
    print("🧠 توليد القاموس الشامل من الجذور العربية...")
    full_dict = generator.generate_full_dictionary()
    print(f"✅ تم توليد {len(full_dict)} مدخل في القاموس")
    # حفظ القاموس
    output_path = "/root/ilm-al-kitab/MAL/examples/ai/mal_tiny/arabic_morphology_dict.json"
    # دمج مع القاموس الحالي إذا وجد
    if os.path.exists(output_path):
        with open(output_path, 'r', encoding='utf-8') as f:
            existing_dict = json.load(f)
        existing_words = {entry['word'] for entry in existing_dict}
        new_entries = [e for e in full_dict if e['word'] not in existing_words]
        print(f"✅ إضافة {len(new_entries)} مدخل جديد إلى القاموس الحالي ({len(existing_dict)} مدخل)")
        final_dict = existing_dict + new_entries
    else:
        final_dict = full_dict
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(final_dict, f, ensure_ascii=False, indent=2)
    print(f"✅ تم حفظ القاموس النهائي ({len(final_dict)} مدخل)")
    # إحصائيات
    regions = {}
    for entry in final_dict:
        region = entry.get('math_region', 'GENERAL')
        regions[region] = regions.get(region, 0) + 1
    print("\n📊 توزيع المناطق الرياضية:")
    for region, count in sorted(regions.items(), key=lambda x: x[1], reverse=True)[:15]:
        print(f"  {region}: {count} كلمة")
    # عينة
    print("\n📋 عينة من القاموس المولد:")
    for entry in final_dict[:10]:
        print(f"  {entry['word']} -> {entry['math_region']} (جذر: {entry['root']})")
if __name__ == "__main__":
    main()
