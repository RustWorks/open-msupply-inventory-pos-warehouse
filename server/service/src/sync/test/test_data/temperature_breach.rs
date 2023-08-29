use crate::sync::translations::{temperature_breach::{LegacyTemperatureBreachRow, LegacyTemperatureBreachType}, LegacyTableName, PullUpsertRecord};

use chrono::{Duration, NaiveDate, NaiveTime};
use repository::{TemperatureBreachRow, TemperatureBreachRowType};
use serde_json::json;

use super::{TestSyncPullRecord, TestSyncPushRecord};

const TEMPERATURE_BREACH_1: (&'static str, &'static str) = (
    "996812e0c33911eb9757779d39ae2dbd",
    r#"{
        "ID": "996812e0c33911eb9757779d39ae2dbd",
        "sensor_ID": "cf5812e0c33911eb9757779d39ae2dbd",
        "location_ID": "",
        "type": "COLD_CONSECUTIVE",
        "threshold_minimum_temperature": -273.0,
        "threshold_maximum_temperature": 2.0,
        "threshold_duration": 3600,
        "duration": 86400,
        "acknowledged": false,
        "store_ID": "store_a",
        "start_date": "2023-07-01",
        "start_time": 47046,
        "end_date": "2023-07-02",
        "end_time": 47046
    }"#,
);

pub(crate) fn test_pull_upsert_records() -> Vec<TestSyncPullRecord> {
    vec![TestSyncPullRecord::new_pull_upsert(
        LegacyTableName::TEMPERATURE_BREACH,
        TEMPERATURE_BREACH_1,
        PullUpsertRecord::TemperatureBreach(TemperatureBreachRow {
            id: TEMPERATURE_BREACH_1.0.to_string(),
            store_id: Some("store_a".to_string()),
            location_id: None,
            r#type: TemperatureBreachRowType::ColdConsecutive,
            duration: 86400,
            acknowledged: false,
            sensor_id: "cf5812e0c33911eb9757779d39ae2dbd".to_string(),
            threshold_minimum: -273.0,
            threshold_maximum: 2.0,
            threshold_duration: 3600,
            start_timestamp: NaiveDate::from_ymd_opt(2023, 7, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    + Duration::seconds(47046),
            end_timestamp: NaiveDate::from_ymd_opt(2023, 7, 2)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    + Duration::seconds(47046),
        }),
    )]
}

pub(crate) fn test_push_records() -> Vec<TestSyncPushRecord> {
    vec![TestSyncPushRecord {
        table_name: LegacyTableName::TEMPERATURE_BREACH.to_string(),
        record_id: TEMPERATURE_BREACH_1.0.to_string(),
        push_data: json!(LegacyTemperatureBreachRow {
            id: TEMPERATURE_BREACH_1.0.to_string(),
            r#type: LegacyTemperatureBreachType::ColdConsecutive,
            duration: 86400,
            acknowledged: false,
            sensor_id: "cf5812e0c33911eb9757779d39ae2dbd".to_string(),
            store_id: Some("store_a".to_string()),
            location_id: None,
            threshold_minimum: -273.0,
            threshold_maximum: 2.0,
            threshold_duration: 3600,
            start_date: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
            start_time: NaiveTime::from_hms_opt(13, 4, 6).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2023, 7, 2).unwrap(),
            end_time: NaiveTime::from_hms_opt(13, 4, 6).unwrap(),
        }),
    }]
}
