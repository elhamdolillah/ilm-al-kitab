#!/usr/bin/env python3
from typing import List, Dict
from typing import List, Dict
import sys, os
sys.path.insert(0, '/root/ilm-al-kitab/MAL/examples/ai/mal_tiny')
from integrated_engine import IntegratedEngine
from advanced_cache_and_constrained import AdvancedIntelligenceEngine
class MALCodeGenerator:
    def __init__(self, user_id: str = "user_001"):
        self.user_id = user_id
        self.engine = IntegratedEngine(user_id=user_id)
        self.advanced_cache = AdvancedIntelligenceEngine()
    def generate_mal_code(self, text: str) -> dict:
        # 1. التخزين المؤقت الدلالي (الأسرع والأوفر للموارد)
        cached = self.advanced_cache.try_semantic_cache(text)
        if cached:
            print("⚡ تم الاسترجاع من التخزين المؤقت الدلالي (توفير 100% من CPU/RAM)")
            return {**cached, 'disclaimer': ""}
        # 2. إذا لم يكن في الذاكرة، نستخدم المحرك الذكي
        print("🧠 جاري التحليل بالمحرك اللغوي المتخصص...")
        result = self.engine.process_query(text)
        region = result.get('compressed_sequence', 'GENERAL').split(' -> ')[0]
        domain = result.get('discovered_domains', ['عام'])[0] if result.get('discovered_domains') else 'عام'
        # 3. التوليد المقيد بالقواعد (Grammar-Constrained) لضمان الصحة النحوية
        mal_code = self.advanced_cache.generate_constrained_code(text, region, domain)
        return {
            'mal_code': mal_code,
            'region': region,
            'certainty': result['certainty_percentage'],
            'is_deterministic': result['is_deterministic'],
            'disclaimer': result['disclaimer'],
            'domain': domain,
            'from_cache': False
        }
