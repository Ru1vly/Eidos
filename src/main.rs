mod error;

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
            let chat = Chat::new();
            chat.run(&text);
        }
        Commands::Core { prompt } => {
            let core = Core::default();
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
