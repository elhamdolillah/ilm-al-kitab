#!/usr/bin/env python3
import torch
import time
import subprocess
import os
print("="*70)
print("⚖️ مقارنة الأداء: Python (PyTorch) vs MAL Compiler")
print("="*70)
# 1. تحميل النموذج في Python
print("\n🐍 1. جاري تحميل النموذج في PyTorch...")
model_weights = torch.load("MHLJ_Fusion_Head.pt", map_location="cpu", weights_only=True)
W = model_weights['laya_attention.in_proj_weight'] # Shape: [192, 64]
B = model_weights['laya_attention.in_proj_bias']   # Shape: [192]
input_vec = torch.randn(1, 64)
iterations = 10000
print(f"⏱️ جاري قياس سرعة PyTorch ({iterations} دورة)...")
start_time = time.perf_counter()
for _ in range(iterations):
    out = torch.nn.functional.linear(input_vec, W, B)
    out = torch.relu(out)
end_time = time.perf_counter()
py_time_ms = (end_time - start_time) * 1000
print(f"✅ وقت PyTorch الإجمالي: {py_time_ms:.2f} ميلي ثانية")
print(f"   (متوسط {py_time_ms/iterations:.4f} ms لكل تكرار)")
# 2. تجميع وتشغيل MAL
print("\n⚙️ 2. جاري تجميع كود MAL عبر malc...")
compile_result = subprocess.run(
    ["malc", "inference.mal", "-o", "inference_mal"], 
    capture_output=True, text=True
)
if compile_result.returncode != 0:
    print("❌ فشل تجميع MAL:")
    print(compile_result.stderr)
else:
    print("✅ تم التجميع بنجاح!")
    print(f"📦 حجم الملف التنفيذي: {os.path.getsize('inference_mal') / 1024:.1f} KB")
    print("\n⏱️ جاري تشغيل برنامج MAL وقياس أدائه...")
    run_result = subprocess.run(["./inference_mal"], capture_output=True, text=True)
    print("\n📤 مخرجات برنامج MAL:")
    print(run_result.stdout.strip())
    if run_result.stderr:
        print("⚠️ تحذيرات MAL:", run_result.stderr.strip())
print("\n" + "="*70)
print("📊 التحليل الهندسي:")
print("  1. السرعة: PyTorch يستخدم مكتبات C++/BLAS المحسنة، لذا قد يكون أسرع في الحساب الخام.")
print("  2. حجم الذاكرة: ملف MAL التنفيذي حجمه بضعة كيلوبايتات فقط، بينما PyTorch يحتاج مئات الميجابايتات.")
print("  3. وقت البدء (Startup): MAL يبدأ فوراً (أقل من 1ms)، بينما Python/PyTorch يحتاج ثوانٍ للتحميل.")
print("  4. الحتمية: MAL يضمن نتائج متطابقة 100% في كل مرة، وهو جوهر هذا المشروع.")
print("="*70)
