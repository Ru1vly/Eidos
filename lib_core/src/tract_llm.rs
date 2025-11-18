use crate::validation::is_safe_command;
use anyhow::anyhow;
use ndarray::arr1;
use std::path::Path;
use tokenizers::Tokenizer;
use tract_onnx::prelude::*;

pub struct Core {
    model: TypedRunnableModel<TypedModel>,
    tokenizer: Tokenizer,
}

impl Core {
    pub fn new<P: AsRef<Path>>(model_path: P, tokenizer_path: P) -> TractResult<Self> {
        let model = tract_onnx::onnx()
            .model_for_path(model_path)?
            .into_optimized()?
            .into_runnable()?;

        let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(|e| anyhow!(e))?;

        Ok(Self { model, tokenizer })
    }

    pub fn generate_command(&self, input: &str) -> TractResult<String> {
        let encoding = self.tokenizer.encode(input, true).map_err(|e| anyhow!(e))?;
        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let input_tensor = arr1(&input_ids).into_dyn().into_tensor();

        let result = self.model.run(tvec!(input_tensor.into()))?;

        let output_tensor = result[0].to_array_view::<i64>()?;
        let output_ids: Vec<u32> = output_tensor.iter().map(|&id| id as u32).collect();

        let command = self
            .tokenizer
            .decode(&output_ids, true)
            .map_err(|e| anyhow!(e))?;

        Ok(command)
    }

    /// Validates if a command is safe to display to users
    /// This prevents generating dangerous commands that could harm the system
    /// Delegates to the validation module for consistency
    pub fn is_safe_command(&self, command: &str) -> bool {
        is_safe_command(command)
    }

    /// Generates an explanation for what a command does
    ///
    /// This helps users understand generated commands before executing them.
    /// The explanation describes the command's purpose, flags used, and potential side effects.
    ///
    /// # Example
    /// ```ignore
    /// let explanation = core.explain_command("ls -la")?;
    /// // Returns: "Lists all files in long format, including hidden files"
    /// ```
    pub fn explain_command(&self, command: &str) -> TractResult<String> {
        let prompt = format!("Explain what this command does: {}", command);

        let encoding = self.tokenizer.encode(prompt.as_str(), true).map_err(|e| anyhow!(e))?;
        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let input_tensor = arr1(&input_ids).into_dyn().into_tensor();

        let result = self.model.run(tvec!(input_tensor.into()))?;

        let output_tensor = result[0].to_array_view::<i64>()?;
        let output_ids: Vec<u32> = output_tensor.iter().map(|&id| id as u32).collect();

        let explanation = self
            .tokenizer
            .decode(&output_ids, true)
            .map_err(|e| anyhow!(e))?;

        Ok(explanation)
    }
}

impl Default for Core {
    fn default() -> Self {
        let model_path = "model.onnx";
        let tokenizer_path = "tokenizer.json";

        Core::new(model_path, tokenizer_path).expect("Failed to create Core instance")
    }
}
