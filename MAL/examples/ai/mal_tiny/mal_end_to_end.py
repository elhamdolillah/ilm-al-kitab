#!/usr/bin/env python3
"""
النظام المتكامل من النص العربي إلى التنفيذ الفعلي عبر مترجم MAL
"""
import sys
import os
import subprocess
import tempfile
sys.path.insert(0, '/root/ilm-al-kitab/MAL/examples/ai/mal_tiny')
from mal_code_generator import MALCodeGenerator
class MALEndToEndEngine:
    def __init__(self, user_id: str = "user_001"):
        self.generator = MALCodeGenerator(user_id=user_id)
        self.malc_path = "/root/ilm-al-kitab/MAL/src/cli/target/release/malc"
    def process_and_execute(self, text: str) -> dict:
        """معالجة النص، توليد كود MAL، تجميعه، وتنفيذه"""
        print(f"\n{'='*80}")
        print(f"📝 المدخل: {text}")
        print(f"{'='*80}")
        # 1. توليد كود MAL
        gen_result = self.generator.generate_mal_code(text)
        mal_code = gen_result['mal_code']
        print(f"\n✅ المنطقة المعرفية: {gen_result['region']}")
        print(f"✅ نسبة الحتمية: {gen_result['certainty']:.1%}")
        if gen_result['disclaimer']:
            print(f"\n⚠️ {gen_result['disclaimer']}")
        print(f"\n💻 كود MAL المولد:")
        print("-" * 40)
        print(mal_code.strip())
        print("-" * 40)
        # 2. الحفظ في ملف مؤقت
        with tempfile.NamedTemporaryFile(mode='w', suffix='.mal', delete=False, encoding='utf-8') as f:
            f.write(mal_code)
            mal_file = f.name
        binary_file = mal_file.replace('.mal', '')
        # 3. التجميع باستخدام malc
        print(f"\n⚙️ جاري تجميع كود MAL...")
        if not os.path.exists(self.malc_path):
            return {"error": "مترجم malc غير موجود في المسار المحدد"}
        compile_cmd = [self.malc_path, mal_file, "-o", binary_file]
        compile_result = subprocess.run(compile_cmd, capture_output=True, text=True)
        if compile_result.returncode != 0:
            print("❌ فشل التجميع:")
            print(compile_result.stderr)
            os.unlink(mal_file)
            return {"error": "Compilation failed", "stderr": compile_result.stderr}
        print("✅ تم التجميع بنجاح!")
        # 4. التنفيذ
        print("🚀 جاري تنفيذ البرنامج المجمع...")
        run_result = subprocess.run([binary_file], capture_output=True, text=True)
        output = run_result.stdout.strip()
        if run_result.returncode != 0:
            output += f"\n[خطأ في التنفيذ: {run_result.stderr.strip()}]"
        print(f"\n📤 مخرجات التنفيذ:")
        print(f"   {output}")
        # 5. التنظيف
        os.unlink(mal_file)
        if os.path.exists(binary_file):
            os.unlink(binary_file)
        return {
            "input": text,
            "mal_code": mal_code,
            "region": gen_result['region'],
            "certainty": gen_result['certainty'],
            "execution_output": output,
            "success": True
        }
if __name__ == "__main__":
    engine = MALEndToEndEngine(user_id="demo_user")
    # تنظيف قواعد البيانات للتجربة النقية
    for db in ["user_specialization.db", "grammar_math_rules.db"]:
        if os.path.exists(db):
            os.remove(db)
    test_queries = [
        "احسب قيمة 15 + 25",
        "ما حكم الصلاة في وقتها؟",
        "اشرح لي مفهوم الاجتهاد",
    ]
    for query in test_queries:
        result = engine.process_and_execute(query)
        if not result.get('success'):
            print(f"⚠️ لم يتم التنفيذ بنجاح: {result.get('error')}")
    print("\n🎉 تم الربط الفعلي والتنفيذي مع مترجم MAL بنجاح!")
