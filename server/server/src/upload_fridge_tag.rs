use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};
use anyhow::Context;

use serde::Deserialize;

use service::{
    sensor::berlinger::{read_sensors, ReadSensor},
    service_provider::ServiceProvider,
    settings::Settings,
};
use util::perpare_file_dir;

const TEMP_FRIDGETAG_FILE_DIR: &'static str = "fridge_tag";

// this function could be located in different module
pub fn config_upload_fridge_tag(cfg: &mut web::ServiceConfig) {
    cfg.service(upload);
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct UrlParams {
    store_id: String,
}

#[post("/fridge-tag")]
async fn upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    url_params: web::Query<UrlParams>,
    settings: Data<Settings>,
    service_provider: Data<ServiceProvider>,
) -> HttpResponse {
    // TODO Permissions

    match upload_fridge_tag(form, url_params.into_inner(), &settings, &service_provider) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error) => HttpResponse::InternalServerError().body(format!("{:#?}", error)),
    }
}

fn upload_fridge_tag(
    mut form: UploadForm,
    url_params: UrlParams,
    settings: &Settings,
    service_provider: &ServiceProvider,
) -> anyhow::Result<ReadSensor> {
    let ctx = service_provider
        .basic_context()
        .context("Cannot get connection")?;

    let file = form.files.pop().context("Cannot find attached file")?;
    let file_name = file.file_name.context("Filename is not specified")?;

    let dir = perpare_file_dir(TEMP_FRIDGETAG_FILE_DIR, &settings.server.base_dir)?;

    let new_file_path = dir.join(file_name);

    // Move file
    std::fs::rename(file.file.path(), &new_file_path)?;

    ctx.connection
        .transaction_sync(|con| {
            read_sensors(&con, &url_params.store_id, new_file_path)
                .context("Error while integrating sensor data")
        })
        .map_err(|error| error.to_inner_error())
}
