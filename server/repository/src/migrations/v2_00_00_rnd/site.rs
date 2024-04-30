use crate::{migrations::sql, StorageConnection};

pub(crate) fn migrate(connection: &StorageConnection) -> anyhow::Result<()> {
    sql!(
        connection,
        r#"CREATE TABLE site (
            id TEXT NOT NULL PRIMARY KEY,
            site_id TEXT NOT NULL,
            initialisation_status TEXT NOT NULL,
        );"#
    )?;

    // TODO initialisation_status should be enum (new, started, completed, error)

    Ok(())
}
