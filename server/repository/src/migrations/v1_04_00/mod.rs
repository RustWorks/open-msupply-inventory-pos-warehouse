use super::{version::Version, Migration};

use crate::StorageConnection;
pub(crate) struct V1_04_00;
mod contact_trace;
mod date_of_death;

impl Migration for V1_04_00 {
    fn version(&self) -> Version {
        Version::from_str("1.4.0")
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        contact_trace::migrate(connection)?;
        date_of_death::migrate(connection)?;
        Ok(())
    }
}

#[cfg(test)]
#[actix_rt::test]
async fn migration_1_04_00() {
    use crate::migrations::*;
    use crate::test_db::*;

    let version = V1_04_00.version();

    // This test allows checking sql syntax
    let SetupResult { connection, .. } = setup_test(SetupOption {
        db_name: &format!("migration_{version}"),
        version: Some(version.clone()),
        ..Default::default()
    })
    .await;

    assert_eq!(get_database_version(&connection), version);
}
