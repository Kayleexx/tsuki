use anyhow::Result;
use rusqlite::Connection;

pub fn list_apps() -> Result<()> {
    let conn = Connection::open(".tsuki/tsuki.db")?;

    let mut stmt = conn.prepare(
        "
        SELECT
            app_name,
            image_tag,
            port,
            deployed_at,
            status
        FROM deployments
        ORDER BY id DESC
        "
    )?;

    let deployments = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, u16>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;

    println!(
        "{:<20} {:<12} {:<8} {:<30} {}",
        "APP",
        "STATUS",
        "PORT",
        "IMAGE",
        "DEPLOYED"
    );

    for deployment in deployments {
        let (
            app,
            image,
            port,
            deployed_at,
            status,
        ) = deployment?;

        println!(
            "{:<20} {:<12} {:<8} {:<30} {}",
            app,
            status,
            port,
            image,
            deployed_at
        );
    }

    Ok(())
}