#!/usr/bin/env python3
import os
import sys
import subprocess
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
    mal_code = "// لم يتم التوليد"
    output = ""
    try:
        original_cwd = os.getcwd()
        os.chdir(GENERATOR_DIR)
        sys.path.insert(0, GENERATOR_DIR)
        from mal_code_generator_v4 import MALCodeGeneratorV4
        generator = MALCodeGeneratorV4()
        mal_code = generator.generate(user_text, save_to_file=False)
        os.chdir(original_cwd)
        with open(TEMP_MAL_FILE, "w", encoding="utf-8") as f:
            f.write(mal_code)
        malc_bin = os.path.join(PROJECT_DIR, "MAL/src/cli/target/release/malc")
        temp_bin = os.path.join(PROJECT_DIR, "temp_mal_binary")
        cmd = [malc_bin, TEMP_MAL_FILE, "-o", temp_bin]
        build_result = subprocess.run(cmd, cwd=PROJECT_DIR, capture_output=True, text=True, timeout=30)
        if build_result.returncode == 0 and os.path.exists(temp_bin):
            run_result = subprocess.run([temp_bin], cwd=PROJECT_DIR, capture_output=True, text=True, timeout=10)
            output = run_result.stdout.strip()
            if run_result.returncode != 0:
                output += "\n" + run_result.stderr.strip()
            os.remove(temp_bin)
        else:
            output = "❌ خطأ في التجميع:\n" + build_result.stderr.strip()
    except Exception as e:
        output = "❌ خطأ: " + str(e)
    finally:
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
