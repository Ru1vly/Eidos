mod config;
mod error;

use crate::config::Config;
use crate::error::Result;
use clap::{Parser, Subcommand};
use lib_bridge::{Bridge, Request};
use lib_chat::Chat;
use lib_core::Core;
use lib_translate::Translate;
use log::{debug, error, info, warn};

#[derive(Parser, Debug)]
#[clap(
    author = "EIDOS",
    version = "0.1.0",
    about = "AI-powered CLI for Linux - Natural language to shell commands"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long, global = true, help = "Enable verbose logging")]
    verbose: bool,

    #[clap(short, long, global = true, help = "Enable debug logging")]
    debug: bool,
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

/// Validate input text for safety and sanity
fn validate_input(text: &str, max_length: usize) -> std::result::Result<(), String> {
    // Check for empty input
    if text.trim().is_empty() {
        return Err("Input cannot be empty".to_string());
    }

    // Check length
    if text.len() > max_length {
        return Err(format!(
            "Input too long ({} chars, max {})",
            text.len(),
            max_length
        ));
    }

    // Check for control characters (except newlines/tabs)
    if text.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
        warn!("Input contains control characters, sanitizing");
    }

    debug!("Input validation passed: {} chars", text.len());
    Ok(())
}

/// Initialize logging based on verbosity level
fn init_logging(verbose: bool, debug_mode: bool) {
    let log_level = if debug_mode {
        "debug"
    } else if verbose {
        "info"
    } else {
        "warn"
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .format_timestamp(None)
        .init();

    debug!("Logging initialized at {} level", log_level);
}

/// Set up the Bridge with all request handlers
fn setup_bridge() -> Bridge {
    let mut bridge = Bridge::new();

    // Register Chat handler
    bridge.register(
        Request::Chat,
        Box::new(|text: &str| {
            info!("Processing chat request");
            debug!("Chat input: {}", text);

            let mut chat = Chat::new();
            chat.run(text);

            debug!("Chat request completed");
            Ok(())
        }),
    );

    // Register Core handler
    bridge.register(
        Request::Core,
        Box::new(|prompt: &str| {
            info!("Processing core command generation request");
            debug!("Prompt: {}", prompt);

            // Load configuration
            debug!("Loading configuration");
            let config = Config::load()
                .map_err(|e| {
                    error!("Configuration loading failed: {}", e);
                    format!("Config error: {}", e)
                })?;

            // Validate configuration
            if let Err(e) = config.validate() {
                error!("Configuration validation failed: {}", e);
                eprintln!("❌ Configuration Error: {}", e);
                eprintln!();
                eprintln!("To configure Eidos, choose one of:");
                eprintln!("  1. Environment variables:");
                eprintln!("     export EIDOS_MODEL_PATH=/path/to/model.onnx");
                eprintln!("     export EIDOS_TOKENIZER_PATH=/path/to/tokenizer.json");
                eprintln!();
                eprintln!("  2. Config file (./eidos.toml or ~/.config/eidos/eidos.toml):");
                eprintln!("     model_path = \"/path/to/model.onnx\"");
                eprintln!("     tokenizer_path = \"/path/to/tokenizer.json\"");
                eprintln!();
                eprintln!("  3. See docs/MODEL_GUIDE.md for training your own model");
                return Ok(());
            }

            debug!("Configuration valid, loading model");

            // Create Core instance with config
            let core = Core::new(&config.model_path, &config.tokenizer_path)
                .map_err(|e| {
                    error!("Model loading failed: {}", e);
                    format!("Failed to load model: {}", e)
                })?;

            info!("Model loaded successfully");

            // Run inference
            match core.run(prompt) {
                Ok(output) => {
                    info!("Command generated successfully");
                    debug!("Generated command: {}", output);
                    println!("{}", output);
                    Ok(())
                }
                Err(e) => {
                    error!("Inference failed: {}", e);
                    eprintln!("❌ Error: {}", e);
                    eprintln!();
                    eprintln!("This could be due to:");
                    eprintln!("  - Invalid or corrupted model file");
                    eprintln!("  - Incompatible model format");
                    eprintln!("  - Prompt too long or malformed");
                    Ok(())
                }
            }
        }),
    );

    // Register Translate handler
    bridge.register(
        Request::Translate,
        Box::new(|text: &str| {
            info!("Processing translation request");
            debug!("Translation input: {}", text);

            let translate = Translate::new();
            translate.run(text);

            debug!("Translation request completed");
            Ok(())
        }),
    );

    debug!("Bridge setup complete with {} handlers", 3);
    bridge
}

fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.verbose, cli.debug);

    info!("Eidos v0.1.0 starting");
    debug!("Command: {:?}", cli.command);

    // Initialize the bridge with all handlers
    let bridge = setup_bridge();

    // Route commands through the bridge with input validation
    let result = match cli.command {
        Commands::Chat { ref text } => {
            // Validate input (max 10000 chars for chat)
            if let Err(e) = validate_input(text, 10000) {
                error!("Input validation failed: {}", e);
                eprintln!("❌ Invalid input: {}", e);
                return Ok(());
            }

            debug!("Routing to chat handler");
            bridge.route(Request::Chat, text)
                .map_err(|e| {
                    error!("Chat routing failed: {}", e);
                    crate::error::AppError::InvalidInputError(e)
                })
        }
        Commands::Core { ref prompt } => {
            // Validate input (max 1000 chars for prompts)
            if let Err(e) = validate_input(prompt, 1000) {
                error!("Input validation failed: {}", e);
                eprintln!("❌ Invalid input: {}", e);
                return Ok(());
            }

            debug!("Routing to core handler");
            bridge.route(Request::Core, prompt)
                .map_err(|e| {
                    error!("Core routing failed: {}", e);
                    crate::error::AppError::InvalidInputError(e)
                })
        }
        Commands::Translate { ref text } => {
            // Validate input (max 5000 chars for translation)
            if let Err(e) = validate_input(text, 5000) {
                error!("Input validation failed: {}", e);
                eprintln!("❌ Invalid input: {}", e);
                return Ok(());
            }

            debug!("Routing to translate handler");
            bridge.route(Request::Translate, text)
                .map_err(|e| {
                    error!("Translate routing failed: {}", e);
                    crate::error::AppError::InvalidInputError(e)
                })
        }
    };

    match result {
        Ok(_) => {
            info!("Operation completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Operation failed: {}", e);
            Err(e)
        }
    }
}
