use std::{
    sync::{Arc, RwLock},
    vec,
};

use actix_multipart::form::tempfile::TempFile;
use repository::{
    ChangelogRepository, SiteRow, SiteRowRepository, SyncBufferRowRepository, SyncFileReferenceRow,
    SyncFileReferenceRowRepository,
};
use util::format_error;

use crate::{
    service_provider::{ServiceContext, ServiceProvider},
    settings::Settings,
    static_files::{StaticFile, StaticFileCategory, StaticFileService},
    sync::{
        api::SyncApiV5, api_v6::SiteStatusV6, synchroniser::integrate_and_translate_sync_buffer,
        translations::ToSyncRecordTranslationType, CentralServerConfig,
    },
};

use super::{
    api::SyncApiSettings,
    api_v6::{
        SiteStatusRequestV6, SyncBatchV6, SyncDownloadFileRequestV6, SyncParsedErrorV6,
        SyncPullRequestV6, SyncPushRequestV6, SyncPushSuccessV6, SyncRecordV6,
        SyncUploadFileRequestV6,
    },
    translations::translate_changelogs_to_sync_records,
};

/// Send Records to a remote open-mSupply Server
pub async fn pull(
    service_provider: &ServiceProvider,
    SyncPullRequestV6 {
        cursor,
        batch_size,
        sync_v5_settings,
        is_initialised,
    }: SyncPullRequestV6,
) -> Result<SyncBatchV6, SyncParsedErrorV6> {
    use SyncParsedErrorV6 as Error;

    if !CentralServerConfig::is_central_server() {
        return Err(Error::NotACentralServer);
    }

    let ctx = service_provider.basic_context()?;
    let site = get_site_info(&ctx, sync_v5_settings)?;

    // Site should retry if we are currently integrating records for this site
    if is_integrating(site.site_id) {
        return Err(Error::IntegrationInProgress);
    }

    let changelog_repo = ChangelogRepository::new(&ctx.connection);

    // We don't need a filter here, as we are filtering in the repository layer
    let changelogs = changelog_repo.outgoing_sync_records_from_central(
        cursor,
        batch_size,
        site.site_id,
        is_initialised,
    )?;
    let total_records = changelog_repo.count_outgoing_sync_records_from_central(
        cursor,
        site.site_id,
        is_initialised,
    )?;
    let max_cursor = changelog_repo.latest_cursor()?;

    let end_cursor = changelogs
        .last()
        .map(|log| log.cursor as u64)
        .unwrap_or(max_cursor);

    let records: Vec<SyncRecordV6> = translate_changelogs_to_sync_records(
        &ctx.connection,
        changelogs,
        ToSyncRecordTranslationType::PullFromOmSupplyCentral,
    )
    .map_err(|e| Error::OtherServerError(format_error(&e)))?
    .into_iter()
    .map(SyncRecordV6::from)
    .collect();

    log::info!("Sending {} records to site {}", records.len(), site.site_id);
    log::debug!("Sending records as central server: {:#?}", records);

    let is_last_batch = total_records <= batch_size as u64;

    Ok(SyncBatchV6 {
        total_records,
        end_cursor,
        records,
        is_last_batch,
    })
}

/// Receive Records from a remote open-mSupply Server
pub async fn push(
    service_provider: Arc<ServiceProvider>,
    SyncPushRequestV6 {
        batch,
        sync_v5_settings,
    }: SyncPushRequestV6,
) -> Result<SyncPushSuccessV6, SyncParsedErrorV6> {
    use SyncParsedErrorV6 as Error;

    if !CentralServerConfig::is_central_server() {
        return Err(Error::NotACentralServer);
    }

    let ctx = service_provider.basic_context()?;
    let site = get_site_info(&ctx, sync_v5_settings)?;

    // Site should retry if we are currently integrating records for this site
    if is_integrating(site.site_id) {
        return Err(Error::IntegrationInProgress);
    }

    log::info!(
        "Receiving {}/{} records from site {}",
        batch.records.len(),
        batch.total_records,
        site.site_id
    );
    log::debug!("Receiving records as central server: {:#?}", batch);

    let SyncBatchV6 {
        records,
        is_last_batch,
        ..
    } = batch;

    let repo = SyncBufferRowRepository::new(&ctx.connection);

    let records_in_this_batch = records.len() as u64;
    for SyncRecordV6 { record, .. } in records {
        let buffer_row = record.to_buffer_row(Some(site.site_id))?;

        repo.upsert_one(&buffer_row)?;
    }

    if is_last_batch {
        spawn_integration(service_provider, site.site_id);
    }

    Ok(SyncPushSuccessV6 {
        records_pushed: records_in_this_batch,
    })
}

pub async fn get_site_status(
    service_provider: &ServiceProvider,
    SiteStatusRequestV6 { sync_v5_settings }: SiteStatusRequestV6,
) -> Result<SiteStatusV6, SyncParsedErrorV6> {
    use SyncParsedErrorV6 as Error;

    if !CentralServerConfig::is_central_server() {
        return Err(Error::NotACentralServer);
    }

    let ctx = service_provider.basic_context()?;
    let site = get_site_info(&ctx, sync_v5_settings)?;

    let is_integrating = is_integrating(site.site_id);

    Ok(SiteStatusV6 { is_integrating })
}

fn spawn_integration(service_provider: Arc<ServiceProvider>, site_id: i32) -> () {
    tokio::spawn(async move {
        let ctx = match service_provider.basic_context() {
            Ok(ctx) => ctx,
            Err(e) => {
                log::error!("Error getting basic context: {}", e);
                return;
            }
        };

        set_integrating(site_id, true);

        match integrate_and_translate_sync_buffer(&ctx.connection, true, None, Some(site_id)) {
            Ok(_) => {
                log::info!("Integration complete for site {}", site_id);
            }
            Err(e) => {
                log::error!("Error integrating records for site {}: {}", site_id, e);
            }
        }

        set_integrating(site_id, false);
    });
}

/// Send a file to a remote open-mSupply Server
pub async fn download_file(
    settings: &Settings,
    SyncDownloadFileRequestV6 {
        id,
        table_name,
        record_id,
        sync_v5_settings,
    }: SyncDownloadFileRequestV6,
) -> Result<(actix_files::NamedFile, StaticFile), SyncParsedErrorV6> {
    use SyncParsedErrorV6 as Error;

    log::info!(
        "Downloading file to remote server for table: {}, record: {}, file: {}",
        table_name,
        record_id,
        id
    );

    if !CentralServerConfig::is_central_server() {
        return Err(Error::NotACentralServer);
    }
    // Check credentials
    // let _ = get_site_info(&ctx, sync_v5_settings)?;

    let service = StaticFileService::new(&settings.server.base_dir)?;
    let static_file_category = StaticFileCategory::SyncFile(table_name, record_id);
    let file_description = service
        .find_file(&id, static_file_category.clone())?
        .ok_or(SyncParsedErrorV6::OtherServerError(
            "File not found".to_string(),
        ))?;

    let named_file =
        actix_files::NamedFile::open(&file_description.path).map_err(|e| Error::from_error(&e))?;
    Ok((named_file, file_description))
}

/// Accept a file from a remote open-mSupply Server
/// This is the endpoint that the remote server will call to upload a file
pub async fn upload_file(
    settings: &Settings,
    service_provider: &ServiceProvider,
    SyncUploadFileRequestV6 {
        file_id,
        sync_v5_settings,
    }: SyncUploadFileRequestV6,
    file_part: TempFile,
) -> Result<(), SyncParsedErrorV6> {
    use SyncParsedErrorV6 as Error;

    log::info!("Receiving a file via sync : {}", file_id);

    if !CentralServerConfig::is_central_server() {
        return Err(Error::NotACentralServer);
    }
    // Check credentials
    // let _ = get_site_info(&ctx, sync_v5_settings)?;

    let file_service = StaticFileService::new(&settings.server.base_dir)?;
    let ctx = service_provider.basic_context()?;

    let repo = SyncFileReferenceRowRepository::new(&ctx.connection);
    let sync_file_reference = repo
        .find_one_by_id(&file_id)?
        .ok_or(Error::SyncFileNotFound(file_id.clone()))?;

    file_service.move_temp_file(
        file_part,
        &StaticFileCategory::SyncFile(
            sync_file_reference.table_name.clone(),
            sync_file_reference.record_id.clone(),
        ),
        Some(file_id),
    )?;

    repo.upsert_one(&SyncFileReferenceRow {
        // Do we really need to store this ?
        // I can see total bytes could be useful, but uploaded ?
        uploaded_bytes: sync_file_reference.total_bytes,
        ..sync_file_reference
    })?;

    Ok(())
}

static SITES_BEING_INTEGRATED: RwLock<Vec<i32>> = RwLock::new(vec![]);

fn is_integrating(site_id: i32) -> bool {
    let sites_being_integrated = SITES_BEING_INTEGRATED.read().unwrap();
    sites_being_integrated.contains(&site_id)
}

fn set_integrating(site_id: i32, is_integrating: bool) {
    let mut sites_being_integrated = SITES_BEING_INTEGRATED.write().unwrap();

    if is_integrating {
        sites_being_integrated.push(site_id);
    } else {
        sites_being_integrated.retain(|id| *id != site_id);
    }
}

fn get_site_info(
    ctx: &ServiceContext,
    sync_v5_settings: SyncApiSettings,
) -> Result<SiteRow, SyncParsedErrorV6> {
    let site_repo = SiteRowRepository::new(&ctx.connection);

    let sites_for_hardware_id = site_repo.find_many_by_hardware_id(&sync_v5_settings.site_uuid)?;

    let site = sites_for_hardware_id
        .iter()
        .find(|site| site.site_name == sync_v5_settings.username);

    // TODO: check password!

    match site {
        Some(site) => Ok(site.to_owned()),
        None => Err(SyncParsedErrorV6::OtherServerError(
            "site not found".to_string(),
        )),
    }
}
