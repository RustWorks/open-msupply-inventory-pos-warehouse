use crate::{site::site::dsl as site_dsl, RepositoryError, StorageConnection};
use diesel::prelude::*;
use serde::Serialize;

table! {
    site (id) {
        id -> Text,
        site_id -> Integer,
        hardware_id -> Text,
        site_name -> Text,
    }
}

// todo enum for int status

#[derive(Clone, Queryable, Insertable, AsChangeset, Debug, PartialEq, Serialize)]
#[table_name = "site"]
pub struct Site {
    pub id: String,
    pub site_id: i32,
    pub hardware_id: String,
    pub site_name: String,
}

pub struct SiteRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> SiteRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        SiteRepository { connection }
    }

    #[cfg(feature = "postgres")]
    pub fn _upsert_one(&self, row: &Site) -> Result<(), RepositoryError> {
        diesel::insert_into(site_dsl::site)
            .values(row)
            .on_conflict(site_dsl::id)
            .do_update()
            .set(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    pub fn _upsert_one(&self, row: &Site) -> Result<(), RepositoryError> {
        diesel::replace_into(site_dsl::site)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn upsert_one(&self, row: &Site) -> Result<(), RepositoryError> {
        self._upsert_one(row)?;
        Ok(())
    }

    pub fn find_one_by_id(&self, id: &str) -> Result<Option<Site>, RepositoryError> {
        let result = site_dsl::site
            .filter(site_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn find_many_by_id(&self, ids: &[String]) -> Result<Vec<Site>, RepositoryError> {
        Ok(site_dsl::site
            .filter(site_dsl::id.eq_any(ids))
            .load(&self.connection.connection)?)
    }

    pub fn find_by_hardware_id(&self, hardware_id: &str) -> Result<Vec<Site>, RepositoryError> {
        Ok(site_dsl::site
            .filter(site_dsl::hardware_id.eq(hardware_id))
            .load(&self.connection.connection)?)
    }

    // woah
    pub fn get_all(&self) -> Result<Vec<Site>, RepositoryError> {
        Ok(site_dsl::site
            // .filter(site_dsl::id.eq_any(ids))
            .load(&self.connection.connection)?)
    }
}
