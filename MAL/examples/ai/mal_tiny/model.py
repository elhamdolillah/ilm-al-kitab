"""MAL-Tiny: 5M parameter Transformer for MAL language."""
import torch
import torch.nn as nn
import torch.nn.functional as F
import math
class MALTokenizer:
    def __init__(self):
        self.chars = list("0123456789+-*/^%()[]|.,:;= _xyzabc\n")
        self.symbols = ["≔","≡","·","∧","∨","¬","<","≤",">","≥","≠","==","⇒","∀","∃","∈","μ","λ","⊸","⊕","⋄"]
        self.keywords = ["true","false","match","with","sqrt","abs","min","max","floor","ceil","round","power"]
        self.vocab = ["<PAD>","<BOS>","<EOS>","<UNK>"] + self.chars + [f"_{s}" for s in self.symbols] + [f"_{k}" for k in self.keywords]
        self.stoi = {c:i for i,c in enumerate(self.vocab)}
        self.itos = {i:c for i,c in enumerate(self.vocab)}
        self.vocab_size = len(self.vocab)
    def encode(self, text, max_len=128):
        tokens = [self.stoi["<BOS>"]]
        i = 0
        specials = self.symbols + self.keywords
        specials.sort(key=len, reverse=True)
        while i < len(text):
            matched = False
            for s in specials:
                if text[i:i+len(s)] == s:
                    tokens.append(self.stoi[f"_{s}"])
                    i += len(s)
                    matched = True
                    break
            if not matched:
                tokens.append(self.stoi.get(text[i], self.stoi["<UNK>"]))
                i += 1
        tokens.append(self.stoi["<EOS>"])
        if len(tokens) < max_len:
            tokens += [self.stoi["<PAD>"]] * (max_len - len(tokens))
        return tokens[:max_len]
    def decode(self, tokens):
        chars = []
        for t in tokens:
            if t in (self.stoi["<BOS>"], self.stoi["<EOS>"], self.stoi["<PAD>"]): continue
            s = self.itos.get(t, "")
            chars.append(s[1:] if s.startswith("_") else s)
        return "".join(chars)
class CausalAttention(nn.Module):
    def __init__(self, n_embd, n_head):
        super().__init__()
        self.n_head = n_head
        self.head_dim = n_embd // n_head
        self.c_attn = nn.Linear(n_embd, 3 * n_embd)
        self.c_proj = nn.Linear(n_embd, n_embd)
    def forward(self, x):
        B, T, C = x.size()
        q, k, v = self.c_attn(x).split(C, dim=2)
        q = q.view(B, T, self.n_head, self.head_dim).transpose(1,2)
        k = k.view(B, T, self.n_head, self.head_dim).transpose(1,2)
        v = v.view(B, T, self.n_head, self.head_dim).transpose(1,2)
        att = (q @ k.transpose(-2,-1)) / math.sqrt(self.head_dim)
        mask = torch.triu(torch.ones(T,T,device=x.device), diagonal=1).bool()
        att = att.masked_fill(mask, float('-inf'))
        att = F.softmax(att, dim=-1)
        y = (att @ v).transpose(1,2).contiguous().view(B, T, C)
        return self.c_proj(y)
class Block(nn.Module):
    def __init__(self, n_embd, n_head):
        super().__init__()
        self.ln1 = nn.LayerNorm(n_embd)
        self.attn = CausalAttention(n_embd, n_head)
        self.ln2 = nn.LayerNorm(n_embd)
        self.mlp = nn.Sequential(
            nn.Linear(n_embd, 4*n_embd), nn.GELU(), nn.Linear(4*n_embd, n_embd))
    def forward(self, x):
        x = x + self.attn(self.ln1(x))
        return x + self.mlp(self.ln2(x))
class MALTiny(nn.Module):
    def __init__(self, vocab_size, n_layer=4, n_head=4, n_embd=64, max_len=128):
        super().__init__()
        self.tok_emb = nn.Embedding(vocab_size, n_embd)
        self.pos_emb = nn.Embedding(max_len, n_embd)
        self.blocks = nn.ModuleList([Block(n_embd, n_head) for _ in range(n_layer)])
        self.ln_f = nn.LayerNorm(n_embd)
        self.head = nn.Linear(n_embd, vocab_size, bias=False)
        self.tok_emb.weight = self.head.weight
        self.apply(self._init)
    def _init(self, m):
        if isinstance(m, (nn.Linear, nn.Embedding)):
            nn.init.normal_(m.weight, std=0.02)
            if hasattr(m, 'bias') and m.bias is not None: nn.init.zeros_(m.bias)
    def forward(self, idx, targets=None):
        B, T = idx.size()
        pos = torch.arange(T, device=idx.device)
        x = self.tok_emb(idx) + self.pos_emb(pos)
        for b in self.blocks: x = b(x)
        logits = self.head(self.ln_f(x))
        loss = None
        if targets is not None:
            loss = F.cross_entropy(logits.view(-1, logits.size(-1)), targets.view(-1), ignore_index=0)
        return logits, loss
    @torch.no_grad()
    def generate(self, idx, max_new=64, temp=0.8):
        self.eval()
        for _ in range(max_new):
            logits, _ = self(idx)
            probs = F.softmax(logits[:,-1,:]/temp, dim=-1)
            next_t = torch.multinomial(probs, 1)
            idx = torch.cat([idx, next_t], dim=1)
            if next_t.item() == 2: break
        return idx
def count_params(m): return sum(p.numel() for p in m.parameters() if p.requires_grad)
if __name__ == "__main__":
    tok = MALTokenizer()
    m = MALTiny(tok.vocab_size)
    print(f"✅ MAL-Tiny: {count_params(m):,} params, ~{count_params(m)*4/1024/1024:.1f} MB")
