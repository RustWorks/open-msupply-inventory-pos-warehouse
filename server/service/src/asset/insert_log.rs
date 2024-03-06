use std::ops::Not;

use super::{
    query_log::get_asset_log,
    validate::{check_asset_exists, check_asset_log_exists},
};
use crate::{service_provider::ServiceContext, SingleRecordError};
use chrono::Utc;
use repository::{
    assets::asset_log_row::{AssetLogRow, AssetLogRowRepository},
    RepositoryError, StorageConnection,
};

#[derive(PartialEq, Debug)]
pub enum InsertAssetLogError {
    AssetLogAlreadyExists,
    AssetDoesNotExist,
    CreatedRecordNotFound,
    DatabaseError(RepositoryError),
}

pub struct InsertAssetLog {
    pub id: String,
    pub asset_id: String,
    pub status: Option<String>,
}

pub fn insert_asset_log(
    ctx: &ServiceContext,
    input: InsertAssetLog,
) -> Result<AssetLogRow, InsertAssetLogError> {
    let asset_log = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&input, connection)?;
            let new_asset_log = generate(input);
            AssetLogRowRepository::new(&connection).upsert_one(&new_asset_log)?;

            get_asset_log(ctx, new_asset_log.id).map_err(InsertAssetLogError::from)
        })
        .map_err(|error| error.to_inner_error())?;
    Ok(asset_log)
}

pub fn validate(
    input: &InsertAssetLog,
    connection: &StorageConnection,
) -> Result<(), InsertAssetLogError> {
    if check_asset_log_exists(&input.id, connection)?.is_some() {
        return Err(InsertAssetLogError::AssetLogAlreadyExists);
    }
    if check_asset_exists(&input.asset_id, connection)?
        .is_some()
        .not()
    {
        return Err(InsertAssetLogError::AssetDoesNotExist);
    }
    Ok(())
}

pub fn generate(
    InsertAssetLog {
        id,
        asset_id,
        status,
    }: InsertAssetLog,
) -> AssetLogRow {
    AssetLogRow {
        id,
        asset_id,
        status,
        log_datetime: Utc::now().naive_utc(),
    }
}

impl From<RepositoryError> for InsertAssetLogError {
    fn from(error: RepositoryError) -> Self {
        InsertAssetLogError::DatabaseError(error)
    }
}

impl From<SingleRecordError> for InsertAssetLogError {
    fn from(error: SingleRecordError) -> Self {
        use InsertAssetLogError::*;
        match error {
            SingleRecordError::DatabaseError(error) => DatabaseError(error),
            SingleRecordError::NotFound(_) => CreatedRecordNotFound,
        }
    }
}