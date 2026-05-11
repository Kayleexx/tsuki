use anyhow::{bail, Result};
use std::fs;
use tokio::process::Command;
use crate::state::state_dir;

pub async fn export_image(tag: &str, output: &str) -> Result<()> {
    
    let artifact_dir = state_dir()?
        .join("artifacts");
    
    fs::create_dir_all(&artifact_dir)?;

    let command = format!(
        "docker save {} | gzip > {}",
        tag,
        output
    );

    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .await?;

    if !status.success() {
        bail!("failed to export docker image");
    }

    Ok(())
}