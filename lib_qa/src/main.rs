use clap::Parser;
use lib_core::quantized_llm::QuantizedLlm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The prompt for the text generation
    #[arg(short, long, default_value = "What is the capital of Turkey?")]
    prompt: String,

    /// The maximum number of tokens to generate
    #[arg(short, long, default_value = "50")]
    max_tokens: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Define the paths to the quantized model and tokenizer
    let model_path = "ggml-model-q4_k_m.gguf";
    let tokenizer_path = "lm-command-finetuned/checkpoint-29500/tokenizer.json";

    // Create a new instance of the QuantizedLlm
    let mut llm = match QuantizedLlm::new(model_path, tokenizer_path) {
        Ok(llm) => llm,
        Err(e) => {
            eprintln!("Failed to load the model: {}", e);
            return;
        }
    };

    // Generate the text
    match llm.generate(&args.prompt, args.max_tokens) {
        Ok(output) => {
            println!("Generated text: {}", output);
        }
        Err(e) => {
            eprintln!("Failed to generate text: {}", e);
        }
    }
}
