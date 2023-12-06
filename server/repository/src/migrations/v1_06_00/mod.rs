use super::{version::Version, Migration};

use crate::StorageConnection;

mod contact_trace;
mod invoice_add_name_link_id;
mod master_list;
mod name_link;
mod name_store_join_name_link_id;
mod plugin_data;
mod temperature_breach;

pub(crate) struct V1_06_00;

impl Migration for V1_06_00 {
    fn version(&self) -> Version {
        Version::from_str("1.6.0")
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        contact_trace::migrate(connection)?;
        plugin_data::migrate(connection)?;
        master_list::migrate(connection)?;
        temperature_breach::migrate(connection)?;
        name_link::migrate(connection)?;
        invoice_add_name_link_id::migrate(connection)?;
        name_store_join_name_link_id::migrate(connection)?;
        Ok(())
    }
}

#[cfg(test)]
#[actix_rt::test]
async fn migration_1_06_00() {
    use crate::migrations::*;
    use crate::test_db::*;

    let version = V1_06_00.version();

    // This test allows checking sql syntax
    let SetupResult { connection, .. } = setup_test(SetupOption {
        db_name: &format!("migration_{version}"),
        version: Some(version.clone()),
        ..Default::default()
    })
    .await;

    assert_eq!(get_database_version(&connection), version);
}
