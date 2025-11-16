use anyhow::anyhow;
use ndarray::arr1;
use std::path::Path;
use std::process::Command;
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

    /// Validates if a command is safe to execute
    /// This is public for testing purposes
    pub fn is_safe_command(&self, command: &str) -> bool {
        // Whitelist of safe base commands
        let allowed_commands = [
            "ls", "pwd", "echo", "cat", "head", "tail", "grep", "find", "wc", "date", "whoami",
            "hostname", "uname", "df", "du", "free", "top", "ps", "which", "whereis", "file",
            "stat", "touch", "mkdir",
        ];

        // Dangerous patterns that should never be allowed
        let dangerous_patterns = [
            "rm",
            "rmdir",
            "dd",
            "mkfs",
            "fdisk",
            "shutdown",
            "reboot",
            "halt",
            "poweroff",
            "init",
            "kill",
            "killall",
            "pkill",
            "chown",
            "chmod",
            "chgrp",
            "useradd",
            "userdel",
            "groupadd",
            "groupdel",
            "passwd",
            "su",
            "sudo",
            "doas",
            "curl",
            "wget",
            "nc",
            "netcat",
            "telnet",
            "ssh",
            "scp",
            "sftp",
            "rsync",
            "mount",
            "umount",
            "mkswap",
            "swapon",
            "swapoff",
            "iptables",
            "ip6tables",
            "nft",
        ];

        // Shell metacharacters and injection patterns
        let shell_injection_patterns = [
            "`", "$(", "${", "$((", ">>", "<<<", "&>", "|&", "&&", "||", "|", ";", "\n", "\r",
            "\\", "'", "\"", "*", "?", "[", "]", "{", "}", "!", "~", "^", "<(", ">(", "../",
            "/dev/", "/proc/", "/sys/", ">", "&",
        ];

        let cmd_lower = command.to_lowercase();
        let cmd_trimmed = command.trim();

        // Check for dangerous patterns
        if dangerous_patterns.iter().any(|&p| {
            cmd_lower.contains(p)
                || cmd_trimmed.starts_with(p)
                || cmd_lower.contains(&format!("/{}", p))
        }) {
            return false;
        }

        // Check for shell injection attempts
        if shell_injection_patterns
            .iter()
            .any(|&p| command.contains(p))
        {
            return false;
        }

        // Check if command starts with an allowed command
        let first_word = cmd_trimmed.split_whitespace().next().unwrap_or("");
        if !allowed_commands.iter().any(|&c| first_word == c) {
            return false;
        }

        // Additional checks for suspicious patterns
        // Check for hex/octal encoded characters
        if command.contains("\\x") || command.contains("\\0") {
            return false;
        }

        // Check for IFS manipulation
        if command.to_uppercase().contains("IFS") {
            return false;
        }

        // Command seems safe
        true
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
        let model_path = "model.onnx";
        let tokenizer_path = "tokenizer.json";

        Core::new(model_path, tokenizer_path).expect("Failed to create Core instance")
    }
}
