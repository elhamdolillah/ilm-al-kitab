# MAL-Tiny: A 212K Parameter Transformer for MAL
## Abstract
We present MAL-Tiny, a 212,864-parameter transformer achieving 80% accuracy
on Mathematical Arabic Language (MAL) code generation, trained in 1 minute
on a 2-core CPU.
## Key Results
- Parameters: 212,864 (4 layers, 4 heads, 64 dim)
- Hardware: 2-core CPU, 4GB RAM, no GPU
- Training: 1 minute for 20 epochs
- Dataset: 203 verified MAL examples
- Accuracy: 80% (8/10 exact matches)
## The Loss-Accuracy Divergence
| Epoch | Loss  | Accuracy |
|-------|-------|----------|
| 20    | 1.40  | 80%      |
| 30    | 1.25  | ~60%     |
| 45    | 1.19  | ~40%     |
| 50    | 1.25  | 30%      |
| 100   | -     | 0%       |
## Successful Examples
- "2 · 3 == 6" → exact match
- "(5 == 6) == 0" → exact match
- "x ≔ 5\nx ≔ 10\nx" → exact match
- "power(2, 3) + power(2, 2)" → exact match
## Conclusions
1. Tiny models CAN achieve meaningful code generation
2. Early stopping based on generation quality is essential
3. Loss metrics alone are misleading for tiny models
﴿وقل رب زدني علماً﴾
