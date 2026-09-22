import torch
import torch.nn as nn
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
class RegexTinyModel(nn.Module):
    def __init__(self, vocab_size=122):
        super().__init__()
        self.embed_dim = 192
        self.num_heads = 8
        self.num_layers = 8
        self.max_seq_len = 256
        self.token_embedding = nn.Embedding(vocab_size, self.embed_dim)
        self.position_embedding = nn.Embedding(self.max_seq_len, self.embed_dim)
        self.layers = nn.ModuleList([
            TransformerBlock(self.embed_dim, self.num_heads)
            for _ in range(self.num_layers)
        ])
        self.ln_f = nn.LayerNorm(self.embed_dim)
        self.head = nn.Linear(self.embed_dim, vocab_size, bias=False)
        total_params = sum(p.numel() for p in self.parameters())
        print(f"🏗️  Regex-Tiny: {total_params:,} params")
    def forward(self, idx, targets=None):
        B, T = idx.shape
        pos = torch.arange(0, T, dtype=torch.long, device=idx.device)
        tok_emb = self.token_embedding(idx)
        pos_emb = self.position_embedding(pos)
        x = tok_emb + pos_emb
        for layer in self.layers:
            x = layer(x)
        x = self.ln_f(x)
        logits = self.head(x)
        loss = None
        if targets is not None:
            loss = nn.functional.cross_entropy(logits.view(-1, logits.size(-1)), targets.view(-1))
        return logits, loss
    def generate(self, idx, max_new=50, temperature=0.7):
        for _ in range(max_new):
            idx_cond = idx if idx.size(1) <= self.max_seq_len else idx[:, -self.max_seq_len:]
            logits, _ = self.forward(idx_cond)
            logits = logits[:, -1, :] / temperature
            probs = nn.functional.softmax(logits, dim=-1)
            idx_next = torch.multinomial(probs, num_samples=1)
            idx = torch.cat((idx, idx_next), dim=1)
        return idx
if __name__ == '__main__':
    model = RegexTinyModel(vocab_size=122)
    print(f"✅ Model created: {sum(p.numel() for p in model.parameters()):,} params")
