import os
from transformers import AutoModelForCausalLM, AutoTokenizer, Trainer, TrainingArguments, TextDataset, DataCollatorForLanguageModeling
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

    # --- 2. Fine-tune Model ---
    model_name = "distilgpt2"
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    # Add a padding token if it doesn't exist
    if tokenizer.pad_token is None:
        tokenizer.add_special_tokens({'pad_token': '[PAD]'})

    model = AutoModelForCausalLM.from_pretrained(model_name)
    model.resize_token_embeddings(len(tokenizer)) # Resize for the new pad token

    train_dataset = TextDataset(tokenizer=tokenizer, file_path=dataset_path, block_size=128)
    data_collator = DataCollatorForLanguageModeling(tokenizer=tokenizer, mlm=False)
    
    output_dir = "./lm-command-finetuned"
    training_args = TrainingArguments(
        output_dir=output_dir,
        num_train_epochs=5, # Increase epochs for better training on small dataset
        per_device_train_batch_size=2,
    )

    trainer = Trainer(
        model=model,
        args=training_args,
        data_collator=data_collator,
        train_dataset=train_dataset,
    )

    trainer.train()
    trainer.save_model(output_dir)
    tokenizer.save_pretrained(output_dir)

    # --- 3. Export to ONNX ---
    onnx_output_path = "model.onnx"
    convert(
        framework="pt",
        model=output_dir,
        output=onnx_output_path,
        opset=12,
        tokenizer=output_dir,
    )
    print(f"Model exported to {onnx_output_path}")

    # --- 4. Save Tokenizer Config ---
    tokenizer_path = "tokenizer.json"
    tokenizer.save(tokenizer_path)
    print(f"Tokenizer config saved to {tokenizer_path}")


if __name__ == "__main__":
    train_and_export_model()
