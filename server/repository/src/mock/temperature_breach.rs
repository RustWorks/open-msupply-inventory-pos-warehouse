use crate::TemperatureBreachRow;
use crate::TemperatureBreachRowType;
use chrono::{Duration, NaiveDate};

// hot breach sensor 1 in store a
pub fn mock_temperature_breach_1() -> TemperatureBreachRow {
    TemperatureBreachRow {
        id: "temperature_breach_1".to_owned(),
        acknowledged: false,
        r#type: TemperatureBreachRowType::HotConsecutive,
        store_id: Some("store_a".to_string()),
        threshold_minimum: 8.0,
        threshold_maximum: 100.0,
        threshold_duration: 3600,
        sensor_id: "sensor_1".to_owned(),
        duration: 6000,
        location_id: None,
        start_datetime: NaiveDate::from_ymd_opt(2022, 7, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + Duration::seconds(47046),
        end_datetime: NaiveDate::from_ymd_opt(2022, 7, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + Duration::seconds(53046),
    }
}

// acknowledged hot breach sensor 1 in store a
pub fn mock_temperature_breach_acknowledged() -> TemperatureBreachRow {
    TemperatureBreachRow {
        id: "temperature_breach_acknowledged".to_owned(),
        acknowledged: true,
        r#type: TemperatureBreachRowType::HotConsecutive,
        store_id: Some("store_a".to_string()),
        threshold_minimum: 8.0,
        threshold_maximum: 100.0,
        threshold_duration: 3600,
        sensor_id: "sensor_1".to_owned(),
        duration: 86400,
        location_id: None,
        start_datetime: NaiveDate::from_ymd_opt(2022, 8, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + Duration::seconds(48246),
        end_datetime: NaiveDate::from_ymd_opt(2022, 8, 2)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + Duration::seconds(48246),
    }
}

// cold breach sensor 2 in store b
pub fn mock_temperature_breach_2() -> TemperatureBreachRow {
    TemperatureBreachRow {
        id: "temperature_breach_2".to_owned(),
        acknowledged: false,
        r#type: TemperatureBreachRowType::ColdConsecutive,
        store_id: Some("store_b".to_string()),
        threshold_minimum: -273.0,
        threshold_maximum: 2.0,
        threshold_duration: 3600,
        sensor_id: "sensor_1".to_owned(),
        duration: 6000,
        location_id: None,
        start_datetime: NaiveDate::from_ymd_opt(2022, 7, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + Duration::seconds(48246),
        end_datetime: NaiveDate::from_ymd_opt(2022, 7, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            + Duration::seconds(54246),
    }
}

pub fn mock_temperature_breaches() -> Vec<TemperatureBreachRow> {
    vec![
        mock_temperature_breach_1(),
        mock_temperature_breach_acknowledged(),
        mock_temperature_breach_2(),
    ]
}
