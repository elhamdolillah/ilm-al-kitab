"""Resume training from epoch 21."""
import json, time, torch, sys
from pathlib import Path
from collections import defaultdict
from model import MALTiny, MALTokenizer, count_params
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
print("=" * 70)
print("  RESUMING TRAINING FROM EPOCH 21")
print("=" * 70)
tok = MALTokenizer()
model = MALTiny(tok.vocab_size)
# Load checkpoint from epoch 20
ckpt_path = Path("v2_checkpoints/best.pt")
if ckpt_path.exists():
    model.load_state_dict(torch.load(ckpt_path, map_location="cpu"))
    print(f"✅ Loaded checkpoint from {ckpt_path}")
    START_EPOCH = 21
    best_eval_loss = 1.4244  # from epoch 20
else:
    print(f"⚠️  No checkpoint, starting from epoch 1")
    START_EPOCH = 1
    best_eval_loss = float('inf')
train_data = load_data("../finetune/dataset_train_v3.jsonl")
eval_data = load_data("../finetune/dataset_eval_v3.jsonl")
train_pairs = build_pairs(train_data, tok)
eval_pairs = build_pairs(eval_data, tok)
print(f"📊 Model: {count_params(model):,} params")
print(f"📊 Train: {len(train_pairs)} | Eval: {len(eval_pairs)}")
opt = torch.optim.AdamW(model.parameters(), lr=3e-4, weight_decay=0.01)
Path("v2_checkpoints").mkdir(exist_ok=True)
start = time.time()
best_epoch = 20 if START_EPOCH > 1 else 0
patience = 0
MAX_PATIENCE = 3
print(f"\n{'Epoch':<8}{'Train':<12}{'Eval':<12}{'Status':<15}")
print("-" * 50)
for epoch in range(START_EPOCH, 41):
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
    if epoch % 5 == 0 or epoch == 40:
        model.eval()
        eval_total = 0
        eval_n = 0
        with torch.no_grad():
            for x, y in eval_pairs:
                _, loss = model(x.unsqueeze(0), y.unsqueeze(0))
                eval_total += loss.item()
                eval_n += 1
        eval_loss = eval_total / max(eval_n, 1)
        status = ""
        if eval_loss < best_eval_loss:
            best_eval_loss = eval_loss
            best_epoch = epoch
            patience = 0
            torch.save(model.state_dict(), "v2_checkpoints/best.pt")
            status = "🏆 BEST"
        else:
            patience += 1
            status = f"⏳ pat={patience}"
        print(f"{epoch:<8}{train_loss:<12.4f}{eval_loss:<12.4f}{status:<15}")
        if patience >= MAX_PATIENCE:
            print(f"\n⏹️  Early stopping at epoch {epoch}")
            break
train_time = (time.time() - start) / 60
print(f"\n✅ Training complete in {train_time:.1f} min")
print(f"🏆 Best epoch: {best_epoch} (eval loss: {best_eval_loss:.4f})")
# Save final
torch.save(model.state_dict(), "v2_checkpoints/final.pt")
model.load_state_dict(torch.load("v2_checkpoints/best.pt", map_location="cpu"))
# ═══════════════════════════════════════════════════════════
# PHASE C: PER-CATEGORY EVALUATION
# ═══════════════════════════════════════════════════════════
print(f"\n{'='*70}")
print("  PER-CATEGORY EVALUATION")
print("=" * 70)
model.eval()
by_category = defaultdict(list)
for ex in eval_data:
    src = get_source(ex)
    if src:
        by_category[ex['category']].append(src)
print(f"\n📊 {sum(len(v) for v in by_category.values())} examples across {len(by_category)} categories\n")
results = {}
total_correct = 0
total_count = 0
for cat, sources in sorted(by_category.items()):
    correct = 0
    for src in sources[:5]:
        prompt = tok.encode(src, max_len=60)
        idx = torch.tensor([prompt])
        gen = model.generate(idx, max_new=20, temp=0.5)
        text = tok.decode(gen[0].tolist())
        if src in text:
            text = src
        if text.strip() == src.strip():
            correct += 1
    count = min(len(sources), 5)
    pct = 100 * correct / count if count > 0 else 0
    results[cat] = {"correct": correct, "total": count, "pct": round(pct, 1)}
    total_correct += correct
    total_count += count
    status = "✅" if pct >= 80 else ("🟡" if pct >= 60 else "❌")
    print(f"  {status} {cat:<25} {correct}/{count} = {pct:.0f}%")
overall = 100 * total_correct / total_count if total_count > 0 else 0
print(f"\n{'='*70}")
print(f"  🏆 OVERALL: {total_correct}/{total_count} = {overall:.1f}%")
print(f"{'='*70}")
# Save report
report = {
    "model": "MAL-Tiny v2.0",
    "params": count_params(model),
    "best_epoch": best_epoch,
    "best_eval_loss": round(best_eval_loss, 4),
    "training_time_min": round(train_time, 1),
    "dataset_v3": {"total": len(train_data) + len(eval_data), "train": len(train_data), "eval": len(eval_data)},
    "overall_accuracy": round(overall, 1),
    "per_category": results
}
with open("v2_report.json", "w") as f:
    json.dump(report, f, indent=2, default=str)
print(f"\n💾 Saved: v2_report.json")
print(f"\n📊 Categories 80%+:")
for c, r in sorted(results.items(), key=lambda x: -x[1]['pct']):
    if r['pct'] >= 80:
        print(f"   ✅ {c}: {r['pct']:.0f}%")
print(f"\n🎊 MAL-Tiny v2.0 COMPLETE!")
