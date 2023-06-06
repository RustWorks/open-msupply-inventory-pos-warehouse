use super::{version::Version, Migration};

use crate::StorageConnection;
mod repack_report;
pub(crate) struct V1_01_15;

impl Migration for V1_01_15 {
    fn version(&self) -> Version {
        Version::from_str("1.1.15")
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        repack_report::migrate(connection)?;
        Ok(())
    }
}

#[cfg(test)]
#[actix_rt::test]
async fn migration_1_01_15() {
    use crate::migrations::*;
    use crate::test_db::*;

    let version = V1_01_15.version();

    // This test allows checking sql syntax
    let SetupResult { connection, .. } = setup_test(SetupOption {
        db_name: &format!("migration_{version}"),
        version: Some(version.clone()),
        ..Default::default()
    })
    .await;

    assert_eq!(get_database_version(&connection), version);
}
