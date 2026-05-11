mod cli;
mod deploy;
mod logs;
mod app;
mod docker;
mod artifact;
mod ps;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Deploy { path } => {
            deploy::run(path).await?;
        }

        Commands::Logs { app } => {
            logs::run(app).await?;
        }

        Commands::Ps => {
            ps::run().await?;
        }
    }

    Ok(())
}
