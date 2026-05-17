mod errors;
mod scanner;
mod storage;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "relict")]
#[command(version)]
#[command(about = "Context extraction for inherited PHP systems")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            println!("Scanning: {}", path);
        }
    }
}
