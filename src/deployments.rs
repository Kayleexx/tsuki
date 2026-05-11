use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

use crate::db::connect;

pub fn record_deployment(
    app_name: &str,
    image_tag: &str,
    port: u16,
    container_id: &str,
) -> Result<()> {
    let conn = connect()?;

    conn.execute(
        "
        INSERT INTO deployments (
            app_name,
            image_tag,
            port,
            container_id,
            deployed_at,
            status
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ",
        params![
            app_name,
            image_tag,
            port,
            container_id,
            Utc::now().to_rfc3339(),
            "deployed"
        ],
    )?;

    Ok(())
}