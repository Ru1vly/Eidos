use anyhow::{Error as E, Result};
use candle_core::quantized::gguf_file;
use candle_core::{Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_llama::ModelWeights;
use std::fs::File;
use tokenizers::Tokenizer;

#[derive(Debug)]
pub enum QuantizedLlmError {
    ModelLoad(E),
    TokenizerLoad(E),
    Inference(E),
}

pub struct QuantizedLlm {
    model: ModelWeights,
    device: Device,
    tokenizer: Tokenizer,
    logits_processor: LogitsProcessor,
}

impl QuantizedLlm {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        let device = Device::Cpu;

        // Load the quantized model from GGUF file
        let mut file = File::open(model_path)
            .map_err(|e| E::msg(format!("Failed to open model file: {}", e)))?;

        // Read GGUF content
        let content = gguf_file::Content::read(&mut file)
            .map_err(|e| E::msg(format!("Failed to read GGUF file: {}", e)))?;

        let model_weights = ModelWeights::from_gguf(content, &mut file, &device)?;

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(E::msg)?;

        let logits_processor = LogitsProcessor::new(299792458, Some(0.0), None);

        Ok(Self {
            model: model_weights,
            device,
            tokenizer,
            logits_processor,
        })
    }

    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String> {
        // Fix tokenizer encoding - handle boxed error
        let encoding = self
            .tokenizer
            .encode(prompt, true)
            .map_err(|e| E::msg(format!("Tokenizer encoding failed: {}", e)))?;
        let tokens = encoding.get_ids().to_vec();
        let mut generated_tokens = Vec::new();
        let mut token_ids = tokens;

        for _ in 0..max_tokens {
            let context_size = token_ids.len();
            let context = &token_ids[..];
            let input = Tensor::new(context, &self.device)?.unsqueeze(0)?;

            // Quantized models manage their own internal state, no external cache needed
            let logits = self.model.forward(&input, context_size - 1)?;
            let logits = logits.squeeze(0)?;
            let next_token = self.logits_processor.sample(&logits)?;

            token_ids.push(next_token);
            generated_tokens.push(next_token);

            // Check for EOS token (empty string or actual EOS)
            if let Some(eos_token) = self.tokenizer.token_to_id("</s>") {
                if next_token == eos_token {
                    break;
                }
            }
        }

        // Fix tokenizer decoding - handle boxed error
        let output = self
            .tokenizer
            .decode(&generated_tokens, true)
            .map_err(|e| E::msg(format!("Tokenizer decoding failed: {}", e)))?;
        Ok(output)
    }
}
