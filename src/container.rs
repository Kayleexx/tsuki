use anyhow::{bail, Result};
use tokio::process::Command;

use crate::ssh::{run_remote_command, Host};

pub async fn run_container(
    host: &Host,
    image: &str,
    container_name: &str,
    host_port: u16,
    container_port: u16,
) -> Result<String> {
    let remove_command = format!(
        "docker rm -f {} >/dev/null 2>&1 || true",
        container_name
    );

    run_remote_command(host, &remove_command).await?;

    let run_command = format!(
        concat!(
            "docker run -d ",
            "--name {} ",
            "-p {}:{} ",
            "{}"
        ),
        container_name,
        host_port,
        container_port,
        image
    );

    let output = Command::new("ssh")
        .arg(format!(
            "{}@{}",
            host.user,
            host.host
        ))
        .arg(&run_command)
        .output()
        .await?;

    if !output.status.success() {
        bail!("failed to start container");
    }

    let container_id =
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();

    Ok(container_id)
}