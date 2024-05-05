use crate::{migrations::sql, StorageConnection};

pub(crate) fn migrate(connection: &StorageConnection) -> anyhow::Result<()> {
    sql!(
        connection,
        r#"CREATE TABLE site (
            id TEXT NOT NULL PRIMARY KEY,
            site_id TEXT NOT NULL,
            site_name TEXT NOT NULL,
            hardware_id TEXT NOT NULL
        );"#
    )?;

    Ok(())
}
