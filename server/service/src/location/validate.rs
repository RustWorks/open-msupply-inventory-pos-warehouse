use domain::location::LocationFilter;
use repository::LocationRepository;

use crate::{current_store_id, service_provider::ServiceConnection, WithDBError};

pub struct LocationWithCodeAlreadyExists;

pub fn check_location_code_is_unique(
    code: &String,
    connection: &ServiceConnection,
) -> Result<(), WithDBError<LocationWithCodeAlreadyExists>> {
    let locations = LocationRepository::new(connection).query_by_filter(
        LocationFilter::new()
            .code(|f| f.equal_to(code))
            .store_id(|f| f.equal_to(&current_store_id())),
    )?;

    if locations.len() > 0 {
        Err(WithDBError::Error(LocationWithCodeAlreadyExists {}))
    } else {
        Ok(())
    }
}
