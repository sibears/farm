use std::{collections::HashMap, sync::Arc};

use rocket::{serde::json::Json, State};

use crate::application::config::service::ConfigService;

/// Check authentication with password
#[utoipa::path(
    post,
    path = "/api/check_auth",
    request_body = HashMap<String, String>,
    responses(
        (status = 200, description = "Authentication successful", body = String),
        (status = 401, description = "Authentication failed", body = String)
    )
)]
#[post("/check_auth", data = "<passwd>")]
pub fn check_auth(
    config_service: &State<Arc<ConfigService>>,
    passwd: Json<HashMap<String, String>>,
) -> Json<String> {
    let config = config_service.get_config().unwrap();
    if passwd.get("passwd").unwrap() == &config.auth.password {
        Json("ok".to_string())
    } else {
        Json("err".to_string())
    }
}
