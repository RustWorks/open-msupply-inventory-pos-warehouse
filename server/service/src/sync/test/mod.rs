#[cfg(feature = "integration_test")]
mod integration;
pub(crate) mod merge_helpers;
mod pull_and_push;
pub(crate) mod test_data;

use super::translations::{IntegrationOperation, PullTranslateResult};
use repository::{mock::MockData, *};
use util::inline_init;

pub(crate) struct TestSyncPullRecord {
    /// Expected result for the imported data
    pub(crate) translated_record: PullTranslateResult,
    /// Row as stored in the remote sync buffer
    pub(crate) sync_buffer_row: SyncBufferRow,
    // Extra data that translation test relies on
    pub(crate) extra_data: Option<MockData>,
}

impl TestSyncPullRecord {
    fn new_pull_upsert<U>(
        table_name: &str,
        // .0 = id .1 = data
        id_and_data: (&str, &str),
        upsert: U,
    ) -> TestSyncPullRecord
    where
        U: Upsert + 'static,
    {
        TestSyncPullRecord {
            translated_record: PullTranslateResult::upsert(upsert),
            sync_buffer_row: inline_init(|r: &mut SyncBufferRow| {
                r.table_name = table_name.to_owned();
                r.record_id = id_and_data.0.to_owned();
                r.data = id_and_data.1.to_owned();
                r.action = SyncBufferAction::Upsert;
            }),
            extra_data: None,
        }
    }

    fn new_pull_delete<U>(table_name: &str, id: &str, result: U) -> TestSyncPullRecord
    where
        U: Delete + 'static,
    {
        Self::new_pull_deletes(table_name, id, vec![result])
    }
    fn new_pull_deletes<U>(table_name: &str, id: &str, deletes: Vec<U>) -> TestSyncPullRecord
    where
        U: Delete + 'static,
    {
        TestSyncPullRecord {
            translated_record: PullTranslateResult::deletes(deletes),
            sync_buffer_row: inline_init(|r: &mut SyncBufferRow| {
                r.table_name = table_name.to_owned();
                r.record_id = id.to_string();
                r.data = "{}".to_string();
                r.action = SyncBufferAction::Delete;
            }),
            extra_data: None,
        }
    }

    pub(crate) async fn insert_extra_data(&self, connection: &StorageConnection) {
        if let Some(data) = &self.extra_data {
            data.insert(connection);
        }
    }
}

/// To be used in combination with TestSyncPullRecord.
/// I.e. first run and integrate a row from TestSyncPullRecord and then try to push this record out
#[derive(Debug)]
pub struct TestSyncPushRecord {
    /// Record id for the row to be pushed.
    /// Its assumed the row exists, e.g. because it has been integrated before through a
    /// TestSyncPullRecord
    pub record_id: String,
    pub table_name: String,
    /// Expected record as pushed out to the server
    pub push_data: serde_json::Value,
}

pub(crate) fn extract_sync_buffer_rows(records: &Vec<TestSyncPullRecord>) -> Vec<SyncBufferRow> {
    records
        .into_iter()
        .map(|test_record| test_record.sync_buffer_row.clone())
        .collect()
}

pub(crate) async fn insert_all_extra_data(
    records: &Vec<TestSyncPullRecord>,
    connection: &StorageConnection,
) {
    for record in records {
        record.insert_extra_data(connection).await
    }
}

pub(crate) async fn check_test_records_against_database(
    con: &StorageConnection,
    test_records: Vec<TestSyncPullRecord>,
) {
    for test_record in test_records {
        let translated_records = match test_record.translated_record {
            PullTranslateResult::IntegrationOperations(translated_record) => translated_record,
            // Should this throw an assertion ?
            _ => continue,
        };
        check_integrated(con, translated_records)
    }
}

pub(crate) fn check_integrated(
    con: &StorageConnection,
    integration_records: Vec<IntegrationOperation>,
) {
    for record in integration_records {
        match record {
            IntegrationOperation::Upsert(upsert) => upsert.assert_upserted(con),
            IntegrationOperation::Delete(delete) => delete.assert_deleted(con),
        }
    }
}
