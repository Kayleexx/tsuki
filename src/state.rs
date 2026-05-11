use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn state_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;

    let path = PathBuf::from(home)
        .join(".tsuki");

    fs::create_dir_all(&path)?;

    Ok(path)
}