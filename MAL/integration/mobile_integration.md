# تكامل MAL مع الهاتف (Mobile Integration)
## المكونات المطلوبة:
1. **مولد الكود الذكي**: `MAL/examples/ai/mal_tiny/mal_code_generator_v4.py`
2. **واجهة سطر الأوامر**: `malc` (check, compile, run)
3. **خادم الويب**: `server.py` (يستقبل النص، يولد الكود، ينفذه، ويعيد النتيجة)
## خطة التنفيذ:
- **Endpoint**: `POST /generate-and-run`
- **Request**: `{"text": "احسب قيمة 15 + 25"}`
- **Response**: `{"mal_code": "...", "output": "..."}`
