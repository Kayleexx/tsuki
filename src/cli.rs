use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tsuki")]
#[command(about = "A tiny self-hosted PaaS")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Deploy {
        path: String,
    },

    Logs {
        app: String,
    },

    Ps,
}