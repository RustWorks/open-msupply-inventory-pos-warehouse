use chrono::Duration;

use repository::{
    DatetimeFilter, EqualFilter, NumberFilter, Pagination, PaginationOption, RepositoryError,
    Sensor, SensorFilter, SensorRepository, SensorSort, Sort, StorageConnection,
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
    let breach_record = TemperatureBreachRowRepository::new(connection)
        .find_one_by_id(breach_id)?
        .ok_or(RepositoryError::NotFound)?;

    let Some(end_datetime) = breach_record.end_datetime else {
        log::info!("Breach {:?} has no end time", breach_record);
        return Ok(Vec::new());
    };
    // Find all temperature logs in the breach time range, sorted by date/time

    let datetime_filter = match breach_record.r#type {
        TemperatureBreachRowType::ColdCumulative | TemperatureBreachRowType::HotCumulative => {
            // Cumulative breach can include any time on the same day (can only be at most one of hot/cold starting per day)
            let start_breach = breach_record
                .start_datetime
                .date()
                .and_hms_opt(0, 0, 0)
                .unwrap(); // set to start of day
            let mut end_breach = end_datetime;
            if end_datetime.date() == start_breach.date() {
                // If ending on the same day, then extend to midnight
                end_breach = start_breach + Duration::days(1);
            }
            DatetimeFilter::date_range(start_breach, end_breach)
        }
        TemperatureBreachRowType::ColdConsecutive | TemperatureBreachRowType::HotConsecutive => {
            DatetimeFilter::date_range(breach_record.start_datetime, end_datetime)
        }
    };

    // Add temperature threashold filter
    let temperature_filter = match breach_record.r#type {
        TemperatureBreachRowType::ColdCumulative | TemperatureBreachRowType::ColdConsecutive => {
            NumberFilter::less_then(breach_record.threshold_minimum)
        }
        TemperatureBreachRowType::HotCumulative | TemperatureBreachRowType::HotConsecutive => {
            NumberFilter::greater_then(breach_record.threshold_maximum)
        }
    };

    let filter = TemperatureLogFilter::new()
        .sensor(SensorFilter::new().id(EqualFilter::equal_to(&breach_record.sensor_id)))
        .datetime(datetime_filter)
        .temperature(temperature_filter);

    let sort = Sort {
        key: TemperatureLogSortField::Datetime,
        desc: None,
    };

    let log_result = TemperatureLogRepository::new(connection).query(
        Pagination::all(),
        Some(filter),
        Some(sort),
    )?;

    Ok(log_result)
}
