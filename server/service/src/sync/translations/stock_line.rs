use crate::sync::{
    sync_serde::{date_option_to_isostring, empty_str_as_option_string, zero_date_as_option},
    translations::{
        item::ItemTranslation, location::LocationTranslation, name::NameTranslation,
        store::StoreTranslation,
    },
};
use chrono::NaiveDate;
use repository::{
    ChangelogRow, ChangelogTableName, EqualFilter, StockLine, StockLineFilter, StockLineRepository,
    StockLineRow, StorageConnection, SyncBufferRow,
};
use serde::{Deserialize, Serialize};

use super::{PullTranslateResult, PushTranslateResult, SyncTranslation};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct LegacyStockLineRow {
    pub ID: String,
    pub store_ID: String,
    pub item_ID: String,
    #[serde(deserialize_with = "empty_str_as_option_string")]
    pub batch: Option<String>,
    #[serde(deserialize_with = "zero_date_as_option")]
    #[serde(serialize_with = "date_option_to_isostring")]
    pub expiry_date: Option<NaiveDate>,
    pub hold: bool,
    #[serde(deserialize_with = "empty_str_as_option_string")]
    pub location_ID: Option<String>,
    pub pack_size: i32,
    pub available: f64,
    pub quantity: f64,
    pub cost_price: f64,
    pub sell_price: f64,
    #[serde(deserialize_with = "empty_str_as_option_string")]
    pub note: Option<String>,
    #[serde(rename = "name_ID")]
    #[serde(deserialize_with = "empty_str_as_option_string")]
    pub supplier_id: Option<String>,
    #[serde(deserialize_with = "empty_str_as_option_string", rename = "barcodeID")]
    pub barcode_id: Option<String>,
}
// Needs to be added to all_translators()
#[deny(dead_code)]
pub(crate) fn boxed() -> Box<dyn SyncTranslation> {
    Box::new(StockLineTranslation)
}

pub(super) struct StockLineTranslation;
impl SyncTranslation for StockLineTranslation {
    fn table_name(&self) -> &'static str {
        "item_line"
    }

    fn pull_dependencies(&self) -> Vec<&'static str> {
        vec![
            ItemTranslation.table_name(),
            NameTranslation.table_name(),
            StoreTranslation.table_name(),
            LocationTranslation.table_name(),
        ]
    }

    fn change_log_type(&self) -> Option<ChangelogTableName> {
        Some(ChangelogTableName::StockLine)
    }

    fn try_translate_from_upsert_sync_record(
        &self,
        _: &StorageConnection,
        sync_record: &SyncBufferRow,
    ) -> Result<PullTranslateResult, anyhow::Error> {
        let data = serde_json::from_str::<LegacyStockLineRow>(&sync_record.data)?;

        let result = StockLineRow {
            id: data.ID,
            store_id: data.store_ID,
            item_link_id: data.item_ID,
            location_id: data.location_ID,
            batch: data.batch,
            pack_size: data.pack_size,
            cost_price_per_pack: data.cost_price,
            sell_price_per_pack: data.sell_price,
            available_number_of_packs: data.available,
            total_number_of_packs: data.quantity,
            expiry_date: data.expiry_date,
            on_hold: data.hold,
            note: data.note,
            supplier_link_id: data.supplier_id,
            barcode_id: data.barcode_id,
        };

        Ok(PullTranslateResult::upsert(result))
    }

    fn try_translate_to_upsert_sync_record(
        &self,
        connection: &StorageConnection,
        changelog: &ChangelogRow,
    ) -> Result<PushTranslateResult, anyhow::Error> {
        let StockLineRow {
            id,
            item_id,
            store_id,
            location_id,
            batch,
            pack_size,
            cost_price_per_pack,
            sell_price_per_pack,
            available_number_of_packs,
            total_number_of_packs,
            expiry_date,
            on_hold,
            note,
            supplier_id,
            barcode_id,
        } = StockLineRowRepository::new(connection).find_one_by_id(&changelog.record_id)?;

        let legacy_row = LegacyStockLineRow {
            ID: id,
            store_ID: store_id,
            item_ID: item_row.id,
            batch,
            expiry_date,
            hold: on_hold,
            location_ID: location_id,
            pack_size,
            available: available_number_of_packs,
            quantity: total_number_of_packs,
            cost_price: cost_price_per_pack,
            sell_price: sell_price_per_pack,
            note,
            supplier_id: supplier_name_row.and_then(|supplier| Some(supplier.id)),
            barcode_id,
        };

        Ok(PushTranslateResult::upsert(
            changelog,
            self.table_name(),
            serde_json::to_value(&legacy_row)?,
        ))
    }

    fn try_translate_to_delete_sync_record(
        &self,
        _: &StorageConnection,
        changelog: &ChangelogRow,
    ) -> Result<PushTranslateResult, anyhow::Error> {
        Ok(PushTranslateResult::delete(changelog, self.table_name()))
    }
}

#[cfg(test)]
mod tests {
    use crate::sync::test::merge_helpers::{merge_all_item_links, merge_all_name_links};

    use super::*;
    use repository::{
        mock::MockDataInserts, test_db::setup_all, ChangelogFilter, ChangelogRepository,
    };
    use serde_json::json;

    #[actix_rt::test]
    async fn test_stock_line_translation() {
        use crate::sync::test::test_data::stock_line as test_data;
        let translator = StockLineTranslation {};

        let (_, connection, _, _) =
            setup_all("test_stock_line_translation", MockDataInserts::none()).await;

        for record in test_data::test_pull_upsert_records() {
            assert!(translator.should_translate_from_sync_record(&record.sync_buffer_row));
            let translation_result = translator
                .try_translate_from_upsert_sync_record(&connection, &record.sync_buffer_row)
                .unwrap();

            assert_eq!(translation_result, record.translated_record);
        }
    }

    #[actix_rt::test]
    async fn test_stock_line_push_merged() {
        // The item_links_merged function will merge ALL items into item_a, so all stock_lines should have an item_id of "item_a" regardless of their original item_id.
        let (mock_data, connection, _, _) =
            setup_all("test_stock_line_push_link_merged", MockDataInserts::all()).await;

        merge_all_item_links(&connection, &mock_data).unwrap();
        merge_all_name_links(&connection, &mock_data).unwrap();

        let repo = ChangelogRepository::new(&connection);
        let changelogs = repo
            .changelogs(
                0,
                1_000_000,
                Some(ChangelogFilter::new().table_name(ChangelogTableName::StockLine.equal_to())),
            )
            .unwrap();

        let translator = StockLineTranslation {};
        for changelog in changelogs {
            // Translate and sort
            let translated = translator
                .try_translate_push_upsert(&connection, &changelog)
                .unwrap()
                .unwrap();

            assert_eq!(translated[0].record.data["item_ID"], json!("item_a"));

            // Supplier ID can be null. We want to check if the non-null supplier_ids is "name_a".
            if translated[0].record.data["name_ID"] != json!(null) {
                assert_eq!(translated[0].record.data["name_ID"], json!("name_a"));
            }
        }
    }
}
