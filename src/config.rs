use anyhow::Result;
use serde::Deserialize;

use crate::ssh::Host;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: HostConfig,
    pub network: NetworkConfig,
}

#[derive(Debug, Deserialize)]
pub struct HostConfig {
    pub ip: String,
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub domain: String,
}

pub fn load_config() -> Result<Config> {
    let path = format!(
        "{}/.config/tsuki/config.toml",
        std::env::var("HOME")?
    );

    let content = std::fs::read_to_string(path)?;

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

pub fn default_host() -> Result<Host> {
    let config = load_config()?;

    Ok(Host {
        user: config.host.user,
        host: config.host.ip,
    })
}