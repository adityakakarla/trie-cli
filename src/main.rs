use clap::{Parser, Subcommand};

// Keep a Trie running in the CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Search { word: String },
    PrefixSearch { word: String },
    Add { word: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { word } => {
            println!("Add {}", word);
        }
        Commands::PrefixSearch { word } => {
            println!("PrefixSearch {}", word);
        }
        Commands::Search { word } => {
            println!("Search {}", word);
        }
    }
}

// Design:
// first do clap
// then, implement accessing stored file + updates
// then, implement trie,
// then, integrate trie + file + cli
