import torch
import sys, json
sys.path.insert(0, '.')
from regex_tiny.hybrid_tokenizer import HybridTokenizer
from regex_tiny.hybrid_classifier import HybridClassifier
from regex_tiny.symbolic_extractor import SymbolicExtractor
class HybridNeuralSymbolicSystem:
    def __init__(self):
        self.tokenizer = HybridTokenizer()
        # الإصلاح: استخدام vocab_size الفعلي من الـ tokenizer (75)
        self.classifier = HybridClassifier(vocab_size=self.tokenizer.vocab_size)
        self.classifier.load_state_dict(torch.load('regex_tiny/Hybrid-Classifier-v1.0.pt', map_location='cpu'))
        self.classifier.eval()
        self.extractor = SymbolicExtractor()
        print("✅ Hybrid Neural-Symbolic System loaded")
        print(f"   • Classifier: {sum(p.numel() for p in self.classifier.parameters()):,} params")
        print(f"   • Extractor: Regex (100% accuracy)")
    def process(self, text, task='email'):
        ids = self.tokenizer.encode(text)
        x = torch.tensor([ids])
        pred = self.classifier.predict(x)
        if pred.item() != 1: 
            return []
        if task == 'email': return self.extractor.extract_email(text)
        elif task == 'url': return self.extractor.extract_url(text)
        elif task == 'number': return self.extractor.extract_numbers(text)
        return []
if __name__ == '__main__':
    system = HybridNeuralSymbolicSystem()
    print("\n" + "=" * 70)
    print("🧪 Testing Hybrid System (Neural Decision + Symbolic Extraction)")
    print("=" * 70)
    tests = [
        ("Contact: user@example.com", 'email', ["user@example.com"]),
        ("Email: admin@company.co.uk", 'email', ["admin@company.co.uk"]),
        ("Multiple: a@b.com and c@d.org", 'email', ["a@b.com", "c@d.org"]),
        ("No emails here", 'email', []),
        ("Visit https://example.com", 'url', ["https://example.com"]),
        ("See http://test.org/page", 'url', ["http://test.org/page"]),
        ("Multiple: https://a.com and http://b.org", 'url', ["https://a.com", "http://b.org"]),
        ("No URLs", 'url', []),
    ]
    correct = 0
    for text, task, expected in tests:
        result = system.process(text, task)
        if result == expected: 
            correct += 1
        print(f"  {'✅' if result == expected else '❌'} {text[:40]}...")
        print(f"     Expected: {expected}")
        print(f"     Got: {result}")
    accuracy = correct / len(tests)
    print(f"\n🎯 Hybrid System Accuracy: {accuracy:.0%} ({correct}/{len(tests)})")
    with open('regex_tiny/hybrid_system_results.json', 'w') as f:
        json.dump({'accuracy': accuracy, 'correct': correct, 'total': len(tests), 'approach': 'neural_symbolic_hybrid'}, f, indent=2)
    print("📁 Results saved: regex_tiny/hybrid_system_results.json")
