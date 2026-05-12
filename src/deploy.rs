use anyhow::Result;

use crate::app::{AppType, detect_app};
use crate::artifact::export_image;
use crate::caddy::configure_app;
use crate::config::default_host;
use crate::container::run_container;
use crate::deployments::record_deployment;
use crate::docker::build_image;
use crate::ports::{get_or_allocate_port, save_app};
use crate::ssh::{run_remote_command, upload_file};

use chrono::Utc;

pub async fn run(path: String) -> Result<()> {
    println!("✓ starting deployment");

    let app_name = std::fs::canonicalize(&path)?
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    println!("✓ detecting app type");

    let app_type = detect_app(&path)?;

    let timestamp = Utc::now().timestamp();

    let image_tag = match app_type {
        AppType::Docker => {
            format!("tsuki-app:{}", timestamp)
        }
        AppType::Rust => {
            format!("tsuki-rust:{}", timestamp)
        }
        AppType::Node => {
            format!("tsuki-node:{}", timestamp)
        }
    };

    println!("✓ building container");

    build_image(&path, &image_tag).await?;

    let artifact_path = ".tsuki/artifacts/tsuki-app.tar.gz";

    println!("✓ exporting artifact");

    export_image(&image_tag, artifact_path).await?;

    let host = default_host()?;

    let remote_artifact_path = "/tmp/tsuki-app.tar.gz";

    println!("✓ uploading artifact");

    upload_file(&host, artifact_path, remote_artifact_path).await?;

    println!("✓ loading remote image");

    run_remote_command(
        &host,
        &format!("docker load < {} >/dev/null 2>&1", remote_artifact_path),
    )
    .await?;

    let port = get_or_allocate_port(&app_name)?;

    save_app(&app_name, port)?;

    println!("✓ allocating port {}", port);

    println!("✓ starting container");

    let container_id = run_container(&host, &image_tag, &app_name, port, 80).await?;

    println!("✓ configuring reverse proxy");

    configure_app(&host, &app_name, port).await?;

    println!("✓ running health checks");

    run_remote_command(
        &host,
        &format!("curl -fsS http://localhost:{} >/dev/null", port),
    )
    .await?;

    println!();
    println!("application live at:");
    println!("http://{}.{}.sslip.io", app_name, host.host);

    record_deployment(&app_name, &image_tag, port, &container_id)?;

    Ok(())
}
