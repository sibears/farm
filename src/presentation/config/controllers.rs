use std::sync::Arc;

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
