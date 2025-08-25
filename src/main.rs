mod operations;
mod trie;

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
    let mut trie = operations::read_trie();

    match &cli.command {
        Commands::Add { word } => {
            trie.add_word(word);
            operations::save_trie(&trie);
        }
        Commands::PrefixSearch { word } => match trie.search_prefix(word) {
            true => println!("Found prefix!"),
            false => println!("Did not find prefix :("),
        },
        Commands::Search { word } => match trie.search_word(word) {
            true => println!("Found word!"),
            false => println!("Did not find word :("),
        },
    }
}
