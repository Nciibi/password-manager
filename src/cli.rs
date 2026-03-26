use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pm")]
#[command(about = "A zero-knowledge, offline-first CLI password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new password vault
    Init,
    /// Add a new item to the vault
    Add,
    /// Get a password by item name (copies to clipboard)
    Get {
        /// Name of the item
        name: String,
    },
    /// List all items in the vault
    List,
    /// Delete an item from the vault
    Delete {
        /// Name of the item
        name: String,
    },
    /// Generate a strong random password
    Generate {
        /// Length of the generated password
        #[arg(short, long, default_value_t = 20)]
        length: usize,
        /// Disable symbols in the generated password
        #[arg(long, default_value_t = false)]
        no_symbols: bool,
    },
}
