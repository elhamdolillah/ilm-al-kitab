import json
import re
class RealPairGenerator:
    def __init__(self):
        self.pairs = []
    def generate_email_pairs(self, n=20):
        """توليد 20 pair حقيقي"""
        inputs = [
            "Contact: user@example.com",
            "Email: john.doe@company.org",
            "Multiple: a@b.com and c@d.org",
            "No emails here",
            "Complex: user.name+tag@subdomain.example.co.uk",
            "Short: x@y.com",
            "Long: very.long.email@subdomain.example.com",
            "Mixed: text1 a@b.com text2 c@d.org text3",
            "Invalid: not-an-email",
            "Valid: test@example.co.uk",
        ]
        for inp in inputs[:n]:
            output = re.findall(r'[\w\.-]+@[\w\.-]+\.\w+', inp)
            self.pairs.append({
                'prompt': f"# extract emails\nInput: {inp}\n",
                'completion': str(output),
                'category': 'extract_email',
                'verified': True
            })
        return self.pairs
    def generate_url_pairs(self, n=20):
        """توليد 20 URL pair"""
        inputs = [
            "Visit https://example.com",
            "URL: http://test.org/page",
            "Multiple: https://a.com and http://b.org",
            "No URLs",
            "Complex: https://sub.domain.com:8080/path?q=1",
            "Short: http://x.com",
            "Long: https://www.example.com/very/long/path",
            "Mixed: text1 https://a.com text2 http://b.org",
            "Invalid: not-a-url",
            "Valid: ftp://files.example.com/data",
        ]
        for inp in inputs[:n]:
            output = re.findall(r'https?://[^\s]+', inp)
            self.pairs.append({
                'prompt': f"# extract URLs\nInput: {inp}\n",
                'completion': str(output),
                'category': 'extract_url',
                'verified': True
            })
        return self.pairs
    def generate_all(self, n_per_task=20):
        """توليد كل المهام"""
        self.generate_email_pairs(n_per_task)
        self.generate_url_pairs(n_per_task)
        print(f"✅ Generated {len(self.pairs)} real pairs")
        # حفظ
        with open('regex_tiny/real_pairs.jsonl', 'w', encoding='utf-8') as f:
            for pair in self.pairs:
                f.write(json.dumps(pair, ensure_ascii=False) + '\n')
        print(f"📁 Saved: real_pairs.jsonl")
        return self.pairs
if __name__ == '__main__':
    gen = RealPairGenerator()
    gen.generate_all(n_per_task=20)
