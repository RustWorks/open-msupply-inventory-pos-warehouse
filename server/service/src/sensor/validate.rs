use repository::{
    sensor::{SensorFilter, SensorRepository},
    RepositoryError, SensorRow, SensorRowRepository, StorageConnection,
};
use repository::{EqualFilter, LocationRowRepository};

pub fn check_sensor_serial_is_unique(
    id: &str,
    serial_option: Option<String>,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    match serial_option {
        None => Ok(true),
        Some(serial) => {
            let sensors = SensorRepository::new(connection).query_by_filter(
                SensorFilter::new()
                    .serial(EqualFilter::equal_to(&serial))
                    .id(EqualFilter::not_equal_to(id))
                    .store_id(EqualFilter::equal_to("store_a")),
            )?;

            Ok(sensors.len() == 0)
        }
    }
}

pub fn check_sensor_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<Option<SensorRow>, RepositoryError> {
    Ok(SensorRowRepository::new(connection).find_one_by_id(id)?)
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