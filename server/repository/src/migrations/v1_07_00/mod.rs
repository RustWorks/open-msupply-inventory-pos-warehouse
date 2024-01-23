use super::{version::Version, Migration};

use crate::StorageConnection;

mod currency;

pub(crate) struct V1_07_00;

impl Migration for V1_07_00 {
    fn version(&self) -> Version {
        Version::from_str("1.7.0")
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        currency::migrate(connection)?;
        Ok(())
    }
}

#[cfg(test)]
#[actix_rt::test]
async fn migration_1_07_00() {
    use crate::migrations::*;
    use crate::test_db::*;

    let version = V1_07_00.version();

    // This test allows checking sql syntax
    let SetupResult { connection, .. } = setup_test(SetupOption {
        db_name: &format!("migration_{version}"),
        version: Some(version.clone()),
        ..Default::default()
    })
    .await;

    assert_eq!(get_database_version(&connection), version);
}
