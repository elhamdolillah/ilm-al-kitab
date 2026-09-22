"""Code Mutation Engine - paths fixed for mal_tiny/ execution."""
import json, time, subprocess, tempfile, os, sys, re
from pathlib import Path
from typing import List, Dict
from dataclasses import dataclass
# WORK DIR: from mal_tiny/ → ./real_evolution_work/
WORK_DIR = Path("./real_evolution_work")
@dataclass
class MutatedRule:
    id: str
    parent_id: str
    task_id: str
    mutation_type: str
    code: str
    accuracy: float
    ops_per_sec: float
def mutate_python_regex(code: str, task_id: str) -> List[Dict]:
    pattern_match = re.search(r'r"([^"]+)"', code)
    if not pattern_match:
        return []
    original = pattern_match.group(1)
    variants = []
    # Variant 1: Pre-compiled module-level (fastest usually)
    variants.append({
        "type": "precompiled",
        "code": f'import re\n_PATTERN = re.compile(r"{original}")\ndef solve(input):\n    return _PATTERN.findall(input)\n'
    })
    # Variant 2: finditer
    variants.append({
        "type": "finditer",
        "code": f'import re\n_PATTERN = re.compile(r"{original}")\ndef solve(input):\n    return [m.group(0) for m in _PATTERN.finditer(input)]\n'
    })
    # Variant 3: lru_cache
    variants.append({
        "type": "cached",
        "code": f'import re\nfrom functools import lru_cache\n@lru_cache(maxsize=128)\ndef _compiled():\n    return re.compile(r"{original}")\ndef solve(input):\n    return _compiled().findall(input)\n'
    })
    return variants
def execute_python(code: str, input_data: str, iterations: int = 5):
    script = code + f'\nimport json\ninput_data = {json.dumps(input_data)}\nresult = solve(input_data)\nprint(json.dumps(result))\n'
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False, encoding='utf-8') as f:
        f.write(script)
        tmp_path = f.name
    try:
        subprocess.run([sys.executable, tmp_path], capture_output=True, timeout=5)
        start = time.perf_counter()
        result = None
        for _ in range(iterations):
            r = subprocess.run([sys.executable, tmp_path], capture_output=True, timeout=5, text=True)
            if r.returncode == 0 and r.stdout.strip():
                result = json.loads(r.stdout.strip())
        elapsed_ms = ((time.perf_counter() - start) / iterations) * 1000.0
        return result, elapsed_ms
    finally:
        try: os.unlink(tmp_path)
        except: pass
def get_expected(task_id, test_input):
    if task_id == "extract_email":
        return re.findall(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}", test_input)
    elif task_id == "extract_url":
        return re.findall(r"https?://[^\s]+", test_input)
    elif task_id == "sum_numbers":
        nums = re.findall(r"-?\d+(?:\.\d+)?", test_input)
        return sum(float(n) if "." in n else int(n) for n in nums) if nums else 0
    elif task_id == "count_words":
        return len(test_input.split()) if test_input.strip() else 0
    return None
def compare_results(actual, expected):
    try:
        if isinstance(expected, list) and isinstance(actual, list):
            return sorted(str(x) for x in expected) == sorted(str(x) for x in actual)
        if isinstance(expected, (int, float)) and isinstance(actual, (int, float)):
            return abs(float(actual) - float(expected)) < 0.01
        return actual == expected
    except:
        return False
def main():
    print("=" * 70)
    print("Mutation Engine (REAL execution)")
    print("=" * 70)
    lib_path = WORK_DIR / "library.json"
    if not lib_path.exists():
        print(f"ERROR: {lib_path} not found")
        return
    with open(lib_path) as f:
        library = json.load(f)
    python_rules = [r for r in library if r["language"] == "python"]
    print(f"Loaded {len(python_rules)} Python rules")
    test_inputs = {
        "extract_email": ["Email: a@b.com, c@d.org", "user.name+tag@domain.co.uk", "Multiple: x@y.com z@w.net"],
        "extract_url": ["URL: https://a.com/b", "Two: https://x.com and http://y.org", "ftp://files.example.com/data"],
        "sum_numbers": ["Values: 1, 2, 3", "100 200 300", "Mixed: 5 apples, 10 oranges"],
        "count_words": ["One", "A B C D E F", "The quick brown fox"]
    }
    seen = set()
    unique_parents = []
    for r in python_rules:
        h = hash(r["code"])
        if h not in seen:
            seen.add(h)
            unique_parents.append(r)
    print(f"Unique parent rules: {len(unique_parents)}")
    all_mutants = []
    for parent in unique_parents:
        variants = mutate_python_regex(parent["code"], parent["task_id"])
        tests = test_inputs.get(parent["task_id"], [])
        for v in variants:
            ok = 0
            times = []
            for t in tests:
                try:
                    result, ms = execute_python(v["code"], t)
                    times.append(ms)
                    exp = get_expected(parent["task_id"], t)
                    if compare_results(result, exp):
                        ok += 1
                except: pass
            acc = ok / len(tests) if tests else 0
            avg = sum(times)/len(times) if times else 1000
            ops = 1000/avg if avg > 0 else 0
            all_mutants.append({
                "id": f"{parent['task_id']}_{v['type']}",
                "task_id": parent["task_id"],
                "type": v["type"],
                "accuracy": acc,
                "ops": ops,
                "code": v["code"]
            })
            s = "PASS" if acc >= 0.97 else "FAIL"
            print(f"  [{s}] {parent['task_id']}_{v['type']}: acc={acc:.0%}, {ops:.0f} ops/s")
    with open(WORK_DIR / "mutations.json", 'w') as f:
        json.dump(all_mutants, f, indent=2)
    baseline = 24
    faster = [m for m in all_mutants if m["ops"] > baseline and m["accuracy"] >= 0.97]
    print(f"\nFaster than {baseline} ops/s: {len(faster)}")
    if faster:
        print("Winners:")
        for m in sorted(faster, key=lambda x: x["ops"], reverse=True)[:5]:
            print(f"  {m['id']}: {m['ops']:.0f} ops/s ({m['ops']/baseline:.2f}x)")
if __name__ == "__main__":
    main()
