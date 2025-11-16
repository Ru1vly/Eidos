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

/// Set up the Bridge with all request handlers
fn setup_bridge() -> Bridge {
    let mut bridge = Bridge::new();

    // Register Chat handler
    bridge.register(
        Request::Chat,
        Box::new(|text: &str| {
            let mut chat = Chat::new();
            chat.run(text);
            Ok(())
        }),
    );

    // Register Core handler
    bridge.register(
        Request::Core,
        Box::new(|prompt: &str| {
            // Load configuration
            let config = Config::load()
                .map_err(|e| format!("Config error: {}", e))?;

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
            let core = Core::new(&config.model_path, &config.tokenizer_path)
                .map_err(|e| format!("Failed to load model: {}", e))?;

            match core.run(prompt) {
                Ok(output) => {
                    println!("{}", output);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Core Error: {}", e);
                    Ok(())
                }
            }
        }),
    );

    // Register Translate handler
    bridge.register(
        Request::Translate,
        Box::new(|text: &str| {
            let translate = Translate::new();
            translate.run(text);
            Ok(())
        }),
    );

    bridge
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize the bridge with all handlers
    let bridge = setup_bridge();

    // Route commands through the bridge
    match cli.command {
        Commands::Chat { text } => {
            bridge.route(Request::Chat, &text)
                .map_err(|e| crate::error::AppError::InvalidInputError(e))?;
        }
        Commands::Core { prompt } => {
            bridge.route(Request::Core, &prompt)
                .map_err(|e| crate::error::AppError::InvalidInputError(e))?;
        }
        Commands::Translate { text } => {
            bridge.route(Request::Translate, &text)
                .map_err(|e| crate::error::AppError::InvalidInputError(e))?;
        }
    }

    Ok(())
}
