"""Evaluate all trained models."""
import json, subprocess, tempfile, torch, sys
from pathlib import Path
from model import MALTiny, MALTokenizer
MALC = "/root/ilm-al-kitab/MAL/tools/malc-native/target/release/malc-native"
def get_source(ex):
    for f in ["mal_source","source","completion","text","code"]:
        if f in ex and ex[f]: return str(ex[f]).strip()
    if "prompt" in ex:
        p = str(ex["prompt"])
        if "### Response:" in p: p = p.split("### Response:")[-1]
        elif "اكتب كود MAL" in p: p = p.split("\n")[-1]
        return p.strip()
    return None
def evaluate_model(name, model_path, max_ex=10):
    print(f"\n{'='*70}")
    print(f"🔬 Evaluating: {name}")
    print(f"{'='*70}")
    if not Path(model_path).exists():
        print(f"   ⚠️ Model not found: {model_path}")
        return None
    tok = MALTokenizer()
    model = MALTiny(tok.vocab_size)
    state = torch.load(model_path, map_location="cpu")
    model.load_state_dict(state)
    model.eval()
    print(f"   ✅ Model loaded")
    # FIXED PATH: ../finetune/
    with open("../finetune/dataset_eval.jsonl", encoding="utf-8") as f:
        data = [json.loads(l) for l in f][:max_ex]
    correct = 0
    for ex in data:
        src = get_source(ex)
        if not src: continue
        exp_exit = ex.get("expected_exit", 0)
        prompt = tok.encode(src[:10], max_len=20)
        idx = torch.tensor([prompt])
        gen = model.generate(idx, max_new=32, temp=0.7)
        text = tok.decode(gen[0].tolist())
        try:
            with tempfile.NamedTemporaryFile("w", suffix=".mal", delete=False) as f:
                f.write(text); mp = f.name
            with tempfile.NamedTemporaryFile(suffix=".bin", delete=False) as f:
                bp = f.name
            cr = subprocess.run([MALC, mp, "-o", bp], capture_output=True, timeout=5)
            if cr.returncode == 0 and Path(bp).exists():
                rr = subprocess.run([bp], capture_output=True, timeout=5)
                if rr.returncode == exp_exit:
                    correct += 1
        except:
            pass
        finally:
            for p in [mp, bp]:
                try:
                    if p and Path(p).exists(): Path(p).unlink()
                except: pass
    accuracy = 100 * correct / len(data) if data else 0
    print(f"   📊 Accuracy: {correct}/{len(data)} = {accuracy:.1f}%")
    return {"name": name, "correct": correct, "total": len(data), "accuracy": accuracy}
print("🏆 MAL-Tiny Comparison: Evaluating Models")
print("="*70)
results = []
models = [
    ("Baseline (20 epochs)", "mal-tiny-ckpt/final.pt"),
    ("C: Inheritance (20+30)", "comparison/C_inherit/final.pt"),
    ("D: Multi-Round (100)", "comparison/D_multiround/final.pt"),
]
for name, path in models:
    r = evaluate_model(name, path, max_ex=10)
    if r:
        results.append(r)
print(f"\n{'='*70}")
print("🏆 FINAL COMPARISON")
print(f"{'='*70}")
print(f"{'Model':<35} {'Accuracy':<15} {'Correct':<10}")
print("-"*70)
for r in sorted(results, key=lambda x: x["accuracy"], reverse=True):
    print(f"{r['name']:<35} {r['accuracy']:>6.1f}%{'':>8} {r['correct']:>3}/{r['total']:<3}")
with open("comparison/evaluation_results.json", "w") as f:
    json.dump(results, f, indent=2)
print(f"\n💾 Results saved")
