import json
class HybridTokenizer:
    def __init__(self):
        # MAL tokens (72 chars)
        self.mal_tokens = list("+-*/≔∀∃∧∨¬=<>·%()[]{}:;,.!?@#&|_\\n\\t ")
        # Code tokens (50 chars إضافية)
        self.code_tokens = list("importdefreturn\"'r\\wWsSdDabcefghijklmnopqrstuvwxyz0123456789")
        # Combined vocab
        self.vocab = sorted(list(set(self.mal_tokens + self.code_tokens)))
        self.vocab_size = len(self.vocab)
        self.char2idx = {c: i for i, c in enumerate(self.vocab)}
        self.idx2char = {i: c for i, c in enumerate(self.vocab)}
        print(f"✅ Hybrid tokenizer: {self.vocab_size} chars")
        print(f"   • MAL: {len(self.mal_tokens)}")
        print(f"   • Code: {len(self.code_tokens)}")
    def encode(self, text, max_len=256):
        return [self.char2idx.get(c, 0) for c in text[:max_len]]
    def decode(self, ids):
        return ''.join([self.idx2char.get(i, '?') for i in ids])
if __name__ == '__main__':
    tok = HybridTokenizer()
    # اختبار
    test = "import re\nre.findall(r'@', 'a@b.com')"
    ids = tok.encode(test)
    decoded = tok.decode(ids)
    print(f"\nOriginal: {test[:50]}...")
    print(f"Decoded:  {decoded[:50]}...")
    print(f"Match: {test[:50] == decoded[:50]}")
    # حفظ
    with open('regex_tiny/hybrid_tokenizer.json', 'w', encoding='utf-8') as f:
        json.dump({
            'vocab': tok.vocab,
            'vocab_size': tok.vocab_size
        }, f, indent=2, ensure_ascii=False)
    print("\n📁 Saved: hybrid_tokenizer.json")
