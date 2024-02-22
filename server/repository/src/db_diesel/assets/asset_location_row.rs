use super::asset_location_row::asset_location::dsl::*;

use crate::RepositoryError;
use crate::StorageConnection;

use chrono::NaiveDateTime;
use diesel::prelude::*;

table! {
    asset_location (id) {
        id -> Text,
        asset_id -> Text,
        location_id -> Text,
        created_datetime -> Timestamp,
        modified_datetime -> Timestamp,
    }
}

#[derive(Clone, Insertable, Queryable, Debug, PartialEq, AsChangeset, Eq)]
#[table_name = "asset_location"]
pub struct AssetLocationRow {
    pub id: String,
    pub asset_id: String,
    pub location_id: String,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

pub struct AssetLocationRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> AssetLocationRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        AssetLocationRowRepository { connection }
    }

    #[cfg(feature = "postgres")]
    pub fn upsert_one(&self, asset_location_row: &AssetLocationRow) -> Result<(), RepositoryError> {
        diesel::insert_into(asset_location)
            .values(asset_location_row)
            .on_conflict(id)
            .do_update()
            .set(asset_location_row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    pub fn upsert_one(&self, asset_location_row: &AssetLocationRow) -> Result<(), RepositoryError> {
        diesel::replace_into(asset_location)
            .values(asset_location_row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn insert_one(&self, asset_location_row: &AssetLocationRow) -> Result<(), RepositoryError> {
        diesel::insert_into(asset_location)
            .values(asset_location_row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_all_by_location(
        &self,
        some_location_id: String,
    ) -> Result<Vec<AssetLocationRow>, RepositoryError> {
        let result = asset_location
            .filter(location_id.eq(some_location_id))
            .load(&self.connection.connection);
        Ok(result?)
    }

    pub fn find_all_by_asset(
        &self,
        some_asset_id: String,
    ) -> Result<Vec<AssetLocationRow>, RepositoryError> {
        let result = asset_location
            .filter(asset_id.eq(some_asset_id))
            .load(&self.connection.connection);
        Ok(result?)
    }

    pub fn find_one_by_id(
        &self,
        asset_location_id: &str,
    ) -> Result<Option<AssetLocationRow>, RepositoryError> {
        let result = asset_location
            .filter(id.eq(asset_location_id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn delete(&self, asset_location_id: &str) -> Result<(), RepositoryError> {
        diesel::delete(asset_location)
            .filter(id.eq(asset_location_id))
            .execute(&self.connection.connection)?;
        Ok(())
    }
}
