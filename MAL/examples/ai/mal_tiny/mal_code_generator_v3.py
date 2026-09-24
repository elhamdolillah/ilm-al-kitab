#!/usr/bin/env python3
"""
MAL Code Generator v3: تحسينات دقيقة في استخراج الكيانات، أسماء ملفات فريدة، ومنطق تصنيف أفضل.
"""
import torch
import torch.nn as nn
import sys
import os
import re
import uuid
from datetime import datetime
current_dir = os.path.dirname(os.path.abspath(__file__))
regex_dir = os.path.join(current_dir, 'regex_tiny')
if current_dir not in sys.path: sys.path.insert(0, current_dir)
from model import MALTiny, MALTokenizer
if regex_dir not in sys.path: sys.path.insert(0, regex_dir)
from hybrid_tokenizer import HybridTokenizer
from hybrid_classifier import HybridClassifier
class FusionHead100(nn.Module):
    def __init__(self):
        super().__init__()
        self.attn = nn.MultiheadAttention(64, 4, batch_first=True)
        self.agg = nn.Sequential(nn.Linear(64, 256), nn.GELU(), nn.Dropout(0.3), nn.Linear(256, 128), nn.GELU(), nn.Dropout(0.3), nn.Linear(128, 64))
        self.intent = nn.Sequential(nn.Linear(64, 128), nn.GELU(), nn.Dropout(0.3), nn.Linear(128, 4))
        self.entity = nn.Sequential(nn.Linear(64, 128), nn.GELU(), nn.Dropout(0.3), nn.Linear(128, 4))
    def forward(self, x):
        a, _ = self.attn(x, x, x)
        rich = self.agg(a.mean(1)) + x.mean(1)
        return self.intent(rich), self.entity(rich)
class MALCodeGeneratorV3:
    def __init__(self, model_path='MHLJ_Fusion_Head_100pct.pt'):
        print("⚙️ جاري تحميل نموذج MHLJ Fusion (100% Accuracy)...")
        self.mal_tok = MALTokenizer()
        self.mal_model = MALTiny(self.mal_tok.vocab_size)
        self.mal_model.load_state_dict(torch.load('MAL-Tiny-Baseline-80pct.pt', map_location='cpu'))
        self.mal_model.eval()
        self.hybrid_tok = HybridTokenizer()
        self.hybrid_clf = HybridClassifier(vocab_size=self.hybrid_tok.vocab_size)
        self.hybrid_clf.load_state_dict(torch.load('regex_tiny/Hybrid-Classifier-v2.0-Expanded.pt', map_location='cpu'))
        self.hybrid_clf.eval()
        self.head = FusionHead100()
        self.head.load_state_dict(torch.load(model_path, map_location='cpu'))
        self.head.eval()
        self.intents = ['eval_expr', 'prove_stmt', 'define_var', 'query_knowledge']
        self.entities = ['variable', 'number', 'operator', 'function']
        self.output_dir = os.path.join(current_dir, 'generated_mal_v3')
        os.makedirs(self.output_dir, exist_ok=True)
        print("✅ مولد كود MAL v3 جاهز!\n")
    def predict(self, text):
        inp = torch.tensor([self.mal_tok.encode(text)])
        with torch.no_grad():
            emb = self.mal_model.tok_emb(inp)
            i_out, e_out = self.head(emb)
        pred_intent = self.intents[torch.argmax(i_out, -1).item()]
        pred_entity = self.entities[torch.argmax(e_out, -1).item()]
        # تحسين منطقي: إذا كانت هناك أرقام وعوامل ولكن لا توجد كلمات برهان، فهي تعبير رياضي
        if pred_intent == 'prove_stmt' and re.search(r'\d+', text) and not any(w in text for w in ['أثبت', 'برهن', 'أظهر', 'نظرية']):
            pred_intent = 'eval_expr'
        return pred_intent, pred_entity
    def extract_entities(self, text):
        entities = {'variables': [], 'numbers': [], 'operators': [], 'functions': [], 'theorem_name': None}
        # 1. استخراج المتغيرات بذكاء (تجنب الأحرف داخل الكلمات مثل 'ف' في 'عرّف')
        var_match = re.search(r'(?:متغير|اسمه|ليكن|عرّف|أنشئ|نعرّف)\s+([أ-ي])\b', text)
        if var_match:
            entities['variables'].append(var_match.group(1))
        else:
            # fallback: أحرف مفردة معزولة باستثناء أدوات الربط الشائعة
            particles = {'في', 'من', 'إلى', 'هو', 'ما', 'أن', 'لن', 'قد', 'لا', 'و', 'أو', 'ثم', 'بل', 'حتى', 'عن', 'مع', 'على', 'إلا', 'إن', 'لم', 'كي', 'لو', 'لأن', 'ب', 'ك', 'ل'}
            candidates = re.findall(r'(?:^|\s)([أ-ي])(?:\s|$|,|:)', text)
            entities['variables'] = list(set([c for c in candidates if c not in particles]))
        # 2. الأرقام
        entities['numbers'] = re.findall(r'\b(\d+(?:\.\d+)?)\b', text)
        # 3. العوامل الرياضية
        entities['operators'] = list(set(re.findall(r'([+\-*/=<>≤≥≠])', text)))
        # 4. الدوال
        func_match = re.search(r'الدالة\s+([a-zA-Z0-9_]+)', text)
        if func_match:
            entities['functions'].append(func_match.group(1))
        else:
            func_match_ar = re.search(r'دالة\s+([أ-ي]+)', text)
            if func_match_ar:
                entities['functions'].append(func_match_ar.group(1))
        # 5. اسم النظرية
        if 'أثبت' in text or 'برهن' in text:
            if '=' in text: entities['theorem_name'] = 'equality_theorem'
            elif '>' in text or '<' in text: entities['theorem_name'] = 'inequality_theorem'
            else: entities['theorem_name'] = 'general_theorem'
        return entities
    def generate(self, text, save_to_file=True):
        intent, entity = self.predict(text)
        ents = self.extract_entities(text)
        print(f"🔍 التحليل: النية = [{intent}] | الكيان = [{entity}]")
        # قوالب محسنة
        if intent == 'define_var':
            var_name = ents['variables'][0] if ents['variables'] else 'x'
            mal_code = f"""-- تعريف متغير مولد آلياً بواسطة MHLJ v3
def {var_name} : Type := 
  -- القيمة الافتراضية
  default
"""
        elif intent == 'eval_expr':
            if ents['numbers'] and ents['operators']:
                expr = f"{ents['numbers'][0]} {ents['operators'][0]} {ents['numbers'][1] if len(ents['numbers']) > 1 else '0'}"
            else:
                expr = text
            mal_code = f"""-- تعبير تقييمي مولد آلياً
eval {expr}
"""
        elif intent == 'prove_stmt':
            theorem_name = ents['theorem_name'] or 'my_theorem'
            vars_str = ' '.join([f"({v} : Nat)" for v in ents['variables'][:2]]) if ents['variables'] else '(n : Nat)'
            stmt = text.replace('أثبت أن', '').replace('برهن أن', '').replace('أظهر أن', '').strip()
            mal_code = f"""-- نظرية مولدة آلياً بواسطة MHLJ v3
theorem {theorem_name} : ∀ {vars_str}, {stmt}
proof
  -- استراتيجية البرهان: استقراء أو تطابق
  admit
qed
"""
        else:
            func_name = ents['functions'][0] if ents['functions'] else 'unknown_function'
            mal_code = f"""-- استعلام معرفي عن دالة
-- الدالة المطلوبة: {func_name}
-- الرجوع إلى مكتبة MAL القياسية
"""
        if save_to_file:
            # استخدام UUID لضمان أسماء ملفات فريدة تماماً
            unique_id = uuid.uuid4().hex[:6]
            filename = f"generated_{intent}_{unique_id}.mal"
            filepath = os.path.join(self.output_dir, filename)
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(mal_code)
            print(f"💾 تم الحفظ في: {filename}")
        return mal_code.strip()
if __name__ == '__main__':
    generator = MALCodeGeneratorV3()
    test_cases = [
        "ليكن س متغيراً جديداً",
        "احسب قيمة 15 + 25",
        "أثبت أن أ = ب باستخدام العامل =",
        "ما هو تعريف الدالة الرياضية؟",
        "ما هو 15 - 5"  # اختبار التحسين المنطقي (يجب أن يكون eval_expr)
    ]
    print("="*70)
    print("🧪 اختبار مولد كود MAL v3 (محسّن)")
    print("="*70)
    for text in test_cases:
        print(f"\n📝 المدخل: '{text}'")
        generator.generate(text)
    print(f"\n✅ تم التوليد في: {generator.output_dir}")
