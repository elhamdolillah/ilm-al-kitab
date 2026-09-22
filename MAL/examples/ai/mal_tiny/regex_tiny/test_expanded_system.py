import torch
import json
import re
import sys
sys.path.insert(0, '.')
from hybrid_tokenizer import HybridTokenizer
from hybrid_classifier import HybridClassifier
tok = HybridTokenizer()
model = HybridClassifier(vocab_size=tok.vocab_size)
model.load_state_dict(torch.load('Hybrid-Classifier-v2.0-Expanded.pt', map_location='cpu'))
model.eval()
with open('expanded_patterns.json', 'r') as f:
    patterns = json.load(f)
tests = [
    ("Contact: user@example.com", 'email', ["user@example.com"]),
    ("Visit https://example.com", 'url', ["https://example.com"]),
    ("Call me at +1-234-567-8900", 'phone', ["+1-234-567-8900"]),
    ("Event on 2024-01-15", 'date', ["2024-01-15"]),
    ("Price is $100.50", 'currency', ["$100.50"]),
    ("Discount: 50%", 'percentage', ["50%"]),
    ("Server IP: 192.168.1.1", 'ip', ["192.168.1.1"]),
    ("Just plain text with no data", 'email', []), 
]
print("\n🧪 Testing Expanded Hybrid System:")
correct = 0
for text, task, expected in tests:
    ids = tok.encode(text)
    x = torch.tensor([ids])
    pred = model.predict(x)
    if pred.item() == 1:
        if task == 'email': result = re.findall(patterns['email'], text)
        elif task == 'url': result = re.findall(patterns['url'], text)
        elif task == 'phone': result = re.findall(patterns['phone'], text)
        elif task == 'date': result = re.findall(patterns['date'], text)
        elif task == 'currency': result = re.findall(patterns['currency'], text)
        elif task == 'percentage': result = re.findall(patterns['percentage'], text)
        elif task == 'ip': result = re.findall(patterns['ip'], text)
        else: result = []
    else:
        result = []
    if result == expected:
        correct += 1
        print(f"  ✅ {text[:40]}... -> {result}")
    else:
        print(f"  ❌ {text[:40]}...")
        print(f"     Expected: {expected}")
        print(f"     Got: {result}")
print(f"\n🎯 Expanded System Accuracy: {correct/len(tests):.0%} ({correct}/{len(tests)})")
