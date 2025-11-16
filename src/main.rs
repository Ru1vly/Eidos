mod config;
mod error;

use crate::config::Config;
use crate::error::Result;
use clap::{Parser, Subcommand};
use lib_bridge::{Bridge, Request};
use lib_chat::Chat;
use lib_core::Core;
use lib_translate::Translate;

#[derive(Parser, Debug)]
#[clap(
    author = "EIDOS",
    version = "0.1.0",
    about = "A multifunctional application"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Chat with the AI model")]
    Chat {
        #[clap(help = "The input text for the chat")]
        text: String,
    },
    #[clap(about = "Core functionality")]
    Core {
        #[clap(help = "The prompt for the core model")]
        prompt: String,
    },
    #[clap(about = "Translate text")]
    Translate {
        #[clap(help = "The text to translate")]
        text: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chat { text } => {
            let mut chat = Chat::new();
            chat.run(&text);
        }
        Commands::Core { prompt } => {
            // Load configuration
            let config = Config::load().map_err(|e| {
                crate::error::AppError::InvalidInputError(format!("Config error: {}", e))
            })?;

            // Validate configuration
            if let Err(e) = config.validate() {
                eprintln!("Configuration validation failed: {}", e);
                eprintln!(
                    "Tip: Set EIDOS_MODEL_PATH and EIDOS_TOKENIZER_PATH environment variables"
                );
                eprintln!("  or create an eidos.toml config file");
                return Ok(());
            }

            // Create Core instance with config
            let core = Core::new(&config.model_path, &config.tokenizer_path).map_err(|e| {
                crate::error::AppError::AIModelError(format!("Failed to load model: {}", e))
            })?;

            match core.run(&prompt) {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("Core Error: {}", e),
            }
        }
        Commands::Translate { text } => {
            let translate = Translate::new();
            translate.run(&text);
        }
    }

    Ok(())
}
