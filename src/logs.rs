use anyhow::Result;

use crate::{
    config::default_host,
    ssh::run_remote_command,
};

pub async fn run(app: String) -> Result<()> {
    let host = default_host();

    let command = format!(
        "docker logs -f {}",
        app
    );

    run_remote_command(
        &host,
        &command,
    )
    .await?;

    Ok(())
}