"""Find optimal training via eval LOSS (not generation)."""
import json, torch, time
from pathlib import Path
from model import MALTiny, MALTokenizer
def get_source(ex):
    for f in ["mal_source","source","completion","text","code"]:
        if f in ex and ex[f]: return str(ex[f]).strip()
    if "prompt" in ex:
        p = str(ex["prompt"])
        if "### Response:" in p: p = p.split("### Response:")[-1]
        elif "اكتب كود MAL" in p: p = p.split("\n")[-1]
        return p.strip()
    return None
def load_data(p):
    with open(p, encoding="utf-8") as f: return [json.loads(l) for l in f]
def build_pairs(data, tok, max_len=128):
    pairs = []
    for ex in data:
        src = get_source(ex)
        if not src or len(src) < 3: continue
        tokens = tok.encode(src, max_len=max_len)
        if len(tokens) < 5: continue
        x = torch.tensor(tokens[:-1], dtype=torch.long)
        y = torch.tensor(tokens[1:], dtype=torch.long)
        pairs.append((x, y))
    return pairs
def eval_loss(model, pairs):
    """Compute loss on eval set (lower = better)."""
    model.eval()
    total = 0
    count = 0
    with torch.no_grad():
        for x, y in pairs:
            _, loss = model(x.unsqueeze(0), y.unsqueeze(0))
            total += loss.item()
            count += 1
    return total / max(count, 1)
print("🎯 Finding Sweet Spot v2 (eval-loss based)")
print("=" * 60)
tok = MALTokenizer()
model = MALTiny(tok.vocab_size)
train_data = load_data("../finetune/dataset_train.jsonl")
eval_data = load_data("../finetune/dataset_eval.jsonl")
train_pairs = build_pairs(train_data, tok)
eval_pairs = build_pairs(eval_data, tok)
print(f"📊 Training: {len(train_pairs)} | Eval: {len(eval_pairs)}")
opt = torch.optim.AdamW(model.parameters(), lr=3e-4, weight_decay=0.01)
Path("sweet_spot_v2").mkdir(exist_ok=True)
start = time.time()
best_eval_loss = float('inf')
best_epoch = 0
print(f"\n{'Epoch':<8}{'Train Loss':<15}{'Eval Loss':<15}{'Status':<10}")
print("-" * 50)
for epoch in range(1, 51):
    model.train()
    total = 0
    n = 0
    idx_perm = torch.randperm(len(train_pairs))
    for i in range(0, len(train_pairs), 4):
        bi = idx_perm[i:i+4]
        bx = torch.stack([train_pairs[j][0] for j in bi])
        by = torch.stack([train_pairs[j][1] for j in bi])
        _, loss = model(bx, by)
        opt.zero_grad()
        loss.backward()
        torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        opt.step()
        total += loss.item()
        n += 1
    train_loss = total / max(n, 1)
    # Evaluate every 5 epochs
    if epoch % 5 == 0:
        e_loss = eval_loss(model, eval_pairs)
        status = ""
        if e_loss < best_eval_loss:
            best_eval_loss = e_loss
            best_epoch = epoch
            torch.save(model.state_dict(), "sweet_spot_v2/best.pt")
            status = "🏆 BEST"
        print(f"{epoch:<8}{train_loss:<15.4f}{e_loss:<15.4f}{status:<10}")
print(f"\n{'='*60}")
print(f"🥇 Best epoch: {best_epoch}")
print(f"📊 Best eval loss: {best_eval_loss:.4f}")
print(f"⏱️  Total time: {(time.time()-start)/60:.1f} min")
print(f"💾 Best model: sweet_spot_v2/best.pt")
