// Example: Basic command generation with Eidos
//
// This example demonstrates the simplest use case: generating a shell command
// from natural language input.
//
// Run with: cargo run --example basic_usage

use lib_core::Core;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Eidos Basic Usage Example ===\n");

    // Note: This example requires valid model files
    // Set EIDOS_MODEL_PATH and EIDOS_TOKENIZER_PATH environment variables

    let model_path = std::env::var("EIDOS_MODEL_PATH")
        .unwrap_or_else(|_| "model.onnx".to_string());
    let tokenizer_path = std::env::var("EIDOS_TOKENIZER_PATH")
        .unwrap_or_else(|_| "tokenizer.json".to_string());

    println!("Loading model from: {}", model_path);
    println!("Loading tokenizer from: {}", tokenizer_path);
    println!();

    // Create a Core instance
    let core = Core::new(&model_path, &tokenizer_path)?;

    // Natural language prompts
    let prompts = vec![
        "list all files in current directory",
        "show current directory",
        "find text files",
        "count lines in readme",
    ];

    for prompt in prompts {
        println!("Prompt: \"{}\"", prompt);

        // Generate command
        match core.generate_command(prompt) {
            Ok(command) => {
                // Validate the generated command
                if core.is_safe_command(&command) {
                    println!("✅ Generated: {}", command);
                } else {
                    println!("❌ Generated command failed safety validation: {}", command);
                }
            }
            Err(e) => {
                println!("❌ Error generating command: {}", e);
            }
        }
        println!();
    }

    Ok(())
}
