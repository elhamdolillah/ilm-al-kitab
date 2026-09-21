"""Train MAL-Tiny on CPU - Flexible field names."""
import json, time, torch
from pathlib import Path
from model import MALTiny, MALTokenizer, count_params
def load_data(p):
    with open(p, encoding="utf-8") as f: return [json.loads(l) for l in f]
def get_source(ex):
    """Extract MAL source from example, trying multiple field names."""
    # Direct fields
    for f in ["mal_source", "source", "completion", "text", "code", "input"]:
        if f in ex and ex[f]:
            return str(ex[f]).strip()
    # Fallback: prompt field (strip instruction part)
    if "prompt" in ex:
        p = str(ex["prompt"])
        # Remove instruction prefix if present
        if "### Response:" in p:
            p = p.split("### Response:")[-1]
        elif "اكتب كود MAL" in p:
            parts = p.split("\n")
            p = parts[-1] if len(parts) > 1 else parts[0]
        return p.strip()
    return None
def build_pairs(data, tok, max_len=128):
    pairs = []
    skipped = 0
    for ex in data:
        src = get_source(ex)
        if not src or len(src) < 3:
            skipped += 1
            continue
        tokens = tok.encode(src, max_len=max_len)
        if len(tokens) < 5:
            skipped += 1
            continue
        x = torch.tensor(tokens[:-1], dtype=torch.long)
        y = torch.tensor(tokens[1:], dtype=torch.long)
        pairs.append((x, y))
    print(f"   Built {len(pairs)} pairs, skipped {skipped}")
    return pairs
def train(train_path="../finetune/dataset_train.jsonl", out="./mal-tiny-ckpt",
          epochs=20, lr=3e-4, batch=4, max_len=128):
    print("=" * 60)
    print("  MAL-Tiny Training (CPU Only)")
    print("=" * 60)
    tok = MALTokenizer()
    model = MALTiny(tok.vocab_size)
    print(f"📊 Params: {count_params(model):,}")
    data = load_data(train_path)
    print(f"📊 Loaded {len(data)} examples")
    print(f"   First example keys: {list(data[0].keys()) if data else 'empty'}")
    pairs = build_pairs(data, tok, max_len)
    print(f"📊 Training pairs: {len(pairs)}")
    if len(pairs) == 0:
        print("❌ No training pairs! Exiting.")
        return
    opt = torch.optim.AdamW(model.parameters(), lr=lr, weight_decay=0.01)
    Path(out).mkdir(exist_ok=True)
    start = time.time()
    for epoch in range(1, epochs+1):
        model.train()
        total = 0
        n = 0
        idx = torch.randperm(len(pairs))
        for i in range(0, len(pairs), batch):
            bi = idx[i:i+batch]
            if len(bi) == 0: continue
            bx = torch.stack([pairs[j][0] for j in bi])
            by = torch.stack([pairs[j][1] for j in bi])
            _, loss = model(bx, by)
            opt.zero_grad()
            loss.backward()
            torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            opt.step()
            total += loss.item()
            n += 1
        avg = total/max(n,1)
        print(f"Epoch {epoch:2d}/{epochs} | loss: {avg:.4f} | {(time.time()-start)/60:.1f}min")
        if epoch % 5 == 0 or epoch == epochs:
            torch.save({"state": model.state_dict(), "epoch": epoch, "loss": avg},
                      f"{out}/ckpt_ep{epoch}.pt")
    torch.save(model.state_dict(), f"{out}/final.pt")
    print(f"\n✅ Done in {(time.time()-start)/60:.1f} min")
if __name__ == "__main__":
    train()
