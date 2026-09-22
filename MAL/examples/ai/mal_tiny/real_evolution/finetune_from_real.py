"""Fine-tune MAL-Tiny on verified examples - run from mal_tiny/."""
import json, torch, time, sys
from pathlib import Path
# Import from current dir (mal_tiny)
sys.path.insert(0, ".")
from model import MALTiny, MALTokenizer, count_params
WORK_DIR = Path("./real_evolution_work")
def main():
    print("=" * 70)
    print("Fine-tune MAL-Tiny from verified library")
    print("=" * 70)
    lib_path = WORK_DIR / "library.json"
    if not lib_path.exists():
        print(f"ERROR: {lib_path} not found")
        return
    with open(lib_path) as f:
        library = json.load(f)
    # Best Python rule per task
    best = {}
    for r in library:
        if r["language"] != "python": continue
        t = r["task_id"]
        if t not in best or r["ops_per_sec"] > best[t]["ops_per_sec"]:
            best[t] = r
    descs = {
        "extract_email": "استخرج عناوين البريد الإلكتروني من النص",
        "extract_url": "استخرج الروابط من النص",
        "extract_attrs": "استخرج خصائص عنصر HTML",
        "sum_numbers": "اجمع الأرقام في النص",
        "count_words": "احسب عدد الكلمات في النص"
    }
    examples = []
    for task_id, rule in best.items():
        desc = descs.get(task_id, task_id)
        examples.append({
            "prompt": f"اكتب Python regex للمهمة: {desc}\n",
            "completion": rule["code"],
            "task_id": task_id,
            "ops": rule["ops_per_sec"]
        })
    dataset_path = WORK_DIR / "finetune_from_real.jsonl"
    with open(dataset_path, "w", encoding="utf-8") as f:
        for ex in examples:
            f.write(json.dumps(ex, ensure_ascii=False) + "\n")
    print(f"Dataset: {len(examples)} examples")
    for ex in examples:
        print(f"  - {ex['task_id']}: {ex['ops']:.0f} ops/s")
    print("\nFine-tuning MAL-Tiny...")
    tok = MALTokenizer()
    model = MALTiny(tok.vocab_size)
    base_path = Path("MAL-Tiny-Baseline-80pct.pt")
    if base_path.exists():
        state = torch.load(base_path, map_location="cpu")
        if isinstance(state, dict) and all(isinstance(k, str) for k in state.keys()):
            model.load_state_dict(state)
        print(f"Loaded base: {count_params(model):,} params")
    # Build training pairs
    pairs = []
    for ex in examples:
        full = ex["prompt"] + ex["completion"]
        tokens = tok.encode(full, max_len=256)
        if len(tokens) >= 10:
            x = torch.tensor(tokens[:-1], dtype=torch.long)
            y = torch.tensor(tokens[1:], dtype=torch.long)
            pairs.append((x, y))
    print(f"Training pairs: {len(pairs)}")
    if len(pairs) < 3:
        print("Not enough pairs")
        return
    opt = torch.optim.AdamW(model.parameters(), lr=1e-4, weight_decay=0.01)
    start = time.time()
    for epoch in range(1, 16):
        model.train()
        total = 0
        n = 0
        for _ in range(5):
            for x, y in pairs:
                _, loss = model(x.unsqueeze(0), y.unsqueeze(0))
                opt.zero_grad()
                loss.backward()
                opt.step()
                total += loss.item()
                n += 1
        avg = total / max(n, 1)
        print(f"  Epoch {epoch}/15: loss={avg:.4f}")
    elapsed = time.time() - start
    out_path = Path("MAL-Tiny-FineTuned-Regex.pt")
    torch.save(model.state_dict(), out_path)
    print(f"\nFine-tuned in {elapsed:.1f}s")
    print(f"Saved: {out_path} ({out_path.stat().st_size / 1024:.1f} KB)")
if __name__ == "__main__":
    main()
