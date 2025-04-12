use std::sync::Arc;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::status::BadRequest;

use rocket::{serde::json::Json, State};

use crate::{application::config::service::ConfigService, domain::config::entities::Config, presentation::auth::guard::AuthGuard};

/// Get config
#[utoipa::path(
    get,
    path = "/api/config",
    responses(
        (status = 200, description = "Get config", body = Config)
    )
)]
#[get("/config")]
pub fn get_config(_auth: AuthGuard, config_service: &State<Arc<ConfigService>>) -> Json<Config> {
    let config = config_service.get_config().unwrap();
    Json(config)
}

#[utoipa::path(
    get,
    path = "/api/start_sploit.py",
    responses(
        (status = 200, description = "Get start_sploit.py")
    )
)]
#[get("/start_sploit.py")]
pub async fn start_sploit() -> Result<(ContentType, NamedFile), BadRequest<String>> {
    NamedFile::open("./start_sploit.py")
        .await
        .map_err(|err| BadRequest(err.to_string()))
        .map(|file| (ContentType::new("application", "x-python-code"), file))
}
