#!/usr/bin/env python3
"""
MAL Mobile Integration Server
يستقبل أوامر نصية من الهاتف، يولد كود MAL، وينفذه عبر المترجم.
"""
import os
import sys
import subprocess
import json
from flask import Flask, request, jsonify
app = Flask(__name__)
PROJECT_DIR = "/root/ilm-al-kitab"
GENERATOR_DIR = os.path.join(PROJECT_DIR, "MAL/examples/ai/mal_tiny")
TEMP_MAL_FILE = os.path.join(PROJECT_DIR, "temp_mobile_input.mal")
@app.route('/health', methods=['GET'])
def health():
    return jsonify({"status": "healthy", "service": "MAL Compiler Server"})
@app.route('/generate-and-run', methods=['POST'])
def generate_and_run():
    data = request.get_json()
    if not data or 'text' not in data:
        return jsonify({"error": "Missing 'text' in request body"}), 400
    user_text = data['text']
    try:
        # حفظ المجلد الحالي والانتقال لمجلد المولد لضمان صحة المسارات النسبية
        original_cwd = os.getcwd()
        os.chdir(GENERATOR_DIR)
        # 1. تحميل مولد الكود الذكي
        sys.path.insert(0, GENERATOR_DIR)
        from mal_code_generator_v4 import MALCodeGeneratorV4
        generator = MALCodeGeneratorV4()
        # توليد الكود دون حفظ تلقائي، سنحفظه نحن للتحكم
        mal_code = generator.generate(user_text, save_to_file=False)
        # 2. حفظ الكود في ملف مؤقت
        with open(TEMP_MAL_FILE, "w", encoding="utf-8") as f:
            f.write(mal_code)
        # 3. تنفيذ الكود عبر malc (نستخدم cargo run لضمان وجود البيئة)
        # ملاحظة: في بيئة الإنتاج، يُفضل استخدام الملف الثنائي المبنى مباشرة
        cli_dir = os.path.join(PROJECT_DIR, "MAL/src/cli")
        result = subprocess.run(
            ["cargo", "run", "--release", "--bin", "malc", "--", "run", TEMP_MAL_FILE],
            cwd=cli_dir,
            capture_output=True,
            text=True,
            timeout=15 # منع التعليق في حال وجود حلقة لا نهائية
        )
        if result.returncode == 0:
            output = result.stdout.strip()
        else:
            output = f"❌ خطأ في التجميع أو التنفيذ:\n{result.stderr.strip()}"
    except ImportError as e:
        output = f"❌ خطأ في استيراد مولد الكود: {str(e)}"
        mal_code = "// Failed to load generator"
    except subprocess.TimeoutExpired:
        output = "❌ تجاوز وقت التنفيذ المحدد (Timeout)."
        mal_code = "// Timeout"
    except Exception as e:
        output = f"❌ خطأ غير متوقع: {str(e)}"
        mal_code = "// Error"
    finally:
        # استعادة مجلد العمل الأصلي
        os.chdir(original_cwd)
        # تنظيف الملف المؤقت
        if os.path.exists(TEMP_MAL_FILE):
            os.remove(TEMP_MAL_FILE)
    return jsonify({
        "input_text": user_text,
        "generated_mal_code": mal_code,
        "execution_output": output
    })
if __name__ == '__main__':
    print("🚀 بدء تشغيل خادم MAL على المنفذ 5000...")
    app.run(host='0.0.0.0', port=5000, debug=False)
