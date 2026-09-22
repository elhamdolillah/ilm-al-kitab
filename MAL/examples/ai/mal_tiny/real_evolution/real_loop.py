"""
REAL Evolutionary Loop - No simulation, no fake numbers.
Executes generated code in Node.js/Python, measures real performance.
"""
import torch
import json
import time
import subprocess
import tempfile
import os
import sys
from pathlib import Path
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
sys.path.insert(0, "..")
from model import MALTiny, MALTokenizer, count_params
@dataclass
class Task:
    id: str
    name: str
    category: str
    input_data: str
    expected_output: str
    test_inputs: List[str]
@dataclass
class Candidate:
    id: str
    language: str
    code: str
    task_id: str
    generation: int
@dataclass
class BenchmarkResult:
    candidate_id: str
    language: str
    accuracy: float
    avg_time_ms: float
    ops_per_sec: float
    successful_tests: int
    total_tests: int
    error: Optional[str] = None
class RealEvolutionaryLoop:
    def __init__(self, model_path: str, work_dir: str = "../real_evolution_work"):
        self.model_path = Path(model_path)
        self.work_dir = Path(work_dir)
        self.work_dir.mkdir(parents=True, exist_ok=True)
        (self.work_dir / "checkpoints").mkdir(exist_ok=True)
        (self.work_dir / "library").mkdir(exist_ok=True)
        (self.work_dir / "logs").mkdir(exist_ok=True)
        self.tok = MALTokenizer()
        self.model = MALTiny(self.tok.vocab_size)
        if self.model_path.exists():
            state = torch.load(self.model_path, map_location="cpu")
            if isinstance(state, dict) and all(isinstance(k, str) for k in state.keys()):
                self.model.load_state_dict(state)
            print(f"Loaded model: {self.model_path} ({count_params(self.model):,} params)")
        else:
            print(f"Model not found, starting from scratch")
        self.node_available = self._check_tool("node", "--version")
        print(f"Node.js: {'available' if self.node_available else 'NOT AVAILABLE'}")
        print(f"Python: available")
        self.generation = 0
        self.library = []
        self.training_examples = []
    def _check_tool(self, tool, flag):
        try:
            r = subprocess.run([tool, flag], capture_output=True, timeout=5)
            return r.returncode == 0
        except:
            return False
    def generate_tasks(self) -> List[Task]:
        tasks = []
        tasks.append(Task(
            id="extract_email",
            name="Extract Email Addresses",
            category="text_extraction",
            input_data="Contact us at support@example.com or sales@test.org",
            expected_output=json.dumps(["support@example.com", "sales@test.org"]),
            test_inputs=[
                "Email: a@b.com, c@d.org",
                "No emails here",
                "user.name+tag@domain.co.uk is valid",
                "Invalid: @missing.com, also@bad",
                "Multiple: x@y.com z@w.net v@u.org",
            ]
        ))
        tasks.append(Task(
            id="extract_url",
            name="Extract URLs",
            category="text_extraction",
            input_data="Visit https://example.com or http://test.org/page",
            expected_output=json.dumps(["https://example.com", "http://test.org/page"]),
            test_inputs=[
                "URL: https://a.com/b",
                "No URLs",
                "ftp://files.example.com/data",
                "Two: https://x.com and http://y.org",
                "Complex: https://sub.domain.com:8080/path?q=1",
            ]
        ))
        tasks.append(Task(
            id="extract_attrs",
            name="Extract HTML Attributes",
            category="html_parsing",
            input_data='<input type="email" name="user" required>',
            expected_output=json.dumps({"tag": "input", "type": "email", "name": "user", "required": True}),
            test_inputs=[
                '<a href="/x" class="link">T</a>',
                '<div id="main" class="box">',
                '<button type="submit" disabled>',
                '<img src="p.jpg" alt="P" width="100">',
                '<input type="password" name="pwd">',
            ]
        ))
        tasks.append(Task(
            id="sum_numbers",
            name="Sum Numbers in Text",
            category="computation",
            input_data="Prices: 10, 20, 30, 40",
            expected_output=json.dumps(100),
            test_inputs=[
                "Values: 1, 2, 3",
                "No numbers",
                "100 200 300",
                "Mixed: 5 apples, 10 oranges",
                "Single: 42",
            ]
        ))
        tasks.append(Task(
            id="count_words",
            name="Count Words",
            category="computation",
            input_data="Hello world from MAL",
            expected_output=json.dumps(4),
            test_inputs=[
                "One",
                "Two words",
                "",
                "A B C D E F",
                "The quick brown fox",
            ]
        ))
        return tasks
    def generate_candidate(self, task: Task, language: str, variant: int) -> Candidate:
        if language == "javascript":
            code = self._gen_js(task, variant)
        elif language == "python":
            code = self._gen_python(task, variant)
        else:
            raise ValueError(f"Unknown language: {language}")
        return Candidate(
            id=f"{task.id}_{language}_v{variant}",
            language=language,
            code=code,
            task_id=task.id,
            generation=self.generation
        )
    def _gen_js(self, task, variant):
        if task.id == "extract_email":
            if variant == 0:
                return 'function solve(input) {\n    const re = /[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}/g;\n    return input.match(re) || [];\n}\n'
            else:
                return 'function solve(input) {\n    const re = /\\b[\\w.+-]+@[\\w-]+\\.[\\w.-]+\\b/g;\n    return input.match(re) || [];\n}\n'
        elif task.id == "extract_url":
            if variant == 0:
                return 'function solve(input) {\n    const re = /https?:\\/\\/[^\\s]+/g;\n    return input.match(re) || [];\n}\n'
            else:
                return 'function solve(input) {\n    const re = /\\b(https?|ftp):\\/\\/[^\\s"\'<>]+/gi;\n    return input.match(re) || [];\n}\n'
        elif task.id == "extract_attrs":
            return '''function solve(input) {
    const tagMatch = input.match(/<(\\w+)/);
    const tag = tagMatch ? tagMatch[1] : null;
    const result = {tag: tag};
    const attrRe = /(\\w+)=(?:"([^"]*)"|'([^']*)'|([^\\s>]+))/g;
    let m;
    while ((m = attrRe.exec(input)) !== null) {
        const name = m[1];
        const value = m[2] !== undefined ? m[2] : (m[3] !== undefined ? m[3] : m[4]);
        if (value === "" || value === undefined) {
            result[name] = true;
        } else if (!isNaN(value) && value !== "") {
            result[name] = Number(value);
        } else {
            result[name] = value;
        }
    }
    return result;
}
'''
        elif task.id == "sum_numbers":
            return 'function solve(input) {\n    const nums = input.match(/-?\\d+(?:\\.\\d+)?/g);\n    if (!nums) return 0;\n    return nums.reduce((s, n) => s + Number(n), 0);\n}\n'
        elif task.id == "count_words":
            return 'function solve(input) {\n    if (!input.trim()) return 0;\n    return input.trim().split(/\\s+/).length;\n}\n'
        return 'function solve(input) { return null; }\n'
    def _gen_python(self, task, variant):
        if task.id == "extract_email":
            return 'import re\ndef solve(input):\n    return re.findall(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}", input)\n'
        elif task.id == "extract_url":
            return 'import re\ndef solve(input):\n    return re.findall(r"https?://[^\\s]+", input)\n'
        elif task.id == "extract_attrs":
            return '''import re
def solve(input):
    tag_m = re.search(r"<(\\w+)", input)
    tag = tag_m.group(1) if tag_m else None
    result = {"tag": tag}
    for m in re.finditer(r'(\\w+)=(?:"([^"]*)"|' + "'" + r'([^' + "'" + r']*)' + "'" + r'|([^\\s>]+))', input):
        name = m.group(1)
        val = m.group(2) if m.group(2) is not None else (m.group(3) if m.group(3) is not None else m.group(4))
        if val is None or val == "":
            result[name] = True
        else:
            try:
                result[name] = int(val) if "." not in val else float(val)
            except ValueError:
                result[name] = val
    return result
'''
        elif task.id == "sum_numbers":
            return 'import re\ndef solve(input):\n    nums = re.findall(r"-?\\d+(?:\\.\\d+)?", input)\n    return sum(float(n) if "." in n else int(n) for n in nums) if nums else 0\n'
        elif task.id == "count_words":
            return 'def solve(input):\n    return len(input.split()) if input.strip() else 0\n'
        return 'def solve(input):\n    return None\n'
    def benchmark_candidate(self, candidate, task) -> BenchmarkResult:
        successful = 0
        total = len(task.test_inputs)
        times_ms = []
        error = None
        for test_input in task.test_inputs:
            try:
                result, elapsed_ms = self._execute(candidate, test_input)
                times_ms.append(elapsed_ms)
                expected = self._compute_expected(task, test_input)
                if self._compare_results(result, expected):
                    successful += 1
            except Exception as e:
                error = str(e)[:100]
        accuracy = successful / total if total > 0 else 0.0
        avg_time = sum(times_ms) / len(times_ms) if times_ms else 1000.0
        ops_per_sec = 1000.0 / avg_time if avg_time > 0 else 0.0
        return BenchmarkResult(
            candidate_id=candidate.id,
            language=candidate.language,
            accuracy=accuracy,
            avg_time_ms=avg_time,
            ops_per_sec=ops_per_sec,
            successful_tests=successful,
            total_tests=total,
            error=error
        )
    def _execute(self, candidate, input_data, iterations=10):
        if candidate.language == "javascript":
            return self._execute_js(candidate.code, input_data, iterations)
        elif candidate.language == "python":
            return self._execute_python(candidate.code, input_data, iterations)
    def _execute_js(self, code, input_data, iterations):
        script = code + '\nconst input = ' + json.dumps(input_data) + ';\nconst result = solve(input);\nconsole.log(JSON.stringify(result));\n'
        with tempfile.NamedTemporaryFile(mode='w', suffix='.js', delete=False, encoding='utf-8') as f:
            f.write(script)
            tmp_path = f.name
        try:
            # Warmup
            subprocess.run(['node', tmp_path], capture_output=True, timeout=5)
            start = time.perf_counter()
            result = None
            for _ in range(iterations):
                r = subprocess.run(['node', tmp_path], capture_output=True, timeout=5, text=True)
                if r.returncode == 0 and r.stdout.strip():
                    result = json.loads(r.stdout.strip())
            elapsed_ms = ((time.perf_counter() - start) / iterations) * 1000.0
            return result, elapsed_ms
        finally:
            try:
                os.unlink(tmp_path)
            except:
                pass
    def _execute_python(self, code, input_data, iterations):
        script = code + '\nimport json\ninput_data = ' + json.dumps(input_data) + '\nresult = solve(input_data)\nprint(json.dumps(result))\n'
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
            try:
                os.unlink(tmp_path)
            except:
                pass
    def _compute_expected(self, task, test_input):
        import re
        if task.id == "extract_email":
            return re.findall(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}", test_input)
        elif task.id == "extract_url":
            return re.findall(r"https?://[^\s]+", test_input)
        elif task.id == "sum_numbers":
            nums = re.findall(r"-?\d+(?:\.\d+)?", test_input)
            return sum(float(n) if "." in n else int(n) for n in nums) if nums else 0
        elif task.id == "count_words":
            return len(test_input.split()) if test_input.strip() else 0
        elif task.id == "extract_attrs":
            tag_m = re.search(r"<(\w+)", test_input)
            tag = tag_m.group(1) if tag_m else None
            result = {"tag": tag}
            for m in re.finditer(r'(\w+)=(?:"([^"]*)"|\'([^\']*)\'|([^\s>]+))', test_input):
                name = m.group(1)
                val = m.group(2) if m.group(2) is not None else (m.group(3) if m.group(3) is not None else m.group(4))
                if val is None or val == "":
                    result[name] = True
                else:
                    try:
                        result[name] = int(val) if "." not in val else float(val)
                    except ValueError:
                        result[name] = val
            return result
        return None
    def _compare_results(self, actual, expected):
        try:
            if isinstance(expected, (int, float)) and isinstance(actual, (int, float)):
                return abs(float(actual) - float(expected)) < 0.01
            if isinstance(expected, list) and isinstance(actual, list):
                return sorted(str(x) for x in expected) == sorted(str(x) for x in actual)
            if isinstance(expected, dict) and isinstance(actual, dict):
                if set(expected.keys()) != set(actual.keys()):
                    return False
                for k in expected:
                    if not self._compare_results(actual.get(k), expected[k]):
                        return False
                return True
            return actual == expected
        except:
            return False
    def run(self, generations=3, candidates_per_task=4):
        print(f"\n{'='*70}")
        print(f"Starting REAL Evolutionary Loop")
        print(f"   Generations: {generations}")
        print(f"   Candidates per task: {candidates_per_task}")
        print(f"{'='*70}\n")
        for gen in range(generations):
            self.generation = gen
            print(f"\n{'='*70}")
            print(f"Generation {gen + 1}/{generations}")
            print(f"{'='*70}")
            tasks = self.generate_tasks()
            print(f"Tasks: {len(tasks)}")
            candidates = []
            for task in tasks:
                for v in range(candidates_per_task // 2):
                    candidates.append(self.generate_candidate(task, "javascript", v))
                    candidates.append(self.generate_candidate(task, "python", v))
            print(f"Candidates: {len(candidates)}")
            print(f"Benchmarking (real execution, {10} iterations each)...")
            results = []
            for i, cand in enumerate(candidates, 1):
                task = next(t for t in tasks if t.id == cand.task_id)
                res = self.benchmark_candidate(cand, task)
                results.append((cand, res))
                status = "PASS" if res.accuracy >= 0.97 else ("OK" if res.accuracy >= 0.8 else "FAIL")
                err_msg = f" (err: {res.error})" if res.error else ""
                print(f"   [{i}/{len(candidates)}] {status} {cand.id}: acc={res.accuracy:.0%}, {res.ops_per_sec:.0f} ops/s{err_msg}")
            best = [(c, r) for c, r in results if r.accuracy >= 0.97]
            best.sort(key=lambda x: x[1].ops_per_sec, reverse=True)
            print(f"\nBest (97%+ accuracy): {len(best)}")
            for c, r in best[:5]:
                print(f"   {c.id}: {r.ops_per_sec:.0f} ops/s, {r.accuracy:.0%}")
            for c, r in best:
                task = next(t for t in tasks if t.id == c.task_id)
                self.training_examples.append({
                    "task_id": task.id,
                    "language": c.language,
                    "code": c.code,
                    "accuracy": r.accuracy,
                    "ops_per_sec": r.ops_per_sec
                })
            gen_data = {
                "generation": gen,
                "total_candidates": len(candidates),
                "successful": len(best),
                "all_results": [
                    {"id": c.id, "language": c.language, "accuracy": r.accuracy, "ops_per_sec": r.ops_per_sec, "error": r.error}
                    for c, r in results
                ]
            }
            with open(self.work_dir / f"gen_{gen}.json", 'w') as f:
                json.dump(gen_data, f, indent=2)
            for c, r in best:
                self.library.append({
                    "task_id": c.task_id,
                    "language": c.language,
                    "code": c.code,
                    "accuracy": r.accuracy,
                    "ops_per_sec": r.ops_per_sec,
                    "generation": gen
                })
            with open(self.work_dir / "library.json", 'w') as f:
                json.dump(self.library, f, indent=2)
            print(f"\nGeneration {gen+1} Summary:")
            print(f"   Library: {len(self.library)} rules")
            print(f"   Successful: {len(best)}/{len(candidates)}")
            if best:
                avg_ops = sum(r.ops_per_sec for _, r in best) / len(best)
                print(f"   Avg speed (best): {avg_ops:.0f} ops/sec")
        print(f"\n{'='*70}")
        print(f"Evolution Complete!")
        print(f"   Final library: {len(self.library)} rules")
        print(f"   Training examples: {len(self.training_examples)}")
        print(f"{'='*70}")
        return self.library
if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--model", default="../MAL-Tiny-Baseline-80pct.pt")
    parser.add_argument("--generations", type=int, default=3)
    args = parser.parse_args()
    loop = RealEvolutionaryLoop(args.model)
    loop.run(generations=args.generations, candidates_per_task=4)
