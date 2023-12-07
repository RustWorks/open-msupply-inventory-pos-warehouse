use crate::{migrations::sql, StorageConnection};

pub(crate) fn migrate(connection: &StorageConnection) -> anyhow::Result<()> {
    sql!(
        connection,
        r#"
        ALTER TABLE item ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT TRUE;
        "#,
    )?;

    Ok(())
}
