use crate::{migrations::*, StorageConnection};

pub(crate) fn migrate(connection: &StorageConnection) -> anyhow::Result<()> {
    sql!(
        connection,
        r#"
            CREATE TABLE pack_unit (
                id TEXT NOT NULL PRIMARY KEY,
                item_id TEXT NOT NULL REFERENCES item(id),
                short_name TEXT NOT NULL,
                long_name TEXT NOT NULL,
                pack_size INTEGER NOT NULL
            );
        "#,
    )?;

    Ok(())
}
