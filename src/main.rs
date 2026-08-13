mod config;
mod doctor;
mod error;
mod joplin;

use clap::{Parser, Subcommand};

use crate::{config::Config, doctor::run_doctor, error::SetuError};

#[derive(Debug, Parser)]
#[command(name = "setu", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate safe local configuration and Joplin reachability.
    Doctor,
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Setu doctor failed: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), SetuError> {
    let cli = Cli::parse();
    let config = Config::from_env()?;

    match cli.command {
        Command::Doctor => run_doctor(&config).await,
    }
}
