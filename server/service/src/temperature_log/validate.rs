use repository::{
    EqualFilter, LocationRowRepository, RepositoryError, StorageConnection, TemperatureLogFilter,
    TemperatureLogRepository, TemperatureLogRow, TemperatureLogRowRepository,
};

pub fn check_temperature_log_does_not_exist(
    id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let temperature_logs = TemperatureLogRepository::new(connection)
        .query_by_filter(TemperatureLogFilter::new().id(EqualFilter::equal_to(id)))?;

    Ok(temperature_logs.len() == 0)
}

pub fn check_temperature_log_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<Option<TemperatureLogRow>, RepositoryError> {
    Ok(TemperatureLogRowRepository::new(connection).find_one_by_id(id)?)
}

pub fn check_location_on_hold(
    location_id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let location = LocationRowRepository::new(connection)
        .find_one_by_id(location_id)?
        .ok_or(RepositoryError::NotFound)?;

    Ok(location.on_hold)
}
