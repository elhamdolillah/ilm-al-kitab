import torch
import torch.nn as nn
import sys
import os
# إصلاح مسارات الاستيراد لمنع التعارض
current_dir = os.path.dirname(os.path.abspath(__file__))
regex_dir = os.path.join(current_dir, 'regex_tiny')
# 1. تحميل النموذج الرئيسي أولاً
sys.path.insert(0, current_dir)
from model import MALTiny, MALTokenizer
# 2. تحميل مكونات Hybrid
sys.path.insert(0, regex_dir)
from hybrid_tokenizer import HybridTokenizer
from hybrid_classifier import HybridClassifier
class FusionHead(nn.Module):
    def __init__(self):
        super().__init__()
        self.laya_attention = nn.MultiheadAttention(embed_dim=64, num_heads=4, batch_first=True)
        self.laya_aggregation = nn.Sequential(nn.Linear(64, 256), nn.GELU(), nn.Linear(256, 64))
        self.intent_router = nn.Sequential(nn.Linear(64, 128), nn.GELU(), nn.Linear(128, 4))
        self.entity_router = nn.Sequential(nn.Linear(64, 128), nn.GELU(), nn.Linear(128, 4))
    def forward(self, x):
        attn, _ = self.laya_attention(x, x, x)
        laya = self.laya_aggregation(attn.mean(dim=1))
        pooled = x.mean(dim=1)
        rich = laya + pooled
        return {'intent': self.intent_router(rich), 'entity': self.entity_router(rich)}
# تحميل النماذج
mal_tokenizer = MALTokenizer()
mal_model = MALTiny(mal_tokenizer.vocab_size)
mal_model.load_state_dict(torch.load('MAL-Tiny-Baseline-80pct.pt', map_location='cpu'))
mal_model.eval()
hybrid_tokenizer = HybridTokenizer()
hybrid_classifier = HybridClassifier(vocab_size=hybrid_tokenizer.vocab_size)
hybrid_classifier.load_state_dict(torch.load('regex_tiny/Hybrid-Classifier-v2.0-Expanded.pt', map_location='cpu'))
hybrid_classifier.eval()
# تحميل رأس الدمج المدرب
fusion_head = FusionHead()
if os.path.exists('MHLJ_Fusion_Head_MAL_Best.pt'):
    fusion_head.load_state_dict(torch.load('MHLJ_Fusion_Head_MAL_Best.pt'))
    print('✅ تم تحميل أفضل نموذج محفوظ (Best)')
elif os.path.exists('MHLJ_Fusion_Head_MAL_100pct.pt'):
    fusion_head.load_state_dict(torch.load('MHLJ_Fusion_Head_MAL_100pct.pt'))
    print('✅ تم تحميل نموذج 100pct')
else:
    print('⚠️ لم يتم العثور على نموذج مدرب، سيتم استخدام الأوزان العشوائية')
intent_labels = ['eval_expr', 'prove_stmt', 'define_var', 'query_knowledge']
entity_labels = ['variable', 'number', 'operator', 'function']
test_cases = [
    ('احسب ناتج الرقم 10 والرقم 2', 'eval_expr', 'number'),
    ('ليكن ص متغيراً جديداً', 'define_var', 'variable'),
    ('برهن أن س > 0 باستخدام العامل >', 'prove_stmt', 'operator'),
    ('ما هو تعريف الدالة الرياضية؟', 'query_knowledge', 'function'),
    ('اشرح معنى الدالة f(x)', 'query_knowledge', 'function'),
    ('احسب قيمة 15 + 25', 'eval_expr', 'number'),
]
correct = 0
for text, exp_i, exp_e in test_cases:
    tokens = torch.tensor([mal_tokenizer.encode(text)])
    with torch.no_grad():
        emb = mal_model.tok_emb(tokens)
        out = fusion_head(emb)
    pred_i = intent_labels[torch.argmax(out['intent'], dim=-1).item()]
    pred_e = entity_labels[torch.argmax(out['entity'], dim=-1).item()]
    match = '✅' if (pred_i == exp_i and pred_e == exp_e) else '❌'
    if match == '✅': correct += 1
    print(f'{match} {text}')
    print(f'   ↳ النية: {pred_i} | الكيان: {pred_e}')
print(f'\n🏆 الدقة النهائية: {correct}/{len(test_cases)} ({correct/len(test_cases)*100:.0f}%)')
