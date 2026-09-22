import torch
import json
import re
import sys
sys.path.insert(0, '.')
from hybrid_tokenizer import HybridTokenizer
from hybrid_classifier import HybridClassifier
# ═══════════════════════════════════════════════════════════
# بيانات تدريب جديدة (5 مهام إضافية)
# ═══════════════════════════════════════════════════════════
new_training_data = [
    # Phone numbers (1)
    ("Phone: +1-234-567-8900", 1), ("Call: +44 20 7946 0958", 1),
    ("Mobile: 123-456-7890", 1), ("No phone here", 0),
    # Dates (1)
    ("Date: 2024-01-15", 1), ("Born: 15/03/2024", 1),
    ("Event: 1-5-2024", 1), ("No date here", 0),
    # Currencies (1)
    ("Price: $100.50", 1), ("Cost: € 50", 1),
    ("Total: £1,234.56", 1), ("No price here", 0),
    # Percentages (1)
    ("Discount: 50%", 1), ("Interest: 75.5%", 1),
    ("Complete: 100%", 1), ("No percentage", 0),
    # IP addresses (1)
    ("Server: 192.168.1.1", 1), ("Gateway: 10.0.0.1", 1),
    ("DNS: 8.8.8.8", 1), ("No IP here", 0),
]
print("🚀 Fine-tuning on 5 new tasks...")
tok = HybridTokenizer()
model = HybridClassifier(vocab_size=tok.vocab_size)
model.load_state_dict(torch.load('Hybrid-Classifier-v1.0.pt', map_location='cpu'))
opt = torch.optim.AdamW(model.parameters(), lr=1e-3)
for epoch in range(1, 31):
    model.train()
    total_loss, correct = 0, 0
    for text, label in new_training_data:
        ids = tok.encode(text, max_len=256)
        x = torch.tensor([ids], dtype=torch.long)
        y = torch.tensor([label], dtype=torch.long)
        logits, loss = model(x, y)
        opt.zero_grad()
        loss.backward()
        opt.step()
        total_loss += loss.item()
        if logits.argmax(dim=-1) == y:
            correct += 1
    if epoch % 10 == 0:
        print(f"  Epoch {epoch}/30: loss={total_loss/len(new_training_data):.4f}, acc={correct/len(new_training_data):.0%}")
torch.save(model.state_dict(), 'Hybrid-Classifier-v2.0-Expanded.pt')
print("\n🏆 Expanded classifier saved: Hybrid-Classifier-v2.0-Expanded.pt")
# تحديث Symbolic Extractor
symbolic_patterns = {
    'email': r'[\w\.-]+@[\w\.-]+\.\w+',
    'url': r'https?://[^\s]+',
    'phone': r'\+?[\d\s-]{10,}',
    'date': r'\d{1,2}[-/]\d{1,2}[-/]\d{2,4}',
    'currency': r'[\$€£¥]\s*[\d,]+\.?\d*',
    'percentage': r'\d+\.?\d*%',
    'ip': r'\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b',
}
with open('expanded_patterns.json', 'w') as f:
    json.dump(symbolic_patterns, f, indent=2)
print("📁 Saved: expanded_patterns.json (7 tasks total)")
