"""Fine-tune v2: char-level tokenizer for Python code."""
import json, torch, time, sys, string
from pathlib import Path
sys.path.insert(0, ".")
WORK_DIR = Path("./real_evolution_work")
class CharTokenizer:
    """Simple char-level tokenizer for code."""
    def __init__(self):
        chars = list(string.printable) + list("ابتثجحخدذرزسشصضطظعغفقكلمنهويىءةؤئإأآ")
        self.stoi = {c: i+3 for i, c in enumerate(chars)}
        self.stoi['<PAD>'] = 0
        self.stoi['<BOS>'] = 1
        self.stoi['<EOS>'] = 2
        self.itos = {i: c for c, i in self.stoi.items()}
        self.vocab_size = len(self.stoi)
    def encode(self, text, max_len=256):
        tokens = [self.stoi['<BOS>']]
        for c in text:
            if c in self.stoi:
                tokens.append(self.stoi[c])
            else:
                tokens.append(self.stoi.get('?', self.stoi['<PAD>']))
        tokens.append(self.stoi['<EOS>'])
        return tokens[:max_len]
    def decode(self, tokens):
        return ''.join(self.itos.get(t, '') for t in tokens if t > 2)
def main():
    print("=" * 70)
    print("Fine-tune v2: Char-level tokenizer")
    print("=" * 70)
    lib_path = WORK_DIR / "library.json"
    if not lib_path.exists():
        print(f"ERROR: {lib_path} not found")
        return
    with open(lib_path) as f:
        library = json.load(f)
    best = {}
    for r in library:
        if r["language"] != "python": continue
        t = r["task_id"]
        if t not in best or r["ops_per_sec"] > best[t]["ops_per_sec"]:
            best[t] = r
    descs = {
        "extract_email": "extract emails",
        "extract_url": "extract URLs",
        "extract_attrs": "extract HTML attrs",
        "sum_numbers": "sum numbers",
        "count_words": "count words"
    }
    examples = []
    for task_id, rule in best.items():
        desc = descs.get(task_id, task_id)
        examples.append({
            "prompt": f"# {desc}\n",
            "completion": rule["code"],
            "task_id": task_id
        })
    print(f"Dataset: {len(examples)} examples")
    tok = CharTokenizer()
    print(f"Char tokenizer: {tok.vocab_size} chars")
    # Build training pairs
    pairs = []
    for ex in examples:
        full = ex["prompt"] + ex["completion"]
        tokens = tok.encode(full, max_len=400)
        if len(tokens) >= 10:
            x = torch.tensor(tokens[:-1], dtype=torch.long)
            y = torch.tensor(tokens[1:], dtype=torch.long)
            pairs.append((x, y))
            print(f"  - {ex['task_id']}: {len(tokens)} tokens")
    if len(pairs) < 3:
        print("Not enough pairs")
        return
    # Simple char-level model (larger capacity)
    import torch.nn as nn
    class CharModel(nn.Module):
        def __init__(self, vocab_size, dim=128, layers=4):
            super().__init__()
            self.tok_emb = nn.Embedding(vocab_size, dim)
            self.pos_emb = nn.Embedding(512, dim)
            layer = nn.TransformerEncoderLayer(d_model=dim, nhead=4, dim_feedforward=dim*4, batch_first=True)
            self.encoder = nn.TransformerEncoder(layer, num_layers=layers)
            self.head = nn.Linear(dim, vocab_size)
        def forward(self, idx, targets=None):
            B, T = idx.shape
            pos = torch.arange(T, device=idx.device).unsqueeze(0).expand(B, T)
            x = self.tok_emb(idx) + self.pos_emb(pos)
            x = self.encoder(x)
            logits = self.head(x)
            loss = None
            if targets is not None:
                loss = nn.functional.cross_entropy(logits.view(-1, logits.size(-1)), targets.view(-1))
            return logits, loss
        def generate(self, idx, max_new=100, temp=0.8):
            self.eval()
            for _ in range(max_new):
                logits, _ = self(idx)
                logits = logits[:, -1, :] / temp
                probs = torch.softmax(logits, dim=-1)
                next_tok = torch.multinomial(probs, 1)
                idx = torch.cat([idx, next_tok], dim=1)
                if idx.size(1) >= 500: break
            return idx
    model = CharModel(tok.vocab_size, dim=128, layers=4)
    params = sum(p.numel() for p in model.parameters())
    print(f"CharModel: {params:,} params")
    # Load MAL-Tiny weights where possible (skip mismatches)
    base_path = Path("MAL-Tiny-Baseline-80pct.pt")
    if base_path.exists():
        try:
            state = torch.load(base_path, map_location="cpu")
            # Filter compatible weights
            my_state = model.state_dict()
            compatible = {k: v for k, v in state.items() if k in my_state and v.shape == my_state[k].shape}
            model.load_state_dict(compatible, strict=False)
            print(f"Loaded {len(compatible)}/{len(my_state)} compatible weights")
        except Exception as e:
            print(f"Could not load base: {e}")
    opt = torch.optim.AdamW(model.parameters(), lr=3e-4, weight_decay=0.01)
    start = time.time()
    print("\nTraining:")
    for epoch in range(1, 21):
        model.train()
        total = 0
        n = 0
        for _ in range(10):
            for x, y in pairs:
                _, loss = model(x.unsqueeze(0), y.unsqueeze(0))
                opt.zero_grad()
                loss.backward()
                opt.step()
                total += loss.item()
                n += 1
        avg = total / max(n, 1)
        if epoch % 5 == 0 or epoch == 1:
            print(f"  Epoch {epoch}/20: loss={avg:.4f}")
    elapsed = time.time() - start
    out_path = Path("MAL-Tiny-CharLevel-Code.pt")
    torch.save(model.state_dict(), out_path)
    print(f"\nFine-tuned in {elapsed:.1f}s")
    print(f"Saved: {out_path} ({out_path.stat().st_size / 1024:.1f} KB)")
    # Test generation
    print("\n🎯 Testing generation:")
    for task_id, desc in [("extract_email", "extract emails"), ("sum_numbers", "sum numbers")]:
        prompt = tok.encode(f"# {desc}\n", max_len=100)
        idx = torch.tensor([prompt])
        gen = model.generate(idx, max_new=50, temp=0.7)
        text = tok.decode(gen[0].tolist())
        print(f"\n  Task: {task_id}")
        print(f"  Prompt: # {desc}")
        print(f"  Generated: {text[:200]}...")
if __name__ == "__main__":
    main()
