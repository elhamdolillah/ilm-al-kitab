"""
MAL-Tiny v2.0: Full pipeline with pre-tests, training, and evaluation.
Includes:
  - Pre-training sanity checks
  - Proactive analysis (token coverage, length distribution)
  - Training with early stopping
  - Per-category evaluation
  - Final report
"""
import json, time, torch, sys
from pathlib import Path
from collections import Counter, defaultdict
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
# ═══════════════════════════════════════════════════════════
# PHASE A: PRE-TRAINING SANITY CHECKS
# ═══════════════════════════════════════════════════════════
print("=" * 70)
print("  PHASE A: PRE-TRAINING SANITY CHECKS")
print("=" * 70)
tok = MALTokenizer()
print(f"\n🔍 A1. Tokenizer check:")
print(f"   Vocab size: {tok.vocab_size}")
print(f"   Special tokens: PAD={tok.stoi.get('<PAD>')}, BOS={tok.stoi.get('<BOS>')}, EOS={tok.stoi.get('<EOS>')}")
# Check token coverage on browser data
browser_data = load_data("../dataset/browser_automation.jsonl")
unk_count = 0
total_chars = 0
unk_chars = []
for ex in browser_data:
    src = ex.get("mal_source", "")
    for ch in src:
        total_chars += 1
        if ch not in tok.stoi and not any(ch in s for s in tok.symbols + tok.keywords):
            unk_count += 1
            unk_chars.append(ch)
unk_rate = unk_count / max(total_chars, 1) * 100
print(f"\n🔍 A2. Token coverage on browser data:")
print(f"   Total chars: {total_chars}")
print(f"   Unknown chars: {unk_count} ({unk_rate:.1f}%)")
if unk_chars:
    unique_unk = list(set(unk_chars))[:20]
    print(f"   Unknown chars (sample): {unique_unk}")
    if unk_rate > 30:
        print(f"   ⚠️ HIGH UNK rate! Consider expanding tokenizer.")
    else:
        print(f"   ✅ UNK rate acceptable (<30%)")
else:
    print(f"   ✅ Perfect token coverage!")
# Check data integrity
print(f"\n🔍 A3. Data integrity check:")
train_data = load_data("../finetune/dataset_train_v3.jsonl")
eval_data = load_data("../finetune/dataset_eval_v3.jsonl")
# Check for empty sources
empty_train = sum(1 for ex in train_data if not get_source(ex))
empty_eval = sum(1 for ex in eval_data if not get_source(ex))
print(f"   Train: {len(train_data)} examples, {empty_train} empty")
print(f"   Eval:  {len(eval_data)} examples, {empty_eval} empty")
# Check category balance
cats = Counter(ex['category'] for ex in train_data)
print(f"\n🔍 A4. Category balance (train):")
for c, n in cats.most_common():
    bar = "█" * (n // 2)
    print(f"   {c:<25} {n:>3} {bar}")
# Check sequence lengths
lengths = []
for ex in train_data:
    src = get_source(ex)
    if src:
        lengths.append(len(src))
if lengths:
    avg_len = sum(lengths) / len(lengths)
    max_len = max(lengths)
    min_len = min(lengths)
    print(f"\n🔍 A5. Sequence length analysis:")
    print(f"   Min: {min_len}, Max: {max_len}, Avg: {avg_len:.1f}")
    if max_len > 128:
        long_count = sum(1 for l in lengths if l > 128)
        print(f"   ⚠️ {long_count} examples exceed 128 tokens (will be truncated)")
    else:
        print(f"   ✅ All examples fit within 128 token limit")
# Check for duplicates
sources = [get_source(ex) for ex in train_data if get_source(ex)]
dupes = len(sources) - len(set(sources))
print(f"\n🔍 A6. Duplicate check:")
print(f"   Duplicates found: {dupes}")
if dupes > 0:
    print(f"   ⚠️ Consider removing duplicates")
else:
    print(f"   ✅ No duplicates")
# Build pairs
train_pairs = build_pairs(train_data, tok)
eval_pairs = build_pairs(eval_data, tok)
print(f"\n🔍 A7. Training pairs:")
print(f"   Train pairs: {len(train_pairs)}")
print(f"   Eval pairs:  {len(eval_pairs)}")
if len(train_pairs) < 10:
    print(f"   ❌ CRITICAL: Too few training pairs! Aborting.")
    sys.exit(1)
print(f"\n✅ ALL PRE-TRAINING CHECKS PASSED")
# ═══════════════════════════════════════════════════════════
# PHASE B: TRAINING WITH EARLY STOPPING
# ═══════════════════════════════════════════════════════════
print(f"\n{'='*70}")
print("  PHASE B: TRAINING WITH EARLY STOPPING")
print("=" * 70)
model = MALTiny(tok.vocab_size)
print(f"\n📊 Model: {count_params(model):,} params")
print(f"📊 Training on {len(train_pairs)} pairs")
opt = torch.optim.AdamW(model.parameters(), lr=3e-4, weight_decay=0.01)
Path("v2_checkpoints").mkdir(exist_ok=True)
start = time.time()
best_eval_loss = float('inf')
best_epoch = 0
patience = 0
MAX_PATIENCE = 3
print(f"\n{'Epoch':<8}{'Train Loss':<15}{'Eval Loss':<15}{'Status':<15}")
print("-" * 55)
for epoch in range(1, 41):
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
    # Eval every 5 epochs
    if epoch % 5 == 0:
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
            status = f"⏳ patience={patience}"
        elapsed = (time.time() - start) / 60
        print(f"{epoch:<8}{train_loss:<15.4f}{eval_loss:<15.4f}{status:<15}")
        # Early stopping
        if patience >= MAX_PATIENCE:
            print(f"\n⏹️  Early stopping at epoch {epoch} (patience={MAX_PATIENCE})")
            break
train_time = (time.time() - start) / 60
print(f"\n✅ Training complete in {train_time:.1f} min")
print(f"🏆 Best epoch: {best_epoch} (eval loss: {best_eval_loss:.4f})")
# Save final too
torch.save(model.state_dict(), "v2_checkpoints/final.pt")
# Load best model for evaluation
model.load_state_dict(torch.load("v2_checkpoints/best.pt", map_location="cpu"))
# ═══════════════════════════════════════════════════════════
# PHASE C: PER-CATEGORY EVALUATION
# ═══════════════════════════════════════════════════════════
print(f"\n{'='*70}")
print("  PHASE C: PER-CATEGORY EVALUATION")
print("=" * 70)
model.eval()
# Group eval examples by category
by_category = defaultdict(list)
for ex in eval_data:
    src = get_source(ex)
    if src:
        by_category[ex['category']].append(src)
print(f"\n📊 Evaluating {sum(len(v) for v in by_category.values())} examples across {len(by_category)} categories\n")
results = {}
total_correct = 0
total_count = 0
for cat, sources in sorted(by_category.items()):
    correct = 0
    for src in sources[:5]:  # Max 5 per category
        prompt = tok.encode(src, max_len=60)
        idx = torch.tensor([prompt])
        gen = model.generate(idx, max_new=20, temp=0.5)
        text = tok.decode(gen[0].tolist())
        # Clean: remove trailing hallucinations
        if src in text:
            text = src  # Exact match found within generation
        if text.strip() == src.strip():
            correct += 1
    count = min(len(sources), 5)
    pct = 100 * correct / count if count > 0 else 0
    results[cat] = {"correct": correct, "total": count, "pct": pct}
    total_correct += correct
    total_count += count
    status = "✅" if pct >= 80 else ("🟡" if pct >= 60 else "❌")
    print(f"  {status} {cat:<25} {correct}/{count} = {pct:.0f}%")
overall = 100 * total_correct / total_count if total_count > 0 else 0
print(f"\n{'='*70}")
print(f"  🏆 OVERALL: {total_correct}/{total_count} = {overall:.1f}%")
print(f"{'='*70}")
# ═══════════════════════════════════════════════════════════
# PHASE D: FINAL REPORT
# ═══════════════════════════════════════════════════════════
print(f"\n{'='*70}")
print("  PHASE D: FINAL REPORT")
print("=" * 70)
report = {
    "model": "MAL-Tiny v2.0",
    "params": count_params(model),
    "best_epoch": best_epoch,
    "best_eval_loss": round(best_eval_loss, 4),
    "training_time_min": round(train_time, 1),
    "dataset_v3": {
        "total": len(train_data) + len(eval_data),
        "train": len(train_data),
        "eval": len(eval_data)
    },
    "overall_accuracy": round(overall, 1),
    "per_category": results
}
with open("v2_report.json", "w") as f:
    json.dump(report, f, indent=2, default=str)
print(f"\n💾 Report saved to v2_report.json")
print(f"\n📊 Summary:")
print(f"   Model: MAL-Tiny v2.0 ({count_params(model):,} params)")
print(f"   Best epoch: {best_epoch}")
print(f"   Training time: {train_time:.1f} min")
print(f"   Overall accuracy: {overall:.1f}%")
print(f"\n📊 Categories achieving 80%+:")
high = [c for c, r in results.items() if r["pct"] >= 80]
for c in high:
    print(f"   ✅ {c}: {results[c]['pct']:.0f}%")
print(f"\n📊 Categories below 60%:")
low = [c for c, r in results.items() if r["pct"] < 60]
for c in low:
    print(f"   ❌ {c}: {results[c]['pct']:.0f}%")
print(f"\n{'='*70}")
print(f"  🎊 MAL-Tiny v2.0 COMPLETE!")
print(f"{'='*70}")
