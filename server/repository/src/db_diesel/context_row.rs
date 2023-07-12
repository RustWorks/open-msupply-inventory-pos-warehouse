use super::StorageConnection;

use crate::repository_error::RepositoryError;

use diesel::prelude::*;

pub const PATIENT_CONTEXT_ID: &str = "Patient";

table! {
    context (id) {
        id -> Text,
        name -> Text,
    }
}

#[derive(Clone, Insertable, Queryable, Debug, PartialEq, AsChangeset, Eq)]
#[table_name = "context"]
pub struct ContextRow {
    pub id: String,
    pub name: String,
}

pub struct ContextRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> ContextRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        ContextRowRepository { connection }
    }

    #[cfg(feature = "postgres")]
    pub fn upsert_one(&self, row: &ContextRow) -> Result<(), RepositoryError> {
        diesel::insert_into(context::dsl::context)
            .values(row)
            .on_conflict(context::dsl::id)
            .do_update()
            .set(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    pub fn upsert_one(&self, row: &ContextRow) -> Result<(), RepositoryError> {
        diesel::replace_into(context::dsl::context)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub async fn insert_one(&self, row: &ContextRow) -> Result<(), RepositoryError> {
        diesel::insert_into(context::dsl::context)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<ContextRow>, RepositoryError> {
        let result = context::dsl::context.load(&self.connection.connection);
        Ok(result?)
    }

    pub fn find_one_by_id(&self, row_id: &str) -> Result<Option<ContextRow>, RepositoryError> {
        let result = context::dsl::context
            .filter(context::dsl::id.eq(row_id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
}
