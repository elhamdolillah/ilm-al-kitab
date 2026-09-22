#!/usr/bin/env python3
"""
MAL-Hybrid Headless Flight Search (Improved & Robust)
Usage: python search_flights.py "أريد السفر من DXB إلى LHR يوم 2024-03-15"
"""
import sys
import asyncio
import re
from playwright.async_api import async_playwright
class FlightSearchExtractor:
    def __init__(self):
        self.patterns = {
            'airport_code': r'\b[A-Z]{3}\b',
            'date': r'\d{4}[-/]\d{1,2}[-/]\d{1,2}|\d{1,2}[-/]\d{1,2}[-/]\d{4}',
        }
    def extract(self, text):
        results = {}
        for name, pattern in self.patterns.items():
            matches = re.findall(pattern, text)
            if matches:
                results[name] = matches
        return results
class HeadlessFlightSearch:
    def __init__(self):
        self.extractor = FlightSearchExtractor()
        print("✅ نظام البحث الآلي المحسّن جاهز")
    async def search(self, query):
        print(f"\n🔍 جاري تحليل الطلب: '{query}'")
        entities = self.extractor.extract(query)
        airports = entities.get('airport_code', [])
        dates = entities.get('date', [])
        origin = airports[0] if len(airports) > 0 else "DXB"
        destination = airports[1] if len(airports) > 1 else "LHR"
        flight_date = dates[0] if len(dates) > 0 else "2024-12-31"
        print(f"✈️  المسار المستهدف: {origin} ➔ {destination}")
        print(f"📅 التاريخ المستهدف: {flight_date}")
        print("⏳ جاري تشغيل المتصفح الخفي والاتصال بـ Skyscanner...")
        async with async_playwright() as p:
            # إطلاق المتصفح مع خيارات لتجنب كشف الأتمتة
            browser = await p.chromium.launch(
                headless=True,
                args=['--disable-blink-features=AutomationControlled']
            )
            page = await browser.new_page()
            try:
                print("🌐 فتح موقع Skyscanner...")
                await page.goto('https://www.skyscanner.net', timeout=30000)
                # انتظار ظهور نموذج البحث الرئيسي
                await page.wait_for_selector('form', timeout=10000)
                await page.wait_for_timeout(3000) # وقت إضافي لتحميل المحتوى الديناميكي
                print("✅ تم تحميل الصفحة بنجاح")
                # محاولة إغلاق نافذة الكوكيز إن وجدت
                try:
                    await page.click('button:has-text("Accept")', timeout=2000)
                except:
                    pass
                # 1. ملء المغادرة والضغط على Enter لاختيار من القائمة
                origin_input = await page.query_selector('input[name="origin"]') or await page.query_selector('[data-test-id="origin"]')
                if origin_input:
                    await origin_input.click()
                    await page.wait_for_timeout(500)
                    await origin_input.fill(origin)
                    await page.wait_for_timeout(1000)
                    await origin_input.press('Enter')
                    await page.wait_for_timeout(1000)
                    print(f"   ✅ تم ملء المغادرة: {origin}")
                # 2. ملء الوصول والضغط على Enter
                dest_input = await page.query_selector('input[name="destination"]') or await page.query_selector('[data-test-id="destination"]')
                if dest_input:
                    await dest_input.click()
                    await page.wait_for_timeout(500)
                    await dest_input.fill(destination)
                    await page.wait_for_timeout(1000)
                    await dest_input.press('Enter')
                    await page.wait_for_timeout(1000)
                    print(f"   ✅ تم ملء الوصول: {destination}")
                # 3. ملء التاريخ
                date_input = await page.query_selector('input[type="date"]') or await page.query_selector('[data-test-id="date-picker"]')
                if date_input:
                    await date_input.fill(flight_date)
                    await page.wait_for_timeout(1000)
                    print(f"   ✅ تم تحديد التاريخ: {flight_date}")
                # 4. البحث عن زر البحث بعدة طرق بديلة
                search_btn = None
                selectors = [
                    'button[data-test-id="search-button"]',
                    'button[type="submit"]',
                    'button:has-text("Search")',
                    'button:has-text("ابحث")',
                    'button[data-testid="search-button"]'
                ]
                for sel in selectors:
                    try:
                        search_btn = await page.query_selector(sel)
                        if search_btn:
                            break
                    except:
                        continue
                if search_btn:
                    print("   🔍 جاري الضغط على زر البحث وانتظار تحميل النتائج...")
                    await search_btn.click()
                    await page.wait_for_timeout(8000) # انتظار تحميل نتائج البحث
                    print("\n" + "="*60)
                    print("📋 نتائج الرحلات المستخرجة:")
                    print("="*60)
                    flight_cards = await page.query_selector_all('[data-test-id="list-card"]')
                    if flight_cards:
                        print(f"\n✈️  تم العثور على {len(flight_cards)} رحلة (عرض أول 5):\n")
                        for i, card in enumerate(flight_cards[:5], 1):
                            try:
                                airline_el = await card.query_selector('[data-test-id="airline-name"]')
                                price_el = await card.query_selector('[data-test-id="price"]')
                                time_el = await card.query_selector('[data-test-id="departure-time"]')
                                airline = (await airline_el.inner_text()).strip() if airline_el else "غير معروف"
                                price = (await price_el.inner_text()).strip() if price_el else "N/A"
                                time = (await time_el.inner_text()).strip() if time_el else "N/A"
                                print(f"{i}. ✈️ {airline}")
                                print(f"   ⏰ الوقت: {time}")
                                print(f"   💰 السعر: {price}")
                                print("-" * 40)
                            except Exception:
                                pass
                    else:
                        print("⚠️  لم يتم العثور على بطاقات رحلات (قد يكون الموقع يطلب تحققاً أمنياً Captcha).")
                        print("💡 تلميح: تم حفظ لقطة شاشة، يمكنك فحصها لرؤية ما ظهر بالضبط.")
                    await page.screenshot(path='flight_results.png', full_page=True)
                    print("\n💾 تم حفظ لقطة شاشة للنتائج باسم: flight_results.png")
                else:
                    print("⚠️  لم يتم العثور على زر البحث بأي من الطرق المعروفة.")
                    await page.screenshot(path='flight_results_error.png', full_page=True)
                    print("💾 تم حفظ لقطة شاشة للحالة الحالية باسم: flight_results_error.png")
            except Exception as e:
                print(f"❌ حدث خطأ أثناء التنفيذ: {e}")
            finally:
                await browser.close()
        print("="*60)
        print("✅ انتهت عملية البحث الآلي!")
        print("="*60)
async def main():
    if len(sys.argv) < 2:
        print("طريقة الاستخدام: python search_flights.py \"نص البحث\"")
        print("مثال: python search_flights.py \"أريد السفر من DXB إلى LHR يوم 2024-03-15\"")
        sys.exit(1)
    query = ' '.join(sys.argv[1:])
    search = HeadlessFlightSearch()
    await search.search(query)
if __name__ == '__main__':
    asyncio.run(main())
