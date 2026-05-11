use anyhow::Result;
use tracing::info;

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
    println!("✓ Starting deployment");

    let app_name = std::fs::canonicalize(&path)?
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let app_type = detect_app(&path)?;

    info!("detected app type: {:?}", app_type);

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

    build_image(&path, &image_tag).await?;

    info!("image built successfully");

    let artifact_path = ".tsuki/artifacts/tsuki-app.tar.gz";

    info!("exporting deployment artifact");

    export_image(&image_tag, artifact_path).await?;

    info!("artifact created at: {}", artifact_path);

    let host = default_host();

    let remote_artifact_path = "/tmp/tsuki-app.tar.gz";

    info!("uploading artifact to remote host");

    upload_file(&host, artifact_path, remote_artifact_path).await?;

    info!("loading docker image remotely");

    run_remote_command(&host, &format!("docker load < {}", remote_artifact_path)).await?;

    info!("remote image loaded successfully");

    let port = get_or_allocate_port(&app_name)?;

    save_app(&app_name, port)?;

    info!("allocated port: {}", port);

    info!("starting remote container");

    let container_id = run_container(&host, &image_tag, &app_name, port, 80).await?;

    info!("configuring reverse proxy");

    println!("✓ Configuring reverse proxy");

    configure_app(&host, &app_name, port).await?;

    println!("✓ Reverse proxy configured");

    println!("✓ Running health checks");

    run_remote_command(
        &host,
        &format!("curl -f http://localhost:{} >/dev/null", port),
    )
    .await?;

    println!("✓ Health checks passed");
    println!(" Application live at:");
    println!("https://{}.{}.sslip.io", app_name, host.host);

    record_deployment(&app_name, &image_tag, port, &container_id)?;

    info!("container started successfully");

    Ok(())
}
