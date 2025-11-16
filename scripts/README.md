# Eidos Model Scripts

This directory contains scripts for training, validating, and converting AI models for Eidos.

## Scripts Overview

### train_model.py
Trains a sequence-to-sequence model to translate natural language to shell commands.

**Usage:**
```bash
./train_model.py datasets/example_commands.jsonl -o ./my-model

# With custom settings
./train_model.py data.jsonl \
  --model t5-base \
  --epochs 5 \
  --batch-size 16 \
  --learning-rate 1e-4
```

**Options:**
- `-o, --output`: Output directory (default: ./eidos-model)
- `-m, --model`: Base model to fine-tune (default: t5-small)
- `-e, --epochs`: Number of training epochs (default: 3)
- `-b, --batch-size`: Training batch size (default: 8)
- `-l, --learning-rate`: Learning rate (default: 3e-4)
- `--no-validation`: Skip validation split

### validate_model.py
Validates a trained model for accuracy and safety.

**Usage:**
```bash
./validate_model.py ./my-model test_data.jsonl

# Verbose output
./validate_model.py ./my-model test_data.jsonl -v
```

**Exit Codes:**
- `0`: Success (>80% accuracy, no safety issues)
- `1`: Low accuracy (<80%)
- `2`: Safety failures detected

**Output:**
- Prints accuracy and safety summary
- Saves detailed results to `validation_results.json`

### convert_to_onnx.py
Converts trained models to ONNX format for deployment.

**Usage:**
```bash
./convert_to_onnx.py ./my-model -o model.onnx

# Skip simplification (faster, but larger file)
./convert_to_onnx.py ./my-model --no-simplify
```

**Output:**
- `model.onnx`: ONNX model file
- `tokenizer/`: Tokenizer configuration directory

### prune_dataset.py
*(Existing script for dataset preprocessing)*

## Complete Workflow

### 1. Prepare Training Data

Create a JSONL file with prompt-command pairs:

```json
{"prompt": "list all files", "command": "ls -la"}
{"prompt": "show current directory", "command": "pwd"}
```

See `../datasets/example_commands.jsonl` for 100+ examples.

### 2. Train Model

```bash
./train_model.py datasets/my_training_data.jsonl -o ./eidos-custom-model
```

This will:
- Fine-tune T5-small (or your chosen base model)
- Save checkpoints every 500 steps
- Create validation split (10% by default)
- Save final model to `./eidos-custom-model/final_model`

**Training Time:**
- 1000 examples on CPU: ~2-4 hours
- 1000 examples on GPU: ~10-20 minutes

### 3. Validate Model

```bash
./validate_model.py ./eidos-custom-model/final_model test_cases.jsonl -v
```

Review the output for:
- **Accuracy**: Should be >80% for production use
- **Safety failures**: Should be 0 (model never generates dangerous commands)

If accuracy is low:
- Add more diverse training examples
- Train for more epochs
- Try a larger base model (t5-base instead of t5-small)

### 4. Convert to ONNX

```bash
./convert_to_onnx.py ./eidos-custom-model/final_model -o model.onnx
```

This creates:
- `model.onnx`: Optimized model for tract runtime
- `tokenizer/tokenizer.json`: Tokenizer configuration

### 5. Deploy to Eidos

```bash
# Copy files to Eidos directory
cp model.onnx ~/eidos/
cp tokenizer/tokenizer.json ~/eidos/

# Configure Eidos
cat > ~/eidos/eidos.toml <<EOF
model_path = "model.onnx"
tokenizer_path = "tokenizer.json"
EOF

# Test it
cd ~/eidos
eidos core "list all files"
```

## Requirements

Install Python dependencies:

```bash
pip install transformers datasets torch sentencepiece onnx onnx-simplifier tensorboard
```

**Recommended versions:**
- Python 3.8+
- PyTorch 2.0+
- Transformers 4.30+

## Tips

### GPU Acceleration

To speed up training with GPU:

```bash
# Check if CUDA is available
python -c "import torch; print(torch.cuda.is_available())"

# Training will automatically use GPU if available
./train_model.py data.jsonl
```

### Monitoring Training

Training progress is logged to TensorBoard:

```bash
# In another terminal
tensorboard --logdir ./eidos-model/logs

# Open http://localhost:6006 in browser
```

### Data Augmentation

To improve model generalization, augment your training data:

```python
# Example: Add variations
original = {"prompt": "list files", "command": "ls"}

variations = [
    {"prompt": "show files", "command": "ls"},
    {"prompt": "display files", "command": "ls"},
    {"prompt": "what files are here", "command": "ls"},
]
```

### Testing Before Deployment

Always test on a diverse set of prompts:

```bash
# Create test file
cat > test_prompts.txt <<EOF
list all files
show current directory
find Python files
count lines in README
EOF

# Test each prompt
while read prompt; do
  echo "Prompt: $prompt"
  eidos core "$prompt"
  echo
done < test_prompts.txt
```

## Troubleshooting

### Out of Memory

- Reduce `--batch-size` (try 4 or 2)
- Use smaller model (distilgpt2, t5-small)
- Reduce `max_length` in training script

### Slow Training

- Use GPU if available
- Increase `--batch-size` (if memory allows)
- Reduce dataset size for initial experiments

### Poor Accuracy

- Increase training data (aim for 10k+ examples)
- Train longer (more epochs)
- Use larger model (t5-base)
- Check data quality (are commands correct?)

### Safety Failures

- Review training data for dangerous commands
- Remove any destructive examples (rm, dd, mkfs, etc.)
- Retrain from scratch with clean data

## See Also

- [Model Guide](../docs/MODEL_GUIDE.md) - Comprehensive model documentation
- [Example Dataset](../datasets/example_commands.jsonl) - Starter training data
- [Eidos README](../README.md) - Project overview
