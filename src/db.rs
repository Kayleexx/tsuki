use anyhow::Result;
use rusqlite::Connection;

pub fn connect() -> Result<Connection> {
    let conn = Connection::open(".tsuki/tsuki.db")?;

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

    Ok(conn)
}