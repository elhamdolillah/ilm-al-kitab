#!/usr/bin/env python3
import torch
import json
print("📦 جاري استخراج الأوزان من MHLJ_Fusion_Head.pt...")
# تحميل الأوزان
weights = torch.load("MHLJ_Fusion_Head.pt", map_location="cpu", weights_only=True)
mal_friendly_weights = {}
for key, tensor in weights.items():
    # تحويل التنسور إلى قائمة بايثون عادية
    # ملاحظة: للسرعة في MAL، نحولها إلى قائمة مسطحة 1D
    mal_friendly_weights[key] = {
        "shape": list(tensor.shape),
        "data": tensor.flatten().tolist()
    }
# حفظ كـ JSON
with open("fusion_head_weights.json", "w") as f:
    json.dump(mal_friendly_weights, f)
print(f"✅ تم الحفظ في fusion_head_weights.json")
print(f"📊 عدد الطبقات: {len(mal_friendly_weights)}")
