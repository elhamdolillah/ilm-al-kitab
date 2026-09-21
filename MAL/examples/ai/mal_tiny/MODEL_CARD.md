# MAL-Tiny v1.0 Model Card
## Model Details
- Name: MAL-Tiny
- Version: 1.0
- Parameters: 212,864
- Architecture: Transformer (4 layers, 4 heads, 64 dim)
- Vocabulary Size: ~70 tokens
- Context Length: 128 tokens
## Training Details
- Dataset: 182 MAL examples (train), 21 (eval)
- Optimizer: AdamW (lr=3e-4, weight_decay=0.01)
- Batch Size: 4
- Epochs: 20 (optimal)
- Hardware: 2-core CPU, 4GB RAM
- Training Time: ~1 minute
## Performance
- Accuracy: 80% (8/10 exact matches)
- Eval Loss: 1.3966
- Compile Rate: 80%
## Intended Use
- Research on tiny language models
- MAL language code generation
- Educational purposes
## Limitations
- Small vocabulary (~70 tokens)
- Limited dataset (203 examples)
- Hallucination on some inputs
- No control flow support
