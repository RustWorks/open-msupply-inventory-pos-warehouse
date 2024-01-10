use chrono::{Duration, NaiveDateTime, NaiveTime};

use repository::{
    DatetimeFilter, EqualFilter, Pagination, PaginationOption, RepositoryError, Sensor,
    SensorFilter, SensorRepository, SensorSort, Sort, StorageConnection,
    TemperatureBreachRowRepository, TemperatureBreachRowType, TemperatureLog, TemperatureLogFilter,
    TemperatureLogRepository, TemperatureLogSortField,
};

use crate::{
    get_default_pagination, i64_to_u32, service_provider::ServiceContext, ListError, ListResult,
    SingleRecordError,
};

pub const MAX_LIMIT: u32 = 1000;
pub const MIN_LIMIT: u32 = 1;

pub fn get_sensors(
    ctx: &ServiceContext,
    pagination: Option<PaginationOption>,
    filter: Option<SensorFilter>,
    sort: Option<SensorSort>,
) -> Result<ListResult<Sensor>, ListError> {
    let pagination = get_default_pagination(pagination, MAX_LIMIT, MIN_LIMIT)?;
    let repository = SensorRepository::new(&ctx.connection);

    Ok(ListResult {
        rows: repository.query(pagination, filter.clone(), sort)?,
        count: i64_to_u32(repository.count(filter)?),
    })
}

pub fn get_sensor(ctx: &ServiceContext, id: String) -> Result<Sensor, SingleRecordError> {
    let repository = SensorRepository::new(&ctx.connection);

    let mut result =
        repository.query_by_filter(SensorFilter::new().id(EqualFilter::equal_to(&id)))?;

    if let Some(record) = result.pop() {
        Ok(record)
    } else {
        Err(SingleRecordError::NotFound(id))
    }
}

pub fn get_sensor_logs_for_breach(
    connection: &StorageConnection,
    breach_id: &String,
) -> Result<Vec<TemperatureLog>, RepositoryError> {
    let mut temperature_logs: Vec<TemperatureLog> = Vec::new();

    let breach_result =
        TemperatureBreachRowRepository::new(connection).find_one_by_id(breach_id)?;

    if let Some(breach_record) = breach_result {
        if let Some(end_datetime) = breach_record.end_datetime {
            // Find all temperature logs in the breach time range, sorted by date/time

            let mut filter = TemperatureLogFilter::new()
                .sensor(SensorFilter::new().id(EqualFilter::equal_to(&breach_record.sensor_id)));
            let sort = Sort {
                key: TemperatureLogSortField::Datetime,
                desc: None,
            };

            match breach_record.r#type {
                TemperatureBreachRowType::ColdCumulative
                | TemperatureBreachRowType::HotCumulative => {
                    // Cumulative breach can include any time on the same day (can only be at most one of hot/cold starting per day)
                    let zero_time = NaiveTime::parse_from_str("00:00", "%H:%M").unwrap(); // hard-coded -> should always work!
                    let start_breach =
                        NaiveDateTime::new(breach_record.start_datetime.date(), zero_time); // set to start of day
                    let mut end_breach = end_datetime;
                    if end_datetime.date() == start_breach.date() {
                        // If ending on the same day, then extend to midnight
                        end_breach = start_breach + Duration::days(1);
                    }
                    filter = filter.datetime(DatetimeFilter::date_range(start_breach, end_breach));
                }
                TemperatureBreachRowType::ColdConsecutive
                | TemperatureBreachRowType::HotConsecutive => {
                    filter = filter.datetime(DatetimeFilter::date_range(
                        breach_record.start_datetime,
                        end_datetime,
                    ));
                }
            }

            let log_result = TemperatureLogRepository::new(connection).query(
                Pagination::all(),
                Some(filter),
                Some(sort),
            )?;

            for temperature_log in log_result {
                // Add log to breach if temperature is outside breach parameters
                match breach_record.r#type {
                    TemperatureBreachRowType::ColdCumulative
                    | TemperatureBreachRowType::ColdConsecutive => {
                        if temperature_log.temperature_log_row.temperature
                            < breach_record.threshold_minimum
                        {
                            temperature_logs.push(temperature_log.clone());
                        }
                    }
                    TemperatureBreachRowType::HotCumulative
                    | TemperatureBreachRowType::HotConsecutive => {
                        if temperature_log.temperature_log_row.temperature
                            > breach_record.threshold_maximum
                        {
                            temperature_logs.push(temperature_log.clone());
                        }
                    }
                }
            }
        } else {
            log::info!("Breach {:?} has no end time", breach_record);
        }

        Ok(temperature_logs)
    } else {
        Err(RepositoryError::NotFound)
    }
}

#[cfg(test)]
mod test {
    use repository::{
        mock::{mock_store_a, MockData, MockDataInserts},
        SensorRow, TemperatureBreachRow, TemperatureBreachRowType, TemperatureLogRow,
    };
    use util::create_datetime;

    use crate::{
        sensor::query::get_sensor_logs_for_breach,
        test_helpers::{setup_all_with_data_and_service_provider, ServiceTestContext},
    };

    #[actix_rt::test]
    async fn test_get_sensors_for_breach() {
        let sensor1 = SensorRow {
            id: "sensor1".to_string(),
            serial: "sensor1".to_string(),
            store_id: mock_store_a().id.clone(),
            ..Default::default()
        };

        let breach1 = TemperatureBreachRow {
            id: "breach1".to_string(),
            sensor_id: sensor1.id.clone(),
            store_id: mock_store_a().id.clone(),
            start_datetime: create_datetime(2022, 06, 03, 00, 00, 00).unwrap(),
            end_datetime: create_datetime(2022, 06, 03, 18, 56, 00),
            threshold_minimum: 2.0,
            r#type: TemperatureBreachRowType::ColdCumulative,
            ..Default::default()
        };

        // Sensor 1 (S1)
        let temperature_logs: Vec<TemperatureLogRow> = vec![
            ((2022, 06, 02), (23, 51), -5.3), // 0
            ((2022, 06, 02), (23, 56), -5.3), // 1
            ((2022, 06, 03), (00, 01), -5.4), // 2
            ((2022, 06, 03), (00, 06), -5.3), // 3
            ((2022, 06, 03), (18, 31), 23.7), // 4
            ((2022, 06, 03), (18, 36), 12.9), // 5
            ((2022, 06, 03), (18, 51), -0.1), // 6
            ((2022, 06, 03), (18, 56), -1.9), // 7
            ((2022, 06, 04), (00, 01), -5.6), // 8
            ((2022, 06, 04), (00, 06), -5.6), // 9
        ]
        .into_iter()
        .map(|(date, time, temperature)| TemperatureLogRow {
            id: util::uuid::uuid(),
            temperature,
            sensor_id: sensor1.id.clone(),
            store_id: mock_store_a().id.clone(),
            datetime: create_datetime(date.0, date.1, date.2, time.0, time.1, 00).unwrap(),
            temperature_breach_id: None, // can add to above tuple for further tests
            ..Default::default()
        })
        .collect();

        let ServiceTestContext { connection, .. } = setup_all_with_data_and_service_provider(
            "test_get_sensors_for_breach",
            MockDataInserts::none().stores().names(),
            MockData {
                sensors: vec![sensor1],
                temperature_breaches: vec![breach1.clone()],
                temperature_logs: temperature_logs.clone(),
                ..MockData::default()
            },
        )
        .await;

        assert_eq!(
            get_sensor_logs_for_breach(&connection, &breach1.id).map(|logs| logs
                .into_iter()
                .map(|log| log.temperature_log_row.id)
                .collect::<Vec<String>>()),
            Ok(vec![
                temperature_logs[2].id.clone(),
                temperature_logs[3].id.clone(),
                temperature_logs[6].id.clone(),
                temperature_logs[7].id.clone()
            ])
        );
    }
}
