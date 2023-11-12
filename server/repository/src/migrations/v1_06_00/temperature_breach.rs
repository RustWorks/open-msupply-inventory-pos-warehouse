use crate::{migrations::sql, StorageConnection};

pub(crate) fn migrate(connection: &StorageConnection) -> anyhow::Result<()> {
    sql!(
        connection,
        r#"
        UPDATE temperature_breach SET acknowledged = not acknowledged;
        ALTER TABLE temperature_breach RENAME COLUMN acknowledged TO unacknowledged;
        "#,
    )?;

    Ok(())
}
