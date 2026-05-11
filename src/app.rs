use anyhow::{bail, Result};
use std::path::Path;

#[derive(Debug)]
pub enum AppType {
    Docker,
    Rust,
    Node,
}

pub fn detect_app(path: &str) -> Result<AppType> {
    let root = Path::new(path);

    if root.join("Dockerfile").exists() {
        return Ok(AppType::Docker);
    }

    if root.join("Cargo.toml").exists() {
        return Ok(AppType::Rust);
    }

    if root.join("package.json").exists() {
        return Ok(AppType::Node);
    }

    bail!("could not detect app type in path: {}", path)
}