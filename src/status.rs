use anyhow::{bail, Result};
use rusqlite::params;

use crate::{
    config::default_host,
    db::connect,
    ssh::run_remote_command_output,
};

pub async fn status(
    app: String,
) -> Result<()> {
    let conn = connect()?;

    let mut stmt = conn.prepare(
        "
        SELECT
            image_tag,
            port,
            container_id
        FROM deployments
        WHERE app_name = ?1
        ORDER BY id DESC
        LIMIT 1
        "
    )?;

    let deployment = stmt.query_row(
        params![app],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u16>(1)?,
                row.get::<_, String>(2)?,
            ))
        },
    );

    let (
        image,
        port,
        container_id,
    ) = match deployment {
        Ok(v) => v,
        Err(_) => bail!("app not found"),
    };

    let host = default_host();

    let runtime_status =
        run_remote_command_output(
            &host,
            &format!(
                "docker inspect -f '{{{{.State.Status}}}}' {}",
                container_id
            ),
        )
        .await?;

    println!("APP:        {}", app);
    println!("STATUS:     {}", runtime_status);
    println!("PORT:       {}", port);
    println!("IMAGE:      {}", image);
    println!("CONTAINER:  {}", container_id);

    Ok(())
}