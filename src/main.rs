mod cli;
mod deploy;
mod logs;
mod app;
mod docker;
mod artifact;
mod ps;
mod ssh;
mod container;
mod config;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
mod db;
mod models;
mod ports;
mod caddy;

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
