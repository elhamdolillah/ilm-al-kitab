#!/usr/bin/env python3
"""
MAL Code Generator v4: توليد كود يتطابق تماماً مع دستور وصيغة MAL الحقيقية
"""
import torch, torch.nn as nn, sys, os, re, uuid
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
        return self.intent(self.agg(a.mean(1)) + x.mean(1)), self.entity(self.agg(a.mean(1)) + x.mean(1))
class MALCodeGeneratorV4:
    def __init__(self, model_path='MHLJ_Fusion_Head_100pct.pt'):
        self.mal_tok = MALTokenizer()
        self.mal_model = MALTiny(self.mal_tok.vocab_size)
        self.mal_model.load_state_dict(torch.load('MAL-Tiny-Baseline-80pct.pt', map_location='cpu'))
        self.mal_model.eval()
        self.head = FusionHead100()
        self.head.load_state_dict(torch.load(model_path, map_location='cpu'))
        self.head.eval()
        self.intents = ['eval_expr', 'prove_stmt', 'define_var', 'query_knowledge']
        self.entities = ['variable', 'number', 'operator', 'function']
        self.output_dir = os.path.join(current_dir, 'generated_mal_v4')
        os.makedirs(self.output_dir, exist_ok=True)
    def predict(self, text):
        inp = torch.tensor([self.mal_tok.encode(text)])
        with torch.no_grad():
            i_out, e_out = self.head(self.mal_model.tok_emb(inp))
        intent = self.intents[torch.argmax(i_out, -1).item()]
        if intent == 'prove_stmt' and re.search(r'\d+', text) and not any(w in text for w in ['أثبت', 'برهن', 'أظهر']):
            intent = 'eval_expr'
        return intent, self.entities[torch.argmax(e_out, -1).item()]
    def extract_entities(self, text):
        ents = {'variables': [], 'numbers': [], 'operators': [], 'functions': [], 'name': 'task'}
        var_match = re.search(r'(?:متغير|اسمه|ليكن|عرّف|أنشئ)\s+([أ-ي])\b', text)
        if var_match: ents['variables'].append(var_match.group(1))
        else:
            particles = {'في', 'من', 'إلى', 'هو', 'ما', 'أن', 'لن', 'قد', 'لا', 'و', 'أو', 'ثم', 'بل', 'حتى', 'عن', 'مع', 'على', 'إلا', 'إن', 'لم', 'كي', 'لو', 'لأن', 'ب', 'ك', 'ل'}
            ents['variables'] = list(set([c for c in re.findall(r'(?:^|\s)([أ-ي])(?:\s|$|,|:)', text) if c not in particles]))
        ents['numbers'] = re.findall(r'\b(\d+(?:\.\d+)?)\b', text)
        ents['operators'] = list(set(re.findall(r'([+\-*/=<>≤≥≠])', text)))
        func_match = re.search(r'الدالة\s+([a-zA-Z0-9_]+)', text) or re.search(r'دالة\s+([أ-ي]+)', text)
        if func_match: ents['functions'].append(func_match.group(1))
        return ents
    def generate(self, text, save_to_file=True):
        intent, _ = self.predict(text)
        ents = self.extract_entities(text)
        uid = uuid.uuid4().hex[:6]
        if intent == 'define_var':
            v = ents['variables'][0] if ents['variables'] else 'x'
            code = f"// تعريف متغير: {v}\nfn init_{v}() -> i32 {{\n    let {v}: i32 = 0;\n    return {v};\n}}\n"
        elif intent == 'eval_expr':
            expr = f"{ents['numbers'][0]} {ents['operators'][0] if ents['operators'] else '+'} {ents['numbers'][1] if len(ents['numbers'])>1 else '0'}" if ents['numbers'] else "0"
            code = f"// تعبير تقييمي\nfn eval_expr_{uid}() -> i32 {{\n    let result: i32 = {expr};\n    return result;\n}}\n"
        elif intent == 'prove_stmt':
            stmt = text.replace('أثبت أن', '').replace('برهن أن', '').replace('أظهر أن', '').strip()
            code = f"// نظرية: {stmt}\nfn theorem_{uid}() -> bool {{\n    // إثبات: {stmt}\n    return true;\n}}\n"
        else:
            func = ents['functions'][0] if ents['functions'] else 'unknown'
            code = f"// استعلام معرفي\nfn query_{uid}() -> i32 {{\n    // الدالة المطلوبة: {func}\n    // {text}\n    return 0;\n}}\n"
        if save_to_file:
            filepath = os.path.join(self.output_dir, f"valid_mal_{intent}_{uid}.mal")
            with open(filepath, 'w', encoding='utf-8') as f: f.write(code)
        return code.strip()
if __name__ == '__main__':
    gen = MALCodeGeneratorV4()
    tests = ["ليكن س متغيراً جديداً", "احسب قيمة 15 + 25", "أثبت أن أ = ب", "ما هو تعريف الدالة f؟"]
    for t in tests:
        print(f"\n📝 {t}\n{gen.generate(t)}\n" + "-"*40)
