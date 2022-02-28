use chrono::NaiveDate;
use repository::{
    mock::{mock_item_a, mock_stock_line_a},
    schema::{
        ChangelogAction, ChangelogRow, ChangelogTableName, InvoiceLineRow, InvoiceLineRowType,
        RemoteSyncBufferAction, RemoteSyncBufferRow,
    },
};
use serde_json::json;

use crate::sync::translation_remote::{
    invoice_line::{LegacyTransLineRow, LegacyTransLineType},
    pull::{IntegrationRecord, IntegrationUpsertRecord},
    test_data::TestSyncRecord,
    TRANSLATION_RECORD_TRANS_LINE,
};

use super::TestSyncPushRecord;

const TRANS_LINE_1: (&'static str, &'static str) = (
    "12ee2f10f0d211eb8dddb54df6d741bc",
    r#"{
        "ID": "12ee2f10f0d211eb8dddb54df6d741bc",
        "Weight": 0,
        "barcodeID": "",
        "batch": "stocktake_1",
        "box_number": "",
        "cost_price": 0,
        "custom_data": null,
        "donor_id": "",
        "expiry_date": "0000-00-00",
        "foreign_currency_price": 0,
        "goods_received_lines_ID": "",
        "isVVMPassed": "",
        "is_from_inventory_adjustment": true,
        "item_ID": "item_a",
        "item_line_ID": "item_a_line_a",
        "item_name": "Item A",
        "line_number": 1,
        "linked_trans_line_ID": "",
        "linked_transact_id": "",
        "local_charge_line_total": 0,
        "location_ID": "",
        "manufacturer_ID": "",
        "medicine_administrator_ID": "",
        "note": "",
        "optionID": "",
        "order_lines_ID": "",
        "pack_inners_in_outer": 0,
        "pack_size": 1,
        "pack_size_inner": 0,
        "prescribedQuantity": 0,
        "price_extension": 0,
        "quantity": 700,
        "repeat_ID": "",
        "sell_price": 0,
        "sentQuantity": 0,
        "sent_pack_size": 1,
        "source_backorder_id": "",
        "spare": 0,
        "supp_trans_line_ID_ns": "",
        "transaction_ID": "outbound_shipment_a",
        "type": "stock_in",
        "user_1": "",
        "user_2": "",
        "user_3": "",
        "user_4": "",
        "user_5_ID": "",
        "user_6_ID": "",
        "user_7_ID": "",
        "user_8_ID": "",
        "vaccine_vial_monitor_status_ID": "",
        "volume_per_pack": 0
        }
    "#,
);
fn trans_line_1_pull_record() -> TestSyncRecord {
    TestSyncRecord {
        translated_record: Some(IntegrationRecord::from_upsert(
            IntegrationUpsertRecord::InvoiceLine(InvoiceLineRow {
                id: TRANS_LINE_1.0.to_string(),
                invoice_id: "outbound_shipment_a".to_string(),
                item_id: mock_item_a().id,
                item_name: mock_item_a().name,
                item_code: mock_item_a().code,
                stock_line_id: Some(mock_stock_line_a().id),
                location_id: None,
                batch: Some("stocktake_1".to_string()),
                expiry_date: None,
                pack_size: 1,
                cost_price_per_pack: 0.0,
                sell_price_per_pack: 0.0,
                total_before_tax: 0.0,
                total_after_tax: 0.0,
                tax: None,
                r#type: InvoiceLineRowType::StockIn,
                number_of_packs: 700,
                note: None,
            }),
        )),
        identifier: "Transact line 1",
        remote_sync_buffer_row: RemoteSyncBufferRow {
            id: "Transact_line_10".to_string(),
            table_name: TRANSLATION_RECORD_TRANS_LINE.to_string(),
            record_id: TRANS_LINE_1.0.to_string(),
            data: TRANS_LINE_1.1.to_string(),
            action: RemoteSyncBufferAction::Update,
        },
    }
}
fn trans_line_1_push_record() -> TestSyncPushRecord {
    TestSyncPushRecord {
        change_log: ChangelogRow {
            id: 2,
            table_name: ChangelogTableName::InvoiceLine,
            row_id: TRANS_LINE_1.0.to_string(),
            row_action: ChangelogAction::Upsert,
        },
        push_data: json!(LegacyTransLineRow {
            ID: TRANS_LINE_1.0.to_string(),
            transaction_ID: "outbound_shipment_a".to_string(),
            item_ID: mock_item_a().id,
            item_name: mock_item_a().name,
            item_line_ID: Some(mock_stock_line_a().id),
            location_ID: None,
            batch: Some("stocktake_1".to_string()),
            expiry_date: None,
            pack_size: 1,
            cost_price: 0.0,
            sell_price: 0.0,
            _type: LegacyTransLineType::StockIn,
            quantity: 700,
            note: None
        }),
    }
}

// placeholder
const TRANS_LINE_2: (&'static str, &'static str) = (
    "C9A2D5854A15457388C8266D95DE1945",
    r#"{
        "ID": "C9A2D5854A15457388C8266D95DE1945",
        "Weight": 0,
        "barcodeID": "",
        "batch": "",
        "box_number": "",
        "cost_price": 5,
        "custom_data": null,
        "donor_id": "",
        "expiry_date": "2022-02-22",
        "foreign_currency_price": 0,
        "goods_received_lines_ID": "",
        "isVVMPassed": "",
        "is_from_inventory_adjustment": false,
        "item_ID": "item_a",
        "item_line_ID": "item_a_line_a",
        "item_name": "Item A",
        "line_number": 1,
        "linked_trans_line_ID": "",
        "linked_transact_id": "",
        "local_charge_line_total": 0,
        "location_ID": "",
        "manufacturer_ID": "",
        "medicine_administrator_ID": "",
        "note": "every FOUR to SIX hours when necessary ",
        "optionID": "",
        "order_lines_ID": "",
        "pack_inners_in_outer": 0,
        "pack_size": 5,
        "pack_size_inner": 0,
        "prescribedQuantity": 0,
        "price_extension": 0,
        "quantity": 1000,
        "repeat_ID": "",
        "sell_price": 10,
        "sentQuantity": 0,
        "sent_pack_size": 100,
        "source_backorder_id": "",
        "spare": 0,
        "supp_trans_line_ID_ns": "",
        "transaction_ID": "outbound_shipment_a",
        "type": "placeholder",
        "user_1": "",
        "user_2": "",
        "user_3": "",
        "user_4": "",
        "user_5_ID": "",
        "user_6_ID": "",
        "user_7_ID": "",
        "user_8_ID": "",
        "vaccine_vial_monitor_status_ID": "",
        "volume_per_pack": 0
    }"#,
);
fn trans_line_2_pull_record() -> TestSyncRecord {
    TestSyncRecord {
        translated_record: Some(IntegrationRecord::from_upsert(
            IntegrationUpsertRecord::InvoiceLine(InvoiceLineRow {
                id: TRANS_LINE_2.0.to_string(),
                invoice_id: "outbound_shipment_a".to_string(),
                item_id: mock_item_a().id,
                item_name: mock_item_a().name,
                item_code: mock_item_a().code,
                stock_line_id: Some(mock_stock_line_a().id),
                location_id: None,
                batch: None,
                expiry_date: Some(NaiveDate::from_ymd(2022, 02, 22)),
                pack_size: 5,
                cost_price_per_pack: 5.0,
                sell_price_per_pack: 10.0,
                total_before_tax: 5.0 * 1000.0,
                total_after_tax: 5.0 * 1000.0,
                tax: None,
                r#type: InvoiceLineRowType::UnallocatedStock,
                number_of_packs: 200,
                note: Some("every FOUR to SIX hours when necessary ".to_string()),
            }),
        )),
        identifier: "Transact line (Placeholder)",
        remote_sync_buffer_row: RemoteSyncBufferRow {
            id: "Transact_line_20".to_string(),
            table_name: TRANSLATION_RECORD_TRANS_LINE.to_string(),
            record_id: TRANS_LINE_2.0.to_string(),
            data: TRANS_LINE_2.1.to_string(),
            action: RemoteSyncBufferAction::Update,
        },
    }
}
fn trans_line_2_push_record() -> TestSyncPushRecord {
    TestSyncPushRecord {
        change_log: ChangelogRow {
            id: 2,
            table_name: ChangelogTableName::InvoiceLine,
            row_id: TRANS_LINE_2.0.to_string(),
            row_action: ChangelogAction::Upsert,
        },
        push_data: json!(LegacyTransLineRow {
            ID: TRANS_LINE_2.0.to_string(),
            transaction_ID: "outbound_shipment_a".to_string(),
            item_ID: mock_item_a().id,
            item_name: mock_item_a().name,
            item_line_ID: Some(mock_stock_line_a().id),
            location_ID: None,
            batch: None,
            expiry_date: Some(NaiveDate::from_ymd(2022, 02, 22)),
            pack_size: 5,
            cost_price: 5.0,
            sell_price: 10.0,
            _type: LegacyTransLineType::Placeholder,
            quantity: 1000,
            note: Some("every FOUR to SIX hours when necessary ".to_string()),
        }),
    }
}

#[allow(dead_code)]
pub fn get_test_trans_line_records() -> Vec<TestSyncRecord> {
    vec![trans_line_1_pull_record(), trans_line_2_pull_record()]
}

#[allow(dead_code)]
pub fn get_test_push_trans_line_records() -> Vec<TestSyncPushRecord> {
    vec![trans_line_1_push_record(), trans_line_2_push_record()]
}
