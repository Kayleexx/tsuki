use anyhow::Result;
use rusqlite::Connection;

pub async fn run() -> Result<()> {
    let conn = Connection::open(".tsuki/tsuki.db")?;

    let mut stmt = conn.prepare(
        "SELECT name, port FROM apps"
    )?;

    let apps = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, u16>(1)?,
        ))
    })?;

    println!(
        "{:<20} {:<10}",
        "APP",
        "PORT"
    );

    for app in apps {
        let (name, port) = app?;

        println!(
            "{:<20} {:<10}",
            name,
            port
        );
    }

    Ok(())
}