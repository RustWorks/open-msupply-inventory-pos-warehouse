use super::{
    location_row::location, sensor_row::sensor, store_row::store,
    temperature_breach_row::temperature_breach,
    temperature_log_row::temperature_log::dsl as temperature_log_dsl, StorageConnection,
};

use crate::repository_error::RepositoryError;

use chrono::NaiveDateTime;
use diesel::prelude::*;

table! {
    temperature_log (id) {
        id -> Text,
        temperature -> Double,
        sensor_id -> Text,
        location_id -> Nullable<Text>,
        store_id -> Nullable<Text>,
        datetime -> Timestamp,
        temperature_breach_id -> Nullable<Text>,
    }
}

table! {
    #[sql_name = "temperature_log"]
    temperature_log_is_sync_update (id) {
        id -> Text,
        is_sync_update -> Bool,
    }
}

joinable!(temperature_log -> sensor (sensor_id));
joinable!(temperature_log -> store (store_id));
joinable!(temperature_log -> location (location_id));
joinable!(temperature_log -> temperature_breach (temperature_breach_id));

#[derive(Clone, Queryable, Insertable, AsChangeset, Debug, PartialEq, Default)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "temperature_log"]
pub struct TemperatureLogRow {
    pub id: String,
    pub temperature: f64,
    pub sensor_id: String,
    pub location_id: Option<String>,
    pub store_id: Option<String>,
    pub datetime: NaiveDateTime,
    pub temperature_breach_id: Option<String>,
}

pub struct TemperatureLogRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> TemperatureLogRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        TemperatureLogRowRepository { connection }
    }

    #[cfg(feature = "postgres")]
    pub fn _upsert_one(&self, row: &TemperatureLogRow) -> Result<(), RepositoryError> {
        diesel::insert_into(temperature_log_dsl::temperature_log)
            .values(row)
            .on_conflict(temperature_log_dsl::id)
            .do_update()
            .set(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    pub fn _upsert_one(&self, row: &TemperatureLogRow) -> Result<(), RepositoryError> {
        diesel::replace_into(temperature_log_dsl::temperature_log)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn upsert_one(&self, row: &TemperatureLogRow) -> Result<(), RepositoryError> {
        self._upsert_one(row)?;
        Ok(())
    }

    pub fn find_one_by_id(&self, id: &str) -> Result<Option<TemperatureLogRow>, RepositoryError> {
        let result = temperature_log_dsl::temperature_log
            .filter(temperature_log_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn find_many_by_id(
        &self,
        ids: &[String],
    ) -> Result<Vec<TemperatureLogRow>, RepositoryError> {
        Ok(temperature_log_dsl::temperature_log
            .filter(temperature_log_dsl::id.eq_any(ids))
            .load(&self.connection.connection)?)
    }
}
