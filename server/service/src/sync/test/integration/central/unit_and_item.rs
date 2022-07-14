use crate::sync::{
    test::{
        integration::{
            central_server_configurations::NewSiteProperties, SyncRecordTester, TestStepData,
        },
        test_data::item::test_pull_upsert_records,
    },
    translations::{IntegrationRecords, PullDeleteRecord, PullDeleteRecordTable, PullUpsertRecord},
};
use repository::{ItemRow, ItemRowType, UnitRow};

use serde_json::json;
use util::{merge_json, uuid::uuid};

pub(crate) struct UnitAndItemTester;

impl SyncRecordTester for UnitAndItemTester {
    fn test_step_data(&self, _: &NewSiteProperties) -> Vec<TestStepData> {
        let mut result = Vec::new();
        // STEP 1 - insert
        let unit_row1 = UnitRow {
            id: uuid(),
            name: uuid(),
            description: None,
            index: 1,
        };
        let unit_json1 = json!({
            "ID": unit_row1.id,
            "units":  unit_row1.name,
            "comment": "",
            "order_number": 1 as u32
        });

        let unit_row2 = UnitRow {
            id: uuid(),
            name: uuid(),
            description: Some("test description".to_string()),
            index: 2,
        };
        let unit_json2 = json!({
            "ID": unit_row2.id,
            "units":  unit_row2.name,
            "comment": "test description",
            "order_number": 2 as u32
        });

        let mut item_row1 = ItemRow {
            id: uuid(),
            name: uuid(),
            code: uuid(),
            unit_id: None,
            r#type: ItemRowType::NonStock,
            legacy_record: "".to_string(),
        };
        let item_json1 = extend_base(json!({
            "ID": item_row1.id,
            "item_name": item_row1.name,
            "code": item_row1.code,
            "type_of": "non_stock",
            "unit_ID": ""
        }));
        item_row1.legacy_record = serde_json::to_string(&item_json1).unwrap();

        let mut item_row2 = ItemRow {
            id: uuid(),
            name: uuid(),
            code: uuid(),
            unit_id: Some(unit_row1.id.clone()),
            r#type: ItemRowType::Stock,
            legacy_record: "".to_string(),
        };
        let item_json2 = extend_base(json!({
            "ID": item_row2.id,
            "item_name": item_row2.name,
            "code": item_row2.code,
            "type_of": "general",
            "unit_ID": unit_row1.id,
        }));
        item_row2.legacy_record = serde_json::to_string(&item_json2).unwrap();

        let mut item_row3 = ItemRow {
            id: uuid(),
            name: uuid(),
            code: uuid(),
            unit_id: None,
            r#type: ItemRowType::Service,
            legacy_record: "".to_string(),
        };
        let item_json3 = extend_base(json!({
            "ID": item_row3.id,
            "item_name": item_row3.name,
            "code": item_row3.code,
            "type_of": "service",
            "unit_ID": "",
        }));
        item_row3.legacy_record = serde_json::to_string(&item_json3).unwrap();

        result.push(TestStepData {
            central_upsert:   json!({ "item": [item_json1,item_json2,item_json3], "unit": [unit_json1, unit_json2] }),
            central_delete: json!({}),
            integration_records: IntegrationRecords::from_upserts(vec![
                PullUpsertRecord::Unit(unit_row1.clone()),
                PullUpsertRecord::Unit(unit_row2),
                PullUpsertRecord::Item(item_row1),
                PullUpsertRecord::Item(item_row2.clone()),
                PullUpsertRecord::Item(item_row3),
            ]),
        });
        // STEP 2 - deletes
        result.push(TestStepData {
            central_upsert: json!({}),
            central_delete: json!({ "item": [item_row2.id], "unit": [unit_row1.id] }),
            integration_records: IntegrationRecords::from_deletes(vec![
                PullDeleteRecord {
                    id: unit_row1.id,
                    table: PullDeleteRecordTable::Unit,
                },
                PullDeleteRecord {
                    id: item_row2.id,
                    table: PullDeleteRecordTable::Item,
                },
            ]),
        });
        result
    }
}

fn extend_base(value: serde_json::Value) -> serde_json::Value {
    let mut base =
        serde_json::from_str(&test_pull_upsert_records()[0].sync_buffer_row.data).unwrap();
    merge_json(&mut base, &value);
    base
}
