import torch
import torch.nn as nn
import sys
sys.path.insert(0, '.')
from hybrid_tokenizer import HybridTokenizer
class TransformerBlock(nn.Module):
    def __init__(self, embed_dim, num_heads):
        super().__init__()
        self.attention = nn.MultiheadAttention(embed_dim, num_heads, batch_first=True)
        self.ln1 = nn.LayerNorm(embed_dim)
        self.ln2 = nn.LayerNorm(embed_dim)
        self.ffn = nn.Sequential(
            nn.Linear(embed_dim, embed_dim * 4),
            nn.GELU(),
            nn.Linear(embed_dim * 4, embed_dim)
        )
    def forward(self, x):
        attn_out, _ = self.attention(x, x, x)
        x = self.ln1(x + attn_out)
        ffn_out = self.ffn(x)
        x = self.ln2(x + ffn_out)
        return x
class HybridClassifier(nn.Module):
    def __init__(self, vocab_size=122):
        super().__init__()
        self.embed_dim = 64
        self.num_heads = 4
        self.num_layers = 4
        self.max_seq_len = 256
        self.token_embedding = nn.Embedding(vocab_size, self.embed_dim)
        self.position_embedding = nn.Embedding(self.max_seq_len, self.embed_dim)
        self.layers = nn.ModuleList([
            TransformerBlock(self.embed_dim, self.num_heads)
            for _ in range(self.num_layers)
        ])
        self.ln_f = nn.LayerNorm(self.embed_dim)
        self.classifier = nn.Linear(self.embed_dim, 2)
        total_params = sum(p.numel() for p in self.parameters())
        print(f"🏗️  Hybrid Classifier: {total_params:,} params")
    def forward(self, idx, targets=None):
        B, T = idx.shape
        pos = torch.arange(0, T, dtype=torch.long, device=idx.device)
        tok_emb = self.token_embedding(idx)
        pos_emb = self.position_embedding(pos)
        x = tok_emb + pos_emb
        for layer in self.layers:
            x = layer(x)
        x = self.ln_f(x)
        x = x.mean(dim=1)  # Pooling
        logits = self.classifier(x)
        loss = None
        if targets is not None:
            loss = nn.functional.cross_entropy(logits, targets)
        return logits, loss
    def predict(self, idx):
        logits, _ = self.forward(idx)
        probs = nn.functional.softmax(logits, dim=-1)
        return probs.argmax(dim=-1)
training_data = [
    ("Contact: user@example.com", 1), ("Email: john.doe@company.org", 1),
    ("Multiple: a@b.com and c@d.org", 1), ("Complex: user.name+tag@subdomain.example.co.uk", 1),
    ("Short: x@y.com", 1), ("Long: very.long.email@subdomain.example.com", 1),
    ("Mixed: text1 a@b.com text2 c@d.org text3", 1), ("Valid: test@example.co.uk", 1),
    ("No emails here", 0), ("Invalid: not-an-email", 0),
    ("Just plain text", 0), ("Numbers only: 12345", 0),
    ("URL only: https://example.com", 0), ("Phone: +1-234-567-8900", 0),
    ("Visit https://example.com", 1), ("URL: http://test.org/page", 1),
    ("Multiple: https://a.com and http://b.org", 1), ("Complex: https://sub.domain.com:8080/path?q=1", 1),
    ("Short: http://x.com", 1), ("Long: https://www.example.com/very/long/path", 1),
    ("No URLs here", 0), ("Email only: user@example.com", 0), ("Plain text", 0),
]
if __name__ == '__main__':
    tok = HybridTokenizer()
    model = HybridClassifier(vocab_size=tok.vocab_size)
    opt = torch.optim.AdamW(model.parameters(), lr=1e-3)
    print("\n🚀 Training classifier (easy task)...")
    for epoch in range(1, 51):
        model.train()
        total_loss, correct = 0, 0
        for text, label in training_data:
            ids = tok.encode(text, max_len=256)
            x = torch.tensor([ids], dtype=torch.long)
            y = torch.tensor([label], dtype=torch.long)
            logits, loss = model(x, y)
            opt.zero_grad()
            loss.backward()
            opt.step()
            total_loss += loss.item()
            if logits.argmax(dim=-1) == y: correct += 1
        if epoch % 10 == 0:
            print(f"  Epoch {epoch}/50: loss={total_loss/len(training_data):.4f}, train_acc={correct/len(training_data):.0%}")
    torch.save(model.state_dict(), 'regex_tiny/Hybrid-Classifier-v1.0.pt')
    print(f"\n🏆 Classifier saved: regex_tiny/Hybrid-Classifier-v1.0.pt")
    print("\n" + "=" * 70)
    print("📊 Evaluation: Classification")
    print("=" * 70)
    test_data = [
        ("Contact: support@newdomain.org", 1), ("Email: admin@company.co.uk", 1),
        ("No email here", 0), ("Plain text only", 0),
        ("Visit https://newsite.com", 1), ("See http://test.org/page", 1),
        ("No URLs", 0), ("Just text", 0),
    ]
    correct = 0
    for text, expected in test_data:
        ids = tok.encode(text)
        x = torch.tensor([ids])
        pred = model.predict(x)
        if pred == expected: correct += 1
        print(f"  {'✅' if pred == expected else '❌'} {text[:40]}... (Pred: {pred.item()}, Exp: {expected})")
    print(f"\n🎯 Test Accuracy: {correct/len(test_data):.0%} ({correct}/{len(test_data)})")
