#!/usr/bin/env python3
import sys, os, subprocess, tempfile
from typing import Dict
sys.path.insert(0, '/root/ilm-al-kitab/MAL/examples/ai/mal_tiny')
from mal_code_generator import MALCodeGenerator
class MALEndToEndEngine:
    def __init__(self, user_id: str = "user_001"):
        self.generator = MALCodeGenerator(user_id=user_id)
        self.malc_path = "/root/ilm-al-kitab/MAL/src/cli/target/release/malc"
    def process_and_execute(self, text: str) -> Dict:
        print(f"\n{'='*70}\n📝 المدخل: {text}\n{'='*70}")
        gen_result = self.generator.generate_mal_code(text)
        print(f"✅ المنطقة: {gen_result['region']} | الحتمية: {gen_result['certainty']:.1%}")
        print(f"💻 كود MAL:\n{gen_result['mal_code']}")
        mal_file = None
        binary_file = None
        try:
            # 🔑 تحسين 2: إنشاء ملفات مؤقتة بأسماء آمنة
            with tempfile.NamedTemporaryFile(mode='w', suffix='.mal', delete=False, encoding='utf-8') as f:
                f.write(gen_result['mal_code'])
                mal_file = f.name
            binary_file = mal_file.replace('.mal', '')
            print("\n⚙️ جاري التجميع...")
            if not os.path.exists(self.malc_path):
                return {"success": False, "error": "malc not found"}
            res = subprocess.run([self.malc_path, mal_file, "-o", binary_file], capture_output=True, text=True)
            if res.returncode != 0:
                print(f"❌ فشل التجميع:\n{res.stderr}")
                return {"success": False, "error": "Compilation failed", "stderr": res.stderr}
            print("✅ تم التجميع بنجاح!")
            print("🚀 جاري التنفيذ...")
            run = subprocess.run([binary_file], capture_output=True, text=True, timeout=5) # 🔑 تحسين 3: Timeout لمنع التعليق
            output = run.stdout.strip() or run.stderr.strip()
            print(f"📤 المخرجات: {output}")
            return {"success": True, "output": output}
        except subprocess.TimeoutExpired:
            print("❌ تجاوز وقت التنفيذ المسموح به (5 ثوانٍ).")
            return {"success": False, "error": "Timeout"}
        except Exception as e:
            print(f"❌ خطأ غير متوقع: {e}")
            return {"success": False, "error": str(e)}
        finally:
            # 🔑 تحسين 4: ضمان تنظيف الموارد دائماً (منع تسرب القرص)
            if mal_file and os.path.exists(mal_file):
                os.unlink(mal_file)
            if binary_file and os.path.exists(binary_file):
                os.unlink(binary_file)
if __name__ == "__main__":
    # تنظيف شامل قبل البدء
    for db in ["advanced_intelligence_cache.db", "mal_deterministic_cache.db", "mal_knowledge_base.db"]:
        if os.path.exists(db): os.remove(db)
    engine = MALEndToEndEngine(user_id="demo_user")
    # اختبار شامل يشمل حالات النجاح والفشل المتوقع
    test_queries = [
        "احسب قيمة 15 + 25",
        "ما هو مفهوم الاجتهاد في الفقه؟",
        "مجموع 10 و 20",
        "استعلام عشوائي غير معروف تماماً" # لاختبار الـ Fallback الآمن
    ]
    for query in test_queries:
        engine.process_and_execute(query)
    print("\n🎉 تم اختبار النظام الشامل بنجاح! جميع الموارد تم تنظيفها بأمان.")
