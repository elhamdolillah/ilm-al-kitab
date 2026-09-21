"""Detailed evaluation showing exactly why each example fails."""
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
def clean_output(text):
    """Remove common generation errors."""
    # Remove trailing ')' if unbalanced
    while text.endswith(')') and text.count('(') < text.count(')'):
        text = text[:-1]
    # Remove trailing ')' if no matching '('
    if text.endswith(')') and '(' not in text:
        text = text[:-1]
    return text.strip()
def evaluate_model_detailed(name, model_path, max_ex=10):
    print(f"\n{'='*80}")
    print(f"🔬 Evaluating: {name}")
    print(f"{'='*80}")
    if not Path(model_path).exists():
        print(f"   ⚠️ Model not found")
        return None
    tok = MALTokenizer()
    model = MALTiny(tok.vocab_size)
    state = torch.load(model_path, map_location="cpu")
    model.load_state_dict(state)
    model.eval()
    with open("../finetune/dataset_eval.jsonl", encoding="utf-8") as f:
        data = [json.loads(l) for l in f][:max_ex]
    stats = {"correct": 0, "compile_fail": 0, "wrong_exit": 0, "total": len(data)}
    for i, ex in enumerate(data):
        src = get_source(ex)
        if not src: continue
        exp_exit = ex.get("expected_exit", 0)
        # Use FULL source as prompt (not just 10 chars)
        prompt = tok.encode(src, max_len=60)
        idx = torch.tensor([prompt])
        gen = model.generate(idx, max_new=20, temp=0.5)  # Lower temp
        text = tok.decode(gen[0].tolist())
        text = clean_output(text)
        print(f"\n[{i+1}/{len(data)}] {ex.get('category', '?')}")
        print(f"  Target:  {src[:60]}")
        print(f"  Gen:     {text[:60]}")
        # Check if generation matches target
        if text.strip() == src.strip():
            print(f"  Match:   ✅ EXACT MATCH")
            stats["correct"] += 1
            continue
        # Try to compile
        try:
            with tempfile.NamedTemporaryFile("w", suffix=".mal", delete=False) as f:
                f.write(text); mp = f.name
            with tempfile.NamedTemporaryFile(suffix=".bin", delete=False) as f:
                bp = f.name
            cr = subprocess.run([MALC, mp, "-o", bp], capture_output=True, timeout=5)
            if cr.returncode == 0 and Path(bp).exists():
                rr = subprocess.run([bp], capture_output=True, timeout=5)
                if rr.returncode == exp_exit:
                    print(f"  Match:   ✅ COMPILED + CORRECT EXIT")
                    stats["correct"] += 1
                else:
                    print(f"  Match:   ❌ Wrong exit: {rr.returncode} (expected {exp_exit})")
                    stats["wrong_exit"] += 1
            else:
                err = cr.stderr.decode()[:60] if cr.stderr else "syntax error"
                print(f"  Match:   ❌ COMPILE_FAIL: {err}")
                stats["compile_fail"] += 1
        except Exception as e:
            print(f"  Match:   ❌ ERROR: {e}")
        finally:
            for p in [mp, bp]:
                try:
                    if p and Path(p).exists(): Path(p).unlink()
                except: pass
    print(f"\n{'='*80}")
    print(f"📊 Statistics for {name}:")
    print(f"   Total:        {stats['total']}")
    print(f"   Correct:      {stats['correct']} ({100*stats['correct']/stats['total']:.1f}%)")
    print(f"   Compile fail: {stats['compile_fail']}")
    print(f"   Wrong exit:   {stats['wrong_exit']}")
    print(f"{'='*80}")
    return {"name": name, **stats}
print("🔬 DETAILED EVALUATION")
print("="*80)
print("Changes from previous evaluation:")
print("  1. Use FULL source as prompt (not just 10 chars)")
print("  2. Clean output before compilation")
print("  3. Lower temperature (0.5 instead of 0.7)")
print("  4. Show detailed error analysis")
print("="*80)
results = []
models = [
    ("Baseline (20 epochs)", "mal-tiny-ckpt/final.pt"),
    ("C: Inheritance (20+30)", "comparison/C_inherit/final.pt"),
    ("D: Multi-Round (100)", "comparison/D_multiround/final.pt"),
]
for name, path in models:
    r = evaluate_model_detailed(name, path, max_ex=10)
    if r:
        results.append(r)
# Final summary
print(f"\n{'='*80}")
print("🏆 FINAL SUMMARY")
print(f"{'='*80}")
print(f"{'Model':<30} {'Correct':<10} {'Compile':<10} {'Wrong':<10} {'%':<8}")
print("-"*80)
for r in sorted(results, key=lambda x: x["correct"], reverse=True):
    pct = 100*r["correct"]/r["total"]
    print(f"{r['name']:<30} {r['correct']:>4}/{r['total']:<4} {r['compile_fail']:>6} {r['wrong_exit']:>7} {pct:>5.1f}%")
with open("comparison/detailed_results.json", "w") as f:
    json.dump(results, f, indent=2)
print(f"\n💾 Saved to comparison/detailed_results.json")
