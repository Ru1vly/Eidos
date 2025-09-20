use std::path::Path;
use std::process::Command;
use tokenizers::Tokenizer;
use tract_onnx::prelude::*;

pub struct Core {
    model: SimplePlan<TypedModel>,
    tokenizer: Tokenizer,
}

impl Core {
    pub fn new<P: AsRef<Path>>(model_path: P, tokenizer_path: P) -> TractResult<Self> {
        let model = tract_onnx::onnx()
            .model_for_path(model_path)?
            .into_optimized()?
            .into_runnable()?;
        
        let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(|e| tract_core::anyhow::anyhow!(e))?;

        Ok(Self { model, tokenizer })
    }

    pub fn generate_command(&self, input: &str) -> TractResult<String> {
        let encoding = self.tokenizer.encode(input, true).map_err(|e| tract_core::anyhow::anyhow!(e))?;
        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let input_tensor = tract_ndarray::arr1(&input_ids).into_tensor().into_arc_tensor();

        let result = self.model.run(tvec!(input_tensor.into()))?;
        
        // This part is highly dependent on the model's output format.
        // Assuming the model outputs a sequence of token IDs that represent the command.
        let output_tensor = result[0].to_array_view::<i64>()?;
        let output_ids: Vec<u32> = output_tensor.iter().map(|&id| id as u32).collect();
        
        let command = self.tokenizer.decode(&output_ids, true).map_err(|e| tract_core::anyhow::anyhow!(e))?;

        Ok(command)
    }

    pub fn execute_command(&self, command: &str) -> Result<String, String> {
        if self.is_safe_command(command) {
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .map_err(|e| e.to_string())?;

            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        } else {
            Err("Command is not allowed.".to_string())
        }
    }

    fn is_safe_command(&self, command: &str) -> bool {
        // Basic security validation
        // A whitelist of allowed commands would be better.
        let forbidden = ["rm -rf", "sudo", ">", "|", "&", ";"];
        !forbidden.iter().any(|&s| command.contains(s))
    }

    pub fn run(&self, input: &str) -> Result<String, String> {
        match self.generate_command(input) {
            Ok(command) => self.execute_command(&command),
            Err(e) => Err(format!("Failed to generate command: {}", e)),
        }
    }
}

impl Default for Core {
    fn default() -> Self {
        // This default implementation will fail if the model/tokenizer files are not present.
        // It's better to use `Core::new()` with proper paths.
        // The paths here are placeholders.
        let model_path = "model.onnx";
        let tokenizer_path = "tokenizer.json";
        
        Core::new(model_path, tokenizer_path).expect("Failed to create Core instance")
    }
}
