use anyhow::{bail, Result};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct Host {
    pub user: String,
    pub host: String,
}

pub async fn run_remote_command(
    target: &Host,
    command: &str,
) -> Result<()> {
    let destination = format!(
        "{}@{}",
        target.user,
        target.host
    );

    let status = Command::new("ssh")
        .arg(destination)
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        bail!("remote command failed");
    }

    Ok(())
}

pub async fn upload_file(
    target: &Host,
    local_path: &str,
    remote_path: &str,
) -> Result<()> {
    let destination = format!(
        "{}@{}:{}",
        target.user,
        target.host,
        remote_path
    );

    let status = Command::new("scp")
        .arg(local_path)
        .arg(destination)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        bail!("file upload failed");
    }

    Ok(())
}

pub async fn run_remote_command_output(
    target: &Host,
    command: &str,
) -> Result<String> {
    let destination = format!(
        "{}@{}",
        target.user,
        target.host
    );

    let output = Command::new("ssh")
        .arg(destination)
        .arg(command)
        .output()
        .await?;

    if !output.status.success() {
        bail!("remote command failed");
    }

    Ok(
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string()
    )
}