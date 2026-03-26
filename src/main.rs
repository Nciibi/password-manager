pub mod cli;
pub mod commands;
pub mod crypto;
pub mod security;
pub mod vault;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => commands::init::execute()?,
        Commands::Add => commands::add::execute()?,
        Commands::Get { name } => commands::get::execute(name)?,
        Commands::List => commands::list::execute()?,
        Commands::Delete { name } => commands::delete::execute(name)?,
        Commands::Generate { length, no_symbols } => commands::generate::execute(*length, *no_symbols)?,
    }

    Ok(())
}
