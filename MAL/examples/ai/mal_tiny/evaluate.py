"""Evaluate MAL-Tiny model."""
import json, subprocess, tempfile, torch, argparse, sys
from pathlib import Path
from model import MALTiny, MALTokenizer
def get_source(ex):
    for f in ["mal_source", "source", "completion", "text", "code"]:
        if f in ex and ex[f]:
            return str(ex[f]).strip()
    if "prompt" in ex:
        p = str(ex["prompt"])
        if "### Response:" in p: p = p.split("### Response:")[-1]
        elif "اكتب كود MAL" in p: p = p.split("\n")[-1]
        return p.strip()
    return None
def evaluate(model_path, eval_path, malc, max_ex=10, max_new=32):
    print(f"🔬 Loading model: {model_path}")
    tok = MALTokenizer()
    model = MALTiny(tok.vocab_size)
    state = torch.load(model_path, map_location="cpu")
    if isinstance(state, dict) and "state" in state:
        state = state["state"]
    model.load_state_dict(state)
    model.eval()
    print(f"✅ Model loaded ({sum(p.numel() for p in model.parameters()):,} params)")
    with open(eval_path, encoding="utf-8") as f:
        data = [json.loads(l) for l in f][:max_ex]
    print(f"📊 Evaluating {len(data)} examples\n")
    print("-" * 70)
    correct, total, compile_fails, parse_errors = 0, 0, 0, 0
    for i, ex in enumerate(data, 1):
        src = get_source(ex)
        if not src:
            print(f"[{i}/{len(data)}] ⚠️ No source")
            continue
        exp_exit = ex.get("expected_exit", 0)
        # Generate
        prompt = tok.encode(src[:10], max_len=20)
        idx = torch.tensor([prompt])
        gen = model.generate(idx, max_new=max_new, temp=0.7)
        text = tok.decode(gen[0].tolist())
        print(f"\n[{i}/{len(data)}] Category: {ex.get('category', '?')}")
        print(f"  Target: {src}")
        print(f"  Gen:    {text[:60]}{'...' if len(text) > 60 else ''}")
        # Compile + run
        mp = bp = None
        try:
            with tempfile.NamedTemporaryFile("w", suffix=".mal", delete=False) as f:
                f.write(text); mp = f.name
            with tempfile.NamedTemporaryFile(suffix=".bin", delete=False) as f:
                bp = f.name
            cr = subprocess.run([malc, mp, "-o", bp], capture_output=True, timeout=5)
            if cr.returncode == 0 and Path(bp).exists():
                rr = subprocess.run([bp], capture_output=True, timeout=5)
                ok = rr.returncode == exp_exit
                correct += int(ok)
                sym = "✅" if ok else "❌"
                print(f"  Result: {sym} exit={rr.returncode} (expected {exp_exit})")
            else:
                compile_fails += 1
                err = cr.stderr.decode()[:80] if cr.stderr else "unknown"
                print(f"  Result: ❌ COMPILE_FAIL ({err})")
        except subprocess.TimeoutExpired:
            parse_errors += 1
            print(f"  Result: ❌ TIMEOUT")
        except Exception as e:
            print(f"  Result: ❌ ERROR: {e}")
        finally:
            for p in [mp, bp]:
                try:
                    if p and Path(p).exists(): Path(p).unlink()
                except: pass
        total += 1
    print("\n" + "=" * 70)
    acc = 100*correct/total if total else 0
    print(f"📊 Final Results:")
    print(f"   Total evaluated:    {total}")
    print(f"   Correct:            {correct} ({acc:.1f}%)")
    print(f"   Compile failures:   {compile_fails}")
    print(f"   Timeouts/errors:    {parse_errors}")
    print("=" * 70)
    # Save results
    results = {
        "model": model_path,
        "total": total,
        "correct": correct,
        "accuracy": round(acc, 1),
        "compile_failures": compile_fails,
        "errors": parse_errors
    }
    with open("eval_results.json", "w") as f:
        json.dump(results, f, indent=2)
    print(f"\n💾 Results saved to eval_results.json")
if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("--model", default="./mal-tiny-ckpt/final.pt")
    p.add_argument("--eval-data", default="../finetune/dataset_eval.jsonl")
    p.add_argument("--malc", default="/root/ilm-al-kitab/MAL/tools/malc-native/target/release/malc-native")
    p.add_argument("--max-ex", type=int, default=10)
    p.add_argument("--max-new", type=int, default=32)
    args = p.parse_args()
    evaluate(args.model, args.eval_data, args.malc, args.max_ex, args.max_new)
