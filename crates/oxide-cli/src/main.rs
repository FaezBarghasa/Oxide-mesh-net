//! Oxide Mesh Net CLI

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oxide", about = "CLI for oxide-mesh-net")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Up,
    Down,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _cli = Cli::parse();
    println!("oxide-mesh-net CLI");
    Ok(())
}
