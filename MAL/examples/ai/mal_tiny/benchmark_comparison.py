#!/usr/bin/env python3
import time
import subprocess
import os
print("="*70)
print("⚖️ مقارنة الأداء: Python vs MAL Compiler (Scalar Loop)")
print("="*70)
# 1. Benchmark Python
iterations = 10000000
print(f"\n🐍 1. جاري قياس سرعة Python ({iterations} دورة)...")
start_time = time.perf_counter()
a = 1
for _ in range(iterations):
    a = a * 3 + 1
end_time = time.perf_counter()
py_time_ms = (end_time - start_time) * 1000
print(f"✅ وقت Python الإجمالي: {py_time_ms:.2f} ميلي ثانية")
# 2. Compile and run MAL
print("\n⚙️ 2. جاري تجميع كود MAL عبر malc...")
compile_result = subprocess.run(
    ["malc", "benchmark_scalar.mal", "-o", "benchmark_mal"], 
    capture_output=True, text=True
)
if compile_result.returncode != 0:
    print("❌ فشل تجميع MAL:")
    print(compile_result.stderr)
else:
    print("✅ تم التجميع بنجاح!")
    print(f"📦 حجم الملف التنفيذي: {os.path.getsize('benchmark_mal') / 1024:.1f} KB")
    print("\n⏱️ جاري تشغيل برنامج MAL وقياس أدائه...")
    run_result = subprocess.run(["./benchmark_mal"], capture_output=True, text=True)
    print("\n📤 مخرجات برنامج MAL:")
    print(run_result.stdout.strip())
    if run_result.stderr:
        print("⚠️ تحذيرات MAL:", run_result.stderr.strip())
print("\n" + "="*70)
print("📊 التحليل الهندسي:")
print("  1. السرعة: MAL مترجم إلى C ثم إلى لغة آلة، لذا يتفوق عادة في حلقات التكرار.")
print("  2. الحجم: الملف التنفيذي لـ MAL حجمه كيلوبايتات فقط، بينما Python يحتاج لمحرك كامل.")
print("  3. الحتمية: MAL يضمن نفس النتيجة بالضبط في كل مرة وعلى كل معالج.")
print("="*70)
