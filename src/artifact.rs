use anyhow::{bail, Result};
use std::fs;
use tokio::process::Command;

pub async fn export_image(tag: &str, output: &str) -> Result<()> {
    fs::create_dir_all(".tsuki/artifacts")?;

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