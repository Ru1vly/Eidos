# This is the technical instructions file for Core package.

## Goal

Integrate a lightweight language model (LM) into your project (`Ru1vly/Eidos`) to interpret natural language and safely execute Linux terminal commands. Recommended: Use Python for AI/ML tasks and Rust for the system/program logic.

---

## Workflow Overview

1. **Fine-tune LM on Linux MAN Pages (Python)**
2. **Export Model for Inference**
3. **Build Main Program Logic in Rust**
4. **Communicate Between Python (AI) & Rust (System)**

---

## Step-by-Step Instructions

### 1. Fine-tune a Lightweight LM (Python)

- **Choose a model:** e.g., GPT-2 small, TinyLlama, DistilBERT.
- **Install dependencies:**
  ```bash
  pip install transformers datasets
  ```
- **Prepare MAN page data:**
  ```bash
  man -k . | awk '{print $1}' | xargs -n 1 man | col -bx > all_man_pages.txt
  ```
- **Fine-tune example:**
  ```python
  from transformers import AutoModelForCausalLM, AutoTokenizer, Trainer, TrainingArguments, TextDataset, DataCollatorForLanguageModeling

  model_name = "distilgpt2"
  tokenizer = AutoTokenizer.from_pretrained(model_name)
  model = AutoModelForCausalLM.from_pretrained(model_name)

  train_dataset = TextDataset(tokenizer=tokenizer, file_path="all_man_pages.txt", block_size=128)
  data_collator = DataCollatorForLanguageModeling(tokenizer=tokenizer, mlm=False)
  training_args = TrainingArguments(output_dir="./lm-man-finetuned", num_train_epochs=1, per_device_train_batch_size=2)

  trainer = Trainer(model=model, args=training_args, data_collator=data_collator, train_dataset=train_dataset)
  trainer.train()
  trainer.save_model("./lm-man-finetuned")
  ```

---

### 2. Export Model for Rust Inference

- **Export to ONNX or GGML:**
  - For ONNX: Use `transformers.onnx` export script.
  - For GGML: Use [llama.cpp](https://github.com/ggerganov/llama.cpp) conversion tools.
- **Recommended:**
  - Use ONNX for [tract](https://github.com/sonos/tract) or [candle](https://github.com/huggingface/candle).
  - Use GGML for [rust-bert](https://github.com/guillaume-be/rust-bert) with supported models.

---

### 3. Build Main Logic in Rust

- **Add dependencies:**
  - For ONNX: `tract-onnx`
  - For Transformers: `rust-bert`, `candle`
- **Sample shell command execution:**
  ```rust
  use std::process::Command;

  let output = Command::new("ls").arg("-l").output().expect("failed to execute process");
  println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
  ```
- **Model inference example (using rust-bert):**
  ```rust
  use rust_bert::pipelines::generation::GPT2Generator;

  let generator = GPT2Generator::new(Default::default()).unwrap();
  let input = "Show me how to list files";
  let output = generator.generate(Some(&[input]), None);
  ```

---

### 4. Connect Python AI & Rust System

- **Option 1: REST API**
  - Run Python model server (e.g., FastAPI/Flask).
  - Call endpoints from Rust (`reqwest` crate).
- **Option 2: Command-line Interface (CLI)**
  - Run Python script as subprocess from Rust.
- **Option 3: FFI/PyO3**
  - Embed Python directly in Rust (advanced).

---

## Security Best Practices

- Always sanitize LM-generated commands before execution.
- Run commands in a sandbox or restricted environment.
- Whitelist allowed commands, reject risky patterns (`rm -rf /`, etc.).

---

## References

- [Hugging Face Transformers](https://huggingface.co/docs/transformers/index)
- [rust-bert](https://github.com/guillaume-be/rust-bert)
- [tract](https://github.com/sonos/tract)
- [candle](https://github.com/huggingface/candle)
- [Open Interpreter](https://github.com/open-interpreter/open-interpreter)
- [llama.cpp](https://github.com/ggerganov/llama.cpp)

---

## Summary

- Use Python for AI/ML (fine-tuning, serving models).
- Use Rust for system logic and command execution.
- Connect via REST, CLI, or FFI.
- Always validate commands for safety.
