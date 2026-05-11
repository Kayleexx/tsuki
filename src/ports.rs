use anyhow::Result;
use rusqlite::params;

use crate::db::connect;

const BASE_PORT: u16 = 8000;

pub fn allocate_port() -> Result<u16> {
    let conn = connect()?;

    let mut stmt = conn.prepare(
        "SELECT port FROM apps ORDER BY port ASC"
    )?;

    let ports = stmt.query_map([], |row| {
        row.get::<_, u16>(0)
    })?;

    let mut next_port = BASE_PORT;

    for port in ports {
        let port = port?;

        if port == next_port {
            next_port += 1;
        }
    }

    Ok(next_port)
}

pub fn save_app(
    name: &str,
    port: u16,
) -> Result<()> {
    let conn = connect()?;

    conn.execute(
        "
        INSERT OR REPLACE INTO apps (name, port)
        VALUES (?1, ?2)
        ",
        params![name, port],
    )?;

    Ok(())
}