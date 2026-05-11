use anyhow::Result;

pub async fn run(path: String) -> Result<()> {
    println!("deploying app from: {}", path);

    Ok(())
}