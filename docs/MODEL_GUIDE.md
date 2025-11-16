# Eidos Model Guide

This guide explains how to train, convert, and use AI models with Eidos.

## Table of Contents

1. [Overview](#overview)
2. [Model Architecture](#model-architecture)
3. [Training Data Format](#training-data-format)
4. [Training Pipeline](#training-pipeline)
5. [ONNX Conversion](#onnx-conversion)
6. [Model Configuration](#model-configuration)
7. [Quantized Models (GGUF)](#quantized-models-gguf)
8. [Validation](#validation)

## Overview

Eidos supports two types of models:

- **ONNX Models**: Standard ONNX format for fast CPU inference via tract
- **Quantized Models**: GGUF format for memory-efficient inference via candle

The Core module uses models to translate natural language prompts into safe shell commands.

## Model Architecture

### Input Format
- **Type**: Text (natural language prompt)
- **Examples**:
  - "list all files"
  - "show current directory"
  - "find Python files"

### Output Format
- **Type**: Text (shell command)
- **Examples**:
  - "ls -la"
  - "pwd"
  - "find . -name '*.py'"

### Recommended Architectures

1. **Sequence-to-Sequence Models**
   - T5-small, T5-base
   - BART-small, BART-base
   - GPT-2 (fine-tuned)

2. **Instruction-Following Models**
   - Flan-T5
   - LLaMA (quantized)
   - Mistral (quantized)

## Training Data Format

### Dataset Structure

Training data should be in JSONL format (one JSON object per line):

```json
{"prompt": "list all files in current directory", "command": "ls -la"}
{"prompt": "show my current location", "command": "pwd"}
{"prompt": "create a new directory called test", "command": "mkdir test"}
{"prompt": "find all Python files", "command": "find . -name '*.py'"}
{"prompt": "count lines in file.txt", "command": "wc -l file.txt"}
```

### Data Guidelines

1. **Diversity**: Include various ways to express the same intent
   ```json
   {"prompt": "list files", "command": "ls"}
   {"prompt": "show files", "command": "ls"}
   {"prompt": "what files are here", "command": "ls"}
   ```

2. **Safety**: Only include safe, non-destructive commands
   - ✅ Good: `ls`, `pwd`, `find`, `cat`, `head`, `tail`
   - ❌ Bad: `rm -rf`, `dd`, `mkfs`, `chmod 777`

3. **Coverage**: Include commands across different categories
   - File operations: `ls`, `cat`, `find`, `grep`
   - System info: `pwd`, `whoami`, `date`, `df`
   - Text processing: `wc`, `sort`, `uniq`, `head`

4. **Size**: Aim for 10,000+ examples for good generalization

### Example Dataset

See `datasets/example_commands.jsonl` for a starter dataset.

## Training Pipeline

### 1. Prepare Environment

```bash
# Install dependencies
pip install transformers datasets torch sentencepiece
```

### 2. Training Script (PyTorch + Transformers)

```python
from transformers import T5Tokenizer, T5ForConditionalGeneration, Trainer, TrainingArguments
from datasets import load_dataset

# Load your dataset
dataset = load_dataset('json', data_files='training_data.jsonl')

# Load model and tokenizer
model_name = "t5-small"
tokenizer = T5Tokenizer.from_pretrained(model_name)
model = T5ForConditionalGeneration.from_pretrained(model_name)

# Preprocess
def preprocess_function(examples):
    inputs = ["translate to command: " + prompt for prompt in examples['prompt']]
    targets = examples['command']
    model_inputs = tokenizer(inputs, max_length=128, truncation=True, padding="max_length")
    labels = tokenizer(targets, max_length=64, truncation=True, padding="max_length")
    model_inputs["labels"] = labels["input_ids"]
    return model_inputs

tokenized_dataset = dataset.map(preprocess_function, batched=True)

# Training arguments
training_args = TrainingArguments(
    output_dir="./eidos-model",
    num_train_epochs=3,
    per_device_train_batch_size=8,
    save_steps=500,
    save_total_limit=2,
    logging_steps=100,
)

# Train
trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=tokenized_dataset["train"],
)

trainer.train()
trainer.save_model("./eidos-model-final")
```

### 3. Evaluation

```python
# Test the model
from transformers import pipeline

generator = pipeline('text2text-generation', model='./eidos-model-final')

test_prompts = [
    "list all files",
    "show current directory",
    "find Python files"
]

for prompt in test_prompts:
    result = generator(f"translate to command: {prompt}", max_length=64)
    print(f"Prompt: {prompt}")
    print(f"Command: {result[0]['generated_text']}")
    print()
```

## ONNX Conversion

### Converting PyTorch/Transformers to ONNX

```python
import torch
from transformers import T5Tokenizer, T5ForConditionalGeneration

# Load your trained model
model = T5ForConditionalGeneration.from_pretrained("./eidos-model-final")
tokenizer = T5Tokenizer.from_pretrained("./eidos-model-final")

# Export to ONNX
dummy_input = tokenizer("list files", return_tensors="pt")

torch.onnx.export(
    model,
    (dummy_input['input_ids'],),
    "model.onnx",
    input_names=['input_ids'],
    output_names=['output'],
    dynamic_axes={
        'input_ids': {0: 'batch_size', 1: 'sequence'},
        'output': {0: 'batch_size', 1: 'sequence'}
    },
    opset_version=14
)

# Save tokenizer separately
tokenizer.save_pretrained("./tokenizer")
```

### Optimizing ONNX Models

```bash
# Install ONNX Runtime tools
pip install onnxruntime onnx-simplifier

# Simplify the model
python -m onnxsim model.onnx model_simplified.onnx

# Quantize for faster inference (optional)
python -m onnxruntime.quantization.preprocess --input model.onnx --output model_prep.onnx
```

## Model Configuration

### Directory Structure

```
eidos/
├── model.onnx              # ONNX model file
├── tokenizer.json          # Tokenizer configuration
├── eidos.toml             # Eidos configuration
└── datasets/
    └── example_commands.jsonl
```

### Configuration File (eidos.toml)

```toml
model_path = "model.onnx"
tokenizer_path = "tokenizer.json"

[model]
max_length = 64
temperature = 0.7
top_k = 50
```

### Environment Variables

```bash
export EIDOS_MODEL_PATH=/path/to/model.onnx
export EIDOS_TOKENIZER_PATH=/path/to/tokenizer.json
```

## Quantized Models (GGUF)

### Converting to GGUF Format

For memory-efficient deployment, convert LLaMA-style models to GGUF:

```bash
# Install llama.cpp
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp
make

# Convert PyTorch model to GGUF
python convert.py /path/to/your/model --outtype f16 --outfile model.gguf

# Quantize to 4-bit (optional, for smaller size)
./quantize model.gguf model_q4.gguf Q4_0
```

### Using Quantized Models

```rust
// Eidos automatically detects GGUF files
use lib_core::QuantizedLlm;

let model = QuantizedLlm::new("model.gguf", "tokenizer.json")?;
let output = model.generate("list files", 50)?;
```

## Validation

### Model Validation Script

Create `scripts/validate_model.py`:

```python
#!/usr/bin/env python3
import json
from transformers import pipeline

# Load model
generator = pipeline('text2text-generation', model='./eidos-model-final')

# Load test cases
with open('test_cases.jsonl') as f:
    test_cases = [json.loads(line) for line in f]

# Validate
correct = 0
total = len(test_cases)

for case in test_cases:
    result = generator(f"translate to command: {case['prompt']}", max_length=64)
    predicted = result[0]['generated_text'].strip()
    expected = case['command'].strip()

    if predicted == expected:
        correct += 1
        print(f"✓ {case['prompt']}")
    else:
        print(f"✗ {case['prompt']}")
        print(f"  Expected: {expected}")
        print(f"  Got:      {predicted}")

accuracy = (correct / total) * 100
print(f"\nAccuracy: {accuracy:.2f}% ({correct}/{total})")
```

### Safety Validation

Eidos includes built-in safety checks. Test your model:

```bash
# This should be blocked by security validation
eidos core "delete all files"

# This should work
eidos core "list files"
```

All generated commands are validated against 60+ dangerous patterns before execution.

## Best Practices

1. **Start Small**: Begin with T5-small or similar (~60M parameters)
2. **Iterate**: Train, test, collect failures, retrain
3. **Safety First**: Never include destructive commands in training data
4. **Version Control**: Tag each model version with git
5. **Document**: Keep notes on training data, hyperparameters, and results

## Troubleshooting

### Model Too Large
- Use quantization (GGUF Q4)
- Try smaller base models (T5-small, distilbert)
- Reduce sequence length

### Poor Accuracy
- Increase training data (>10k examples)
- Train longer (more epochs)
- Use data augmentation
- Try different base models

### Slow Inference
- Convert to ONNX and optimize
- Use quantized models
- Reduce max_length

## Resources

- [Hugging Face Transformers](https://huggingface.co/docs/transformers)
- [ONNX Runtime](https://onnxruntime.ai/)
- [llama.cpp GGUF](https://github.com/ggerganov/llama.cpp)
- [tract ONNX Runtime](https://github.com/sonos/tract)

## Example Models

Pre-trained models will be available at:
- `eidos-t5-small` - Lightweight model (~60MB)
- `eidos-t5-base` - Standard model (~220MB)
- `eidos-llama-7b-q4` - Quantized LLaMA (3.5GB)

Stay tuned for model releases!
