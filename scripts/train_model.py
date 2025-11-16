import os
from pathlib import Path
from transformers import AutoModelForCausalLM, AutoTokenizer, Trainer, TrainingArguments, DataCollatorForLanguageModeling
from datasets import load_dataset
from transformers.convert_graph_to_onnx import convert

def train_and_export_model():
    # --- 1. Prepare Dataset ---
    # Use the MAN pages dataset
    dataset_path = "SLM-training-data.txt"
    if not os.path.exists(dataset_path):
        raise FileNotFoundError(
            f"Dataset file not found: {dataset_path}. "
            "Please generate it using WSL and place it in the project root."
        )

    # --- 2. Initialize Model and Tokenizer ---
    model_name = "distilgpt2"
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    
    # Add a padding token if it doesn't exist
    if tokenizer.pad_token is None:
        tokenizer.add_special_tokens({'pad_token': '[PAD]'})

    model = AutoModelForCausalLM.from_pretrained(model_name)
    model.resize_token_embeddings(len(tokenizer))

    # --- 3. Process Dataset ---
    print("Processing dataset...")
    raw_dataset = load_dataset("text", data_files=dataset_path)

    def tokenize_function(examples):
        return tokenizer(examples["text"], truncation=True, max_length=128, padding="max_length")

    tokenized_dataset = raw_dataset.map(tokenize_function, batched=True, remove_columns=["text"])
    
    data_collator = DataCollatorForLanguageModeling(tokenizer=tokenizer, mlm=False)

    # --- 4. Fine-tune Model with Performance Optimizations ---
    output_dir = "./lm-command-finetuned"
    training_args = TrainingArguments(
        output_dir=output_dir,
        num_train_epochs=1,  # Reduced epochs to 1 for the large dataset
        per_device_train_batch_size=8,  # Increased batch size
        fp16=True,  # Enable mixed-precision training for speed
        dataloader_num_workers=4,  # Use more workers for data loading
        logging_steps=50, # Log progress more frequently
    )

    trainer = Trainer(
        model=model,
        args=training_args,
        data_collator=data_collator,
        train_dataset=tokenized_dataset["train"],
    )

    print("Starting training...")
    trainer.train()
    print("Training finished.")
    
    trainer.save_model(output_dir)
    tokenizer.save_pretrained(output_dir)

    # --- 5. Export to ONNX ---
    print("Exporting model to ONNX...")
    onnx_output_path = Path("model.onnx")
    convert(
        framework="pt",
        model=output_dir,
        output=onnx_output_path,
        opset=12,
        tokenizer=output_dir,
    )
    print(f"Model exported to {onnx_output_path}")

    # --- 6. Save Tokenizer Config for Rust ---
    tokenizer_path = "tokenizer.json"
    tokenizer.save(tokenizer_path)
    print(f"Tokenizer config saved to {tokenizer_path}")


if __name__ == "__main__":
    train_and_export_model()
