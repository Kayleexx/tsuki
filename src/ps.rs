use anyhow::Result;

use crate::{
    config::default_host,
    ssh::run_remote_command,
};

pub async fn run() -> Result<()> {
    let host = default_host();

    run_remote_command(
        &host,
        "docker ps --format 'table {{.Names}}\t{{.Status}}\t{{.Ports}}'"
    )
    .await?;

    Ok(())
}