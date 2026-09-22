#!/usr/bin/env python3
"""
Multi-Airline Flight Search
Searches: Royal Air Maroc, Emirates, Qatar Airways, flynas
Route: CMN (Casablanca) → MED (Madinah)
Dates: 15/02/2027 → 17/03/2027
"""
import sys
import asyncio
import re
from playwright.async_api import async_playwright
class HeadlessFlightSearch:
    def __init__(self):
        self.origin = "CMN"
        self.destination = "MED"
        self.departure_date = "2027-02-15"
        self.return_date = "2027-03-17"
        print("✅ نظام البحث متعدد الشركات جاهز")
        print(f"✈️  المسار: {self.origin} ➔ {self.destination}")
        print(f"📅 الذهاب: {self.departure_date} | العودة: {self.return_date}")
    async def search_royal_air_maroc(self, page):
        """البحث في موقع الملكية المغربية"""
        print("\n" + "="*60)
        print("🇲🇦 البحث في Royal Air Maroc...")
        print("="*60)
        try:
            await page.goto('https://www.royalairmaroc.com', timeout=45000)
            await page.wait_for_load_state('domcontentloaded')
            await page.wait_for_timeout(5000)
            print("✅ تم تحميل الصفحة")
            # محاولة إغلاق الكوكيز
            try:
                await page.click('button:has-text("Accept")', timeout=3000)
            except:
                pass
            # البحث عن حقول الإدخال
            origin_input = await page.query_selector('input[name*="origin"], input[placeholder*="Departure"], input[id*="origin"]')
            if origin_input:
                await origin_input.fill(self.origin)
                await page.wait_for_timeout(1000)
                await origin_input.press('Enter')
                print(f"   ✅ المغادرة: {self.origin}")
            dest_input = await page.query_selector('input[name*="destination"], input[placeholder*="Arrival"], input[id*="destination"]')
            if dest_input:
                await dest_input.fill(self.destination)
                await page.wait_for_timeout(1000)
                await dest_input.press('Enter')
                print(f"   ✅ الوصول: {self.destination}")
            # التاريخ
            date_input = await page.query_selector('input[type="date"], input[name*="date"]')
            if date_input:
                await date_input.fill(self.departure_date)
                print(f"   ✅ التاريخ: {self.departure_date}")
            # زر البحث
            search_btn = await page.query_selector('button[type="submit"], button:has-text("Search"), button:has-text("Rechercher")')
            if search_btn:
                print("   🔍 جاري البحث...")
                await search_btn.click()
                await page.wait_for_timeout(8000)
                await page.screenshot(path='royal_air_maroc_results.png', full_page=True)
                print("   💾 تم حفظ: royal_air_maroc_results.png")
                return True
            else:
                print("   ⚠️  لم يتم العثور على زر البحث")
        except Exception as e:
            print(f"   ❌ خطأ: {str(e)[:100]}")
        await page.screenshot(path='royal_air_maroc_debug.png', full_page=True)
        return False
    async def search_emirates(self, page):
        """البحث في موقع طيران الإمارات"""
        print("\n" + "="*60)
        print("🇦🇪 البحث في Emirates...")
        print("="*60)
        try:
            await page.goto('https://www.emirates.com', timeout=45000)
            await page.wait_for_load_state('domcontentloaded')
            await page.wait_for_timeout(5000)
            print("✅ تم تحميل الصفحة")
            try:
                await page.click('button:has-text("Accept")', timeout=3000)
            except:
                pass
            origin_input = await page.query_selector('input[name*="origin"], input[placeholder*="From"]')
            if origin_input:
                await origin_input.fill(self.origin)
                await page.wait_for_timeout(1000)
                await origin_input.press('Enter')
                print(f"   ✅ المغادرة: {self.origin}")
            dest_input = await page.query_selector('input[name*="destination"], input[placeholder*="To"]')
            if dest_input:
                await dest_input.fill(self.destination)
                await page.wait_for_timeout(1000)
                await dest_input.press('Enter')
                print(f"   ✅ الوصول: {self.destination}")
            date_input = await page.query_selector('input[type="date"]')
            if date_input:
                await date_input.fill(self.departure_date)
                print(f"   ✅ التاريخ: {self.departure_date}")
            search_btn = await page.query_selector('button[type="submit"], button:has-text("Search")')
            if search_btn:
                print("   🔍 جاري البحث...")
                await search_btn.click()
                await page.wait_for_timeout(8000)
                await page.screenshot(path='emirates_results.png', full_page=True)
                print("   💾 تم حفظ: emirates_results.png")
                return True
            else:
                print("   ⚠️  لم يتم العثور على زر البحث")
        except Exception as e:
            print(f"   ❌ خطأ: {str(e)[:100]}")
        await page.screenshot(path='emirates_debug.png', full_page=True)
        return False
    async def search_qatar_airways(self, page):
        """البحث في موقع الخطوط القطرية"""
        print("\n" + "="*60)
        print("🇶🇦 البحث في Qatar Airways...")
        print("="*60)
        try:
            await page.goto('https://www.qatarairways.com', timeout=45000)
            await page.wait_for_load_state('domcontentloaded')
            await page.wait_for_timeout(5000)
            print("✅ تم تحميل الصفحة")
            try:
                await page.click('button:has-text("Accept")', timeout=3000)
            except:
                pass
            origin_input = await page.query_selector('input[name*="origin"], input[placeholder*="From"]')
            if origin_input:
                await origin_input.fill(self.origin)
                await page.wait_for_timeout(1000)
                await origin_input.press('Enter')
                print(f"   ✅ المغادرة: {self.origin}")
            dest_input = await page.query_selector('input[name*="destination"], input[placeholder*="To"]')
            if dest_input:
                await dest_input.fill(self.destination)
                await page.wait_for_timeout(1000)
                await dest_input.press('Enter')
                print(f"   ✅ الوصول: {self.destination}")
            date_input = await page.query_selector('input[type="date"]')
            if date_input:
                await date_input.fill(self.departure_date)
                print(f"   ✅ التاريخ: {self.departure_date}")
            search_btn = await page.query_selector('button[type="submit"], button:has-text("Search")')
            if search_btn:
                print("   🔍 جاري البحث...")
                await search_btn.click()
                await page.wait_for_timeout(8000)
                await page.screenshot(path='qatar_airways_results.png', full_page=True)
                print("   💾 تم حفظ: qatar_airways_results.png")
                return True
            else:
                print("   ⚠️  لم يتم العثور على زر البحث")
        except Exception as e:
            print(f"   ❌ خطأ: {str(e)[:100]}")
        await page.screenshot(path='qatar_airways_debug.png', full_page=True)
        return False
    async def search_flynas(self, page):
        """البحث في موقع فالي ناس"""
        print("\n" + "="*60)
        print("🇸🇦 البحث في flynas...")
        print("="*60)
        try:
            await page.goto('https://www.flynas.com', timeout=45000)
            await page.wait_for_load_state('domcontentloaded')
            await page.wait_for_timeout(5000)
            print("✅ تم تحميل الصفحة")
            try:
                await page.click('button:has-text("Accept")', timeout=3000)
            except:
                pass
            origin_input = await page.query_selector('input[name*="origin"], input[placeholder*="From"]')
            if origin_input:
                await origin_input.fill(self.origin)
                await page.wait_for_timeout(1000)
                await origin_input.press('Enter')
                print(f"   ✅ المغادرة: {self.origin}")
            dest_input = await page.query_selector('input[name*="destination"], input[placeholder*="To"]')
            if dest_input:
                await dest_input.fill(self.destination)
                await page.wait_for_timeout(1000)
                await dest_input.press('Enter')
                print(f"   ✅ الوصول: {self.destination}")
            date_input = await page.query_selector('input[type="date"]')
            if date_input:
                await date_input.fill(self.departure_date)
                print(f"   ✅ التاريخ: {self.departure_date}")
            search_btn = await page.query_selector('button[type="submit"], button:has-text("Search")')
            if search_btn:
                print("   🔍 جاري البحث...")
                await search_btn.click()
                await page.wait_for_timeout(8000)
                await page.screenshot(path='flynas_results.png', full_page=True)
                print("   💾 تم حفظ: flynas_results.png")
                return True
            else:
                print("   ⚠️  لم يتم العثور على زر البحث")
        except Exception as e:
            print(f"   ❌ خطأ: {str(e)[:100]}")
        await page.screenshot(path='flynas_debug.png', full_page=True)
        return False
    async def search_all(self):
        """البحث في جميع الشركات"""
        print("⏳ جاري تشغيل المتصفح الخفي...")
        async with async_playwright() as p:
            user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
            browser = await p.chromium.launch(
                headless=True,
                args=['--disable-blink-features=AutomationControlled', '--no-sandbox']
            )
            context = await browser.new_context(
                user_agent=user_agent,
                viewport={'width': 1920, 'height': 1080}
            )
            await context.add_init_script("""
                Object.defineProperty(navigator, 'webdriver', { get: () => undefined });
            """)
            page = await context.new_page()
            results = {
                'royal_air_maroc': await self.search_royal_air_maroc(page),
                'emirates': await self.search_emirates(page),
                'qatar_airways': await self.search_qatar_airways(page),
                'flynas': await self.search_flynas(page)
            }
            await browser.close()
            print("\n" + "="*60)
            print("📊 ملخص النتائج:")
            print("="*60)
            for airline, success in results.items():
                status = "✅ نجح" if success else "❌ فشل (راجع الصورة)"
                print(f"  {airline}: {status}")
            print("\n💾 الملفات المولدة:")
            for airline, success in results.items():
                filename = f"{airline}_results.png" if success else f"{airline}_debug.png"
                print(f"  - {filename}")
            print("="*60)
async def main():
    search = HeadlessFlightSearch()
    await search.search_all()
if __name__ == '__main__':
    asyncio.run(main())
