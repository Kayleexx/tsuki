use anyhow::Result;

use crate::ssh::{run_remote_command, Host};

pub async fn run_container(
    host: &Host,
    image: &str,
    container_name: &str,
    host_port: u16,
    container_port: u16,
) -> Result<()> {
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

    run_remote_command(host, &run_command).await?;

    Ok(())
}