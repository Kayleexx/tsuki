use anyhow::{bail, Result};
use rusqlite::params;

use crate::{
    config::default_host,
    container::run_container,
    db::connect,
};

pub async fn rollback(
    app: String,
) -> Result<()> {
    let conn = connect()?;

    let mut stmt = conn.prepare(
        "
        SELECT image_tag, port
        FROM deployments
        WHERE app_name = ?1
        ORDER BY id DESC
        LIMIT 1 OFFSET 1
        "
    )?;

    let deployment = stmt.query_row(
        params![app],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u16>(1)?,
            ))
        },
    );

    let (image_tag, port) = match deployment {
        Ok(v) => v,
        Err(_) => bail!("no previous deployment found"),
    };

    let host = default_host()?;

    println!(
        "rolling back to image: {}",
        image_tag
    );

    run_container(
        &host,
        &image_tag,
        &app,
        port,
        80,
    )
    .await?;

    println!("rollback completed");

    Ok(())
}