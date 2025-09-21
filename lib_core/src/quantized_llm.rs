
use candle_core::{Device, Tensor};
use candle_gguf::gguf_file;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_llama as model;
use thiserror::Error;
use tokenizers::Tokenizer;

#[derive(Error, Debug)]
pub enum QuantizedLlmError {
    #[error("Failed to load model: {0}")]
    ModelLoad(String),
    #[error("Failed to load tokenizer: {0}")]
    TokenizerLoad(String),
    #[error("Inference failed: {0}")]
    Inference(String),
}

pub struct QuantizedLlm {
    model: model::Llama,
    device: Device,
    tokenizer: Tokenizer,
    logits_processor: LogitsProcessor,
}

impl QuantizedLlm {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self, QuantizedLlmError> {
        let device = Device::Cpu;

        let mut file =
            std::fs::File::open(model_path).map_err(|e| QuantizedLlmError::ModelLoad(e.to_string()))?;
        let model_weights =
            model::Llama::load(&mut file, &device).map_err(|e| QuantizedLlmError::ModelLoad(e.to_string()))?;

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| QuantizedLlmError::TokenizerLoad(e.to_string()))?;

        let logits_processor = LogitsProcessor::new(299792458, Some(0.0), None);

        Ok(Self {
            model: model_weights,
            device,
            tokenizer,
            logits_processor,
        })
    }

    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String, QuantizedLlmError> {
        let tokens = self
            .tokenizer
            .encode(prompt, true)
            .map_err(|e| QuantizedLlmError::Inference(e.to_string()))?
            .get_ids()
            .to_vec();

        let mut generated_tokens = Vec::new();
        let mut token_ids = tokens;

        for _ in 0..max_tokens {
            let context_size = token_ids.len();
            let context = &token_ids[..];
            let input = Tensor::new(context, &self.device)
                .unwrap()
                .unsqueeze(0)
                .unwrap();
            let logits = self.model.forward(&input, context_size - 1).unwrap();
            let logits = logits.squeeze(0).unwrap();
            let next_token = self.logits_processor.sample(&logits).unwrap();
            
            token_ids.push(next_token);
            generated_tokens.push(next_token);

            if next_token == self.tokenizer.token_to_id("").unwrap() {
                break;
            }
        }

        let output = self
            .tokenizer
            .decode(&generated_tokens, true)
            .map_err(|e| QuantizedLlmError::Inference(e.to_string()))?;

        Ok(output)
    }
}
