use actix_web::{web::Data, Error, HttpRequest, HttpResponse};
use service::auth_data::AuthData;
use service::service_provider::ServiceProvider;
use service::settings::Settings;

#[cfg(feature = "postgres")]
pub async fn get_database(
    _request: HttpRequest,
    _service_provider: Data<ServiceProvider>,
    _auth_data: Data<AuthData>,
    _settings: Data<Settings>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::InternalServerError().body("Postgres Databases export not supported"))
}

#[cfg(not(feature = "postgres"))]
pub async fn get_database(
    request: HttpRequest,
    service_provider: Data<ServiceProvider>,
    auth_data: Data<AuthData>,
    settings: Data<Settings>,
) -> Result<HttpResponse, Error> {
    use actix_files as fs;
    use actix_web::http::header::ContentDisposition;
    use actix_web::http::header::DispositionParam;
    use actix_web::http::header::DispositionType;
    use std::path::Path;

    // TODO Authentication and Permissions Check!

    let db_path = settings.database.connection_string(); // TODO: Merge https://github.com/msupply-foundation/open-msupply/pull/2899    database_path(&self)
    let path = Path::new(&db_path);

    let response = fs::NamedFile::open(path)?
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![DispositionParam::Filename(
                settings.database.database_name.clone(),
            )],
        })
        .into_response(&request);

    Ok(response)
}
