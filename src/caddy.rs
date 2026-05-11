use anyhow::Result;

use crate::ssh::{run_remote_command, Host};

pub async fn configure_app(
    host: &Host,
    app_name: &str,
    port: u16,
) -> Result<()> {
    let domain = format!(
        "{}.{}.sslip.io",
        app_name,
        host.host
    );

    let config = format!(
        r#"cat <<EOF | sudo tee /etc/caddy/Caddyfile
{} {{
    reverse_proxy localhost:{}
}}
EOF"#,
        domain,
        port
    );

    run_remote_command(host, &config).await?;

    run_remote_command(
        host,
        "sudo caddy validate --config /etc/caddy/Caddyfile",
    )
    .await?;

    run_remote_command(
        host,
        "sudo systemctl reload caddy",
    )
    .await?;

    Ok(())
}