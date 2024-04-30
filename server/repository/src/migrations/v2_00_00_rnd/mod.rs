use super::{version::Version, Migration};

use crate::StorageConnection;

mod site;

pub(crate) struct V2_00_00RND;

impl Migration for V2_00_00RND {
    fn version(&self) -> Version {
        Version::from_str("2.0.0-rnd")
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        site::migrate(connection)?;
        Ok(())
    }
}

#[cfg(test)]
#[actix_rt::test]
async fn migration_2_00_00_rnd() {
    use crate::migrations::*;
    use crate::test_db::*;

    let version = V2_00_00RND.version();

    // This test allows checking sql syntax
    let SetupResult { connection, .. } = setup_test(SetupOption {
        db_name: &format!("migration_{version}"),
        version: Some(version.clone()),
        ..Default::default()
    })
    .await;

    assert_eq!(get_database_version(&connection), version);
}
