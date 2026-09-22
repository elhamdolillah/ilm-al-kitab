import torch
import json
from model import RegexTinyModel
from hybrid_tokenizer import HybridTokenizer
print("=" * 70)
print("Regex-Tiny v1.0: Training + Honest Evaluation")
print("=" * 70)
# Load tokenizer
tok = HybridTokenizer()
# Load pairs
with open('real_pairs.jsonl', 'r', encoding='utf-8') as f:
    pairs = [json.loads(line) for line in f]
print(f"\n📚 Dataset: {len(pairs)} pairs")
# Build model
model = RegexTinyModel(vocab_size=tok.vocab_size)
# Training
opt = torch.optim.AdamW(model.parameters(), lr=3e-4, weight_decay=0.01)
print("\n🚀 Training:")
for epoch in range(1, 21):
    model.train()
    total_loss = 0
    for pair in pairs:
        prompt = pair['prompt']
        completion = pair['completion']
        full = prompt + completion
        ids = tok.encode(full, max_len=256)
        if len(ids) < 10:
            continue
        x = torch.tensor([ids[:-1]], dtype=torch.long)
        y = torch.tensor([ids[1:]], dtype=torch.long)
        _, loss = model(x, y)
        opt.zero_grad()
        loss.backward()
        opt.step()
        total_loss += loss.item()
    avg_loss = total_loss / len(pairs)
    if epoch % 5 == 0:
        print(f"  Epoch {epoch}/20: loss={avg_loss:.4f}")
# Save model
torch.save(model.state_dict(), 'Regex-Tiny-v1.0.pt')
print(f"\n🏆 Model saved: Regex-Tiny-v1.0.pt")
# Honest evaluation
print("\n" + "=" * 70)
print("📊 Honest Evaluation (unseen data)")
print("=" * 70)
test_pairs = [
    ('# extract emails\nInput: contact@newdomain.org', '["contact@newdomain.org"]'),
    ('# extract URLs\nInput: See https://newsite.com', '["https://newsite.com"]'),
    ('# extract emails\nInput: Email support@company.co.uk', '["support@company.co.uk"]'),
    ('# extract URLs\nInput: Visit http://test.org/page', '["http://test.org/page"]'),
]
correct = 0
for prompt, expected in test_pairs:
    prompt_ids = tok.encode(prompt)
    idx = torch.tensor([prompt_ids])
    gen = model.generate(idx, max_new=50, temperature=0.5)
    output = tok.decode(gen[0].tolist())
    if expected in output:
        correct += 1
        print(f"  ✅ {prompt[:40]}...")
    else:
        print(f"  ❌ {prompt[:40]}...")
        print(f"     Expected: {expected}")
        print(f"     Got: {output[:80]}...")
accuracy = correct / len(test_pairs)
print(f"\n🎯 Test Accuracy: {accuracy:.0%} ({correct}/{len(test_pairs)})")
# Save results
with open('regex_tiny/evaluation_results.json', 'w') as f:
    json.dump({
        'accuracy': accuracy,
        'correct': correct,
        'total': len(test_pairs)
    }, f, indent=2)
print("📁 Results saved: regex_tiny/evaluation_results.json")
