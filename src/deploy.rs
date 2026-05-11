use anyhow::Result;
use tracing::info;

use crate::app::{detect_app, AppType};
use crate::docker::build_image;
use crate::artifact::export_image;
use crate::ssh::{run_remote_command, upload_file, Host};
use crate::container::run_container;

pub async fn run(path: String) -> Result<()> {
    info!("starting deployment");

    let app_type = detect_app(&path)?;

    info!("detected app type: {:?}", app_type);

    let image_tag = match app_type {
        AppType::Docker => "tsuki-app:latest",
        AppType::Rust => "tsuki-rust:latest",
        AppType::Node => "tsuki-node:latest",
    };

    build_image(&path, image_tag).await?;

    info!("image built successfully");

    let artifact_path = ".tsuki/artifacts/tsuki-app.tar.gz";
    
    info!("exporting deployment artifact");
    
    export_image(image_tag, artifact_path).await?;
    
    info!("artifact created at: {}", artifact_path);

    let host = Host {
        user: "kaylee".into(),
        host: "192.168.0.47".into(),
    };
    
    let remote_artifact_path = "/tmp/tsuki-app.tar.gz";
    
    info!("uploading artifact to remote host");
    
    upload_file(
        &host,
        artifact_path,
        remote_artifact_path,
    )
    .await?;
    
    info!("loading docker image remotely");
    
    run_remote_command(
        &host,
        &format!(
            "docker load < {}",
            remote_artifact_path
        ),
    )
    .await?;
    
    info!("remote image loaded successfully");

    info!("starting remote container");
    
    run_container(
        &host,
        image_tag,
        "tsuki-test-app",
        8080,
        80,
    )
    .await?;
    
    info!("container started successfully");

    Ok(())
}