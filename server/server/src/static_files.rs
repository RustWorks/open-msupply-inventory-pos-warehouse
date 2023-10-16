use std::io::ErrorKind;

use actix_files as fs;
use actix_web::error::InternalError;
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web::web::Data;
use actix_web::{get, guard, web, Error, HttpRequest, HttpResponse};
use reqwest::StatusCode;
use serde::Deserialize;
use service::plugin_files::{PluginFileService, PluginInfo};
use service::settings::Settings;
use service::static_files::StaticFileService;

// this function could be located in different module
pub fn config_static_files(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/files").guard(guard::Get()).to(files));
    cfg.service(plugins);
}

#[derive(Debug, Deserialize)]
pub struct FileRequestQuery {
    id: String,
}

async fn files(
    req: HttpRequest,
    query: web::Query<FileRequestQuery>,
    settings: Data<Settings>,
) -> Result<HttpResponse, Error> {
    let service = StaticFileService::new(&settings.server.base_dir)
        .map_err(|err| InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR))?;
    let file = service
        .find_file(&query.id)
        .map_err(|err| InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(std::io::Error::new(
            ErrorKind::NotFound,
            "Static file not found",
        ))?;

    let response = fs::NamedFile::open(file.path)?
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![DispositionParam::Filename(file.name)],
        })
        .into_response(&req);

    Ok(response)
}

#[get(r#"/plugins/{plugin}/{filename:.*\..+$}"#)]
async fn plugins(
    req: HttpRequest,
    plugin_info: web::Path<PluginInfo>,
    settings: Data<Settings>,
) -> Result<HttpResponse, Error> {
    let file = PluginFileService::find_file(&settings.server.base_dir, &plugin_info)
        .map_err(|err| InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(std::io::Error::new(ErrorKind::NotFound, "Plugin not found"))?;

    let response = fs::NamedFile::open(file.path)?
        .set_content_type("application/javascript; charset=utf-8".parse().unwrap())
        .into_response(&req);

    Ok(response)
}
