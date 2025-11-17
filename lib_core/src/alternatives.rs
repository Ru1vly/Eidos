// Alternative command generation strategies

use crate::Core;
use tract_onnx::prelude::TractResult;

impl Core {
    /// Generate multiple alternative commands for the same prompt
    ///
    /// This provides users with options to choose from, enhancing flexibility.
    /// Different alternatives may vary in:
    /// - Verbosity (more or fewer flags)
    /// - Approach (different tools for same task)
    /// - Output format
    ///
    /// # Example
    /// ```ignore
    /// let alternatives = core.generate_alternatives("list files", 3)?;
    /// // Might return: ["ls", "ls -a", "ls -la"]
    /// ```
    pub fn generate_alternatives(&self, input: &str, count: usize) -> TractResult<Vec<String>> {
        if count == 0 {
            return Ok(vec![]);
        }

        if count == 1 {
            return Ok(vec![self.generate_command(input)?]);
        }

        let mut alternatives = Vec::with_capacity(count);

        // Generate base command
        let base_command = self.generate_command(input)?;
        alternatives.push(base_command.clone());

        // Generate variations with modified prompts
        let variations = vec![
            format!("{} with details", input),
            format!("{} verbose", input),
            format!("{} concise", input),
            format!("{} with all options", input),
            format!("{} simple", input),
        ];

        for variation in variations.iter().take(count - 1) {
            match self.generate_command(variation) {
                Ok(cmd) => {
                    // Only add if different from base and not already in list
                    if cmd != base_command && !alternatives.contains(&cmd) {
                        alternatives.push(cmd);
                    }
                }
                Err(_) => continue, // Skip variations that fail
            }

            if alternatives.len() >= count {
                break;
            }
        }

        // If we didn't get enough unique alternatives, pad with the base command
        while alternatives.len() < count {
            alternatives.push(base_command.clone());
        }

        Ok(alternatives)
    }
}
