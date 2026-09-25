#!/usr/bin/env python3
import sys, os, subprocess, tempfile
sys.path.insert(0, '/root/ilm-al-kitab/MAL/examples/ai/mal_tiny')
from mal_code_generator import MALCodeGenerator
class MALEndToEndEngine:
    def __init__(self, user_id: str = "user_001"):
        self.generator = MALCodeGenerator(user_id=user_id)
        self.malc_path = "/root/ilm-al-kitab/MAL/src/cli/target/release/malc"
    def process_and_execute(self, text: str) -> dict:
        print(f"\n{'='*80}\n📝 المدخل: {text}\n{'='*80}")
        gen_result = self.generator.generate_mal_code(text)
        print(f"\n✅ المنطقة: {gen_result['region']} | الحتمية: {gen_result['certainty']:.1%}")
        if gen_result.get('disclaimer'): print(f"⚠️ {gen_result['disclaimer']}")
        print(f"\n💻 كود MAL:\n{'-'*40}\n{gen_result['mal_code'].strip()}\n{'-'*40}")
        with tempfile.NamedTemporaryFile(mode='w', suffix='.mal', delete=False, encoding='utf-8') as f:
            f.write(gen_result['mal_code'])
            mal_file = f.name
        binary_file = mal_file.replace('.mal', '')
        print("\n⚙️ جاري التجميع...")
        if not os.path.exists(self.malc_path):
            return {"error": "malc not found"}
        res = subprocess.run([self.malc_path, mal_file, "-o", binary_file], capture_output=True, text=True)
        if res.returncode != 0:
            print(f"❌ فشل التجميع:\n{res.stderr}")
            os.unlink(mal_file)
            return {"error": "Compilation failed"}
        print("✅ تم التجميع!")
        print("🚀 جاري التنفيذ...")
        run = subprocess.run([binary_file], capture_output=True, text=True)
        output = run.stdout.strip() or run.stderr.strip()
        print(f"📤 المخرجات: {output}")
        os.unlink(mal_file)
        if os.path.exists(binary_file): os.unlink(binary_file)
        return {"success": True, "output": output}
if __name__ == "__main__":
    engine = MALEndToEndEngine(user_id="demo_user")
    for db in ["user_specialization.db", "grammar_math_rules.db", "mal_deterministic_cache.db"]:
        if os.path.exists(db): os.remove(db)
    for query in ["احسب قيمة 15 + 25", "ما هو مفهوم الاجتهاد في الفقه؟", "مجموع المصفوفة"]:
        engine.process_and_execute(query)
    print("\n🎉 تم الربط الفعلي بنجاح!")
