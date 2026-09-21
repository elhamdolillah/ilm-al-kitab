# MAL-Tiny v1.0
## Overview
A 212,864-parameter transformer model for Mathematical Arabic Language (MAL).
## Results
- Accuracy: 80% (8/10 exact matches)
- Training Time: 1 minute on 2-core CPU
- Hardware: CPU only, no GPU required
- Dataset: 203 verified MAL examples
## Quick Start
### Installation
cd MAL/examples/ai/mal_tiny
source venv/bin/activate
### Training
python3 train.py
### Evaluation
python3 evaluate_detailed.py
## Model Architecture
- 4 Transformer layers
- 4 attention heads
- 64 embedding dimension
- ~70 token vocabulary
## Files
- model.py - Model definition
- train.py - Training script
- evaluate_detailed.py - Evaluation script
- MAL-Tiny-Baseline-80pct.pt - Best model (80% accuracy)
- RESEARCH_PAPER.md - Scientific report
## Dataset
Located in ../dataset/:
- 203 verified MAL examples
- 16 categories
- 100% compile rate via malc-native
