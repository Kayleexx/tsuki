use anyhow::Result;
use rusqlite::Connection;
use crate::state::state_dir;

pub fn connect() -> Result<Connection> {
    std::fs::create_dir_all(".tsuki")?;

    
    let db_path = state_dir()?
        .join("tsuki.db");
    
    let conn = Connection::open(db_path)?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS apps (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            port INTEGER NOT NULL
        )
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS deployments (
            id INTEGER PRIMARY KEY,
            app_name TEXT NOT NULL,
            image_tag TEXT NOT NULL,
            port INTEGER NOT NULL,
            container_id TEXT NOT NULL,
            deployed_at TEXT NOT NULL,
            status TEXT NOT NULL
        )
        ",
        [],
    )?;

    Ok(conn)
}