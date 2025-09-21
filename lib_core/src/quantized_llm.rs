use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_llama as model;
use tokenizers::Tokenizer;
use std::fs::File;
use std::io::BufReader;
use candle_transformers::models::llama::Llama;

#[derive(Debug)]
pub enum QuantizedLlmError {
    ModelLoad(E),
    TokenizerLoad(E),
    Inference(E),
}

pub struct QuantizedLlm {
    model: Llama,
    device: Device,
    tokenizer: Tokenizer,
    logits_processor: LogitsProcessor,
}

impl QuantizedLlm {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        let device = Device::Cpu;
        let mut file = File::open(model_path)?;
        let mut model_reader = BufReader::new(file);
        let model_weights = Llama::load(&mut model_reader, &device)?;
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
        let tokens = self.tokenizer.encode(prompt, true)?.get_ids().to_vec();
        let mut generated_tokens = Vec::new();
        let mut token_ids = tokens;

        for _ in 0..max_tokens {
            let context_size = token_ids.len();
            let context = &token_ids[..];
            let input = Tensor::new(context, &self.device)?.unsqueeze(0)?;
            let logits = self.model.forward(&input, context_size - 1)?;
            let logits = logits.squeeze(0)?;
            let next_token = self.logits_processor.sample(&logits)?;
            
            token_ids.push(next_token);
            generated_tokens.push(next_token);

            if next_token == self.tokenizer.token_to_id("").unwrap() {
                break;
            }
        }

        let output = self.tokenizer.decode(&generated_tokens, true).map_err(E::msg)?;
        Ok(output)
    }
}
