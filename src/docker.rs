use anyhow::{bail, Result};
use std::process::Stdio;
use tokio::process::Command;

pub async fn build_image(path: &str, tag: &str) -> Result<()> {
    let status = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(tag)
        .arg(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        bail!("docker build failed");
    }

    Ok(())
}