use repository::{MasterListRow, StorageConnection, SyncBufferRow};

use serde::Deserialize;

use super::{
    IntegrationRecords, LegacyTableName, PullDeleteRecordTable, PullUpsertRecord, SyncTranslation,
};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct LegacyListMasterRow {
    #[serde(rename = "ID")]
    id: String,
    description: String,
    code: String,
    note: String,
}

fn match_pull_table(sync_record: &SyncBufferRow) -> bool {
    sync_record.table_name == LegacyTableName::LIST_MASTER
}
pub(crate) struct MasterListTranslation {}
impl SyncTranslation for MasterListTranslation {
    fn try_translate_pull_upsert(
        &self,
        _: &StorageConnection,
        sync_record: &SyncBufferRow,
    ) -> Result<Option<IntegrationRecords>, anyhow::Error> {
        if !match_pull_table(sync_record) {
            return Ok(None);
        }

        let data = serde_json::from_str::<LegacyListMasterRow>(&sync_record.data)?;

        let master_list = MasterListRow {
            id: data.id.clone(),
            name: data.description.clone(),
            code: data.code.clone(),
            description: data.note.clone(),
        };
        Ok(Some(IntegrationRecords::from_upsert(
            PullUpsertRecord::MasterList(master_list),
        )))
    }

    fn try_translate_pull_delete(
        &self,
        _: &StorageConnection,
        sync_record: &SyncBufferRow,
    ) -> Result<Option<IntegrationRecords>, anyhow::Error> {
        let result = match_pull_table(sync_record).then(|| {
            IntegrationRecords::from_delete(
                &sync_record.record_id,
                PullDeleteRecordTable::MasterList,
            )
        });

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use repository::{mock::MockDataInserts, test_db::setup_all};

    #[actix_rt::test]
    async fn test_master_list_translation() {
        use crate::sync::test::test_data::master_list as test_data;
        let translator = MasterListTranslation {};

        let (_, connection, _, _) = setup_all(
            "test_master_list_translation",
            MockDataInserts::none().name_tags().period_schedules(),
        )
        .await;

        for record in test_data::test_pull_upsert_records() {
            let translation_result = translator
                .try_translate_pull_upsert(&connection, &record.sync_buffer_row)
                .unwrap();

            assert_eq!(translation_result, record.translated_record);
        }

        for record in test_data::test_pull_delete_records() {
            let translation_result = translator
                .try_translate_pull_delete(&connection, &record.sync_buffer_row)
                .unwrap();

            assert_eq!(translation_result, record.translated_record);
        }
    }
}
