use anyhow::Result;
use tracing::info;

use crate::app::{detect_app, AppType};
use crate::docker::build_image;

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

    Ok(())
}