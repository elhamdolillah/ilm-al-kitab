#!/usr/bin/env python3
"""
Ultimate Resilient Flight Search
Uses Playwright's User-Facing Locators for maximum resilience against DOM changes.
"""
import sys
import asyncio
from playwright.async_api import async_playwright, TimeoutError as PlaywrightTimeout
class UltimateFlightSearch:
    def __init__(self):
        self.origin = "CMN"
        self.destination = "MED"
        self.departure_date = "2027-02-15"
        self.return_date = "2027-03-17"
        print("✅ نظام البحث المتقدم (User-Facing Locators) جاهز")
    async def _handle_cookies(self, page):
        """محاولة إغلاق نوافذ الكوكيز الشائعة"""
        cookie_selectors = [
            'button:has-text("Accept")',
            'button:has-text("Accept All")',
            'button:has-text("موافق")',
            '#onetrust-accept-btn-handler',
            'button[id*="accept"]'
        ]
        for selector in cookie_selectors:
            try:
                await page.locator(selector).first.click(timeout=3000)
                await page.wait_for_timeout(1000)
                break
            except:
                continue
    async def search_royal_air_maroc(self, page):
        print("\n" + "="*60)
        print("🇲🇦 Royal Air Maroc (أفضل خيار للمسار)")
        print("="*60)
        try:
            await page.goto('https://www.royalairmaroc.com', timeout=60000)
            await page.wait_for_load_state('networkidle')
            await self._handle_cookies(page)
            # استخدام محددات مرئية قوية
            await page.get_by_placeholder("المغادرة أو المدينة أو المطار").or_(page.get_by_placeholder("Departure")).fill(self.origin)
            await page.wait_for_timeout(1500)
            await page.keyboard.press('Enter')
            await page.wait_for_timeout(1000)
            print(f"   ✅ المغادرة: {self.origin}")
            await page.get_by_placeholder("الوصول أو المدينة أو المطار").or_(page.get_by_placeholder("Arrival")).fill(self.destination)
            await page.wait_for_timeout(1500)
            await page.keyboard.press('Enter')
            await page.wait_for_timeout(1000)
            print(f"   ✅ الوصول: {self.destination}")
            # البحث عن زر البحث باستخدام النص المرئي
            search_btn = page.get_by_role("button", name="Rechercher").or_(page.get_by_role("button", name="Search"))
            await search_btn.click(timeout=10000)
            await page.wait_for_timeout(10000) # انتظار نتائج البحث
            await page.screenshot(path='ram_results.png', full_page=True)
            print("   💾 تم الحفظ: ram_results.png")
            return True
        except Exception as e:
            print(f"   ❌ فشل: {str(e)[:80]}")
            await page.screenshot(path='ram_debug.png', full_page=True)
            return False
    async def search_emirates(self, page):
        print("\n" + "="*60)
        print("🇦🇪 Emirates")
        print("="*60)
        try:
            await page.goto('https://www.emirates.com', timeout=60000)
            await page.wait_for_load_state('networkidle')
            await self._handle_cookies(page)
            await page.get_by_placeholder("المغادرة").or_(page.get_by_placeholder("From")).fill(self.origin)
            await page.wait_for_timeout(1500)
            await page.keyboard.press('Enter')
            await page.get_by_placeholder("الوصول").or_(page.get_by_placeholder("To")).fill(self.destination)
            await page.wait_for_timeout(1500)
            await page.keyboard.press('Enter')
            search_btn = page.get_by_role("button", name="Search flights").or_(page.get_by_role("button", name="البحث عن رحلات"))
            await search_btn.click(timeout=10000)
            await page.wait_for_timeout(10000)
            await page.screenshot(path='emirates_results.png', full_page=True)
            print("   💾 تم الحفظ: emirates_results.png")
            return True
        except Exception as e:
            print(f"   ❌ فشل: {str(e)[:80]}")
            await page.screenshot(path='emirates_debug.png', full_page=True)
            return False
    async def search_all(self):
        print(f"✈️  المسار: {self.origin} ➔ {self.destination} | 📅 {self.departure_date}")
        print("⏳ جاري التشغيل...")
        async with async_playwright() as p:
            browser = await p.chromium.launch(
                headless=True,
                args=['--disable-blink-features=AutomationControlled', '--no-sandbox', '--disable-setuid-sandbox']
            )
            context = await browser.new_context(
                user_agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                viewport={'width': 1920, 'height': 1080}
            )
            await context.add_init_script("Object.defineProperty(navigator, 'webdriver', {get: () => undefined})")
            page = await context.new_page()
            results = {
                'Royal Air Maroc': await self.search_royal_air_maroc(page),
                'Emirates': await self.search_emirates(page)
            }
            await browser.close()
            print("\n" + "="*60)
            print("📊 الملخص:")
            for name, success in results.items():
                print(f"  {name}: {'✅ نجح' if success else '❌ فشل (راجع ملف debug.png)'}")
            print("="*60)
async def main():
    search = UltimateFlightSearch()
    await search.search_all()
if __name__ == '__main__':
    asyncio.run(main())
