use anyhow::Result;

pub async fn run(app: String) -> Result<()> {
    println!("showing logs for: {}", app);

    Ok(())
}