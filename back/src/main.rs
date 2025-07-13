#[macro_use]
extern crate rocket;

pub mod application;
pub mod cors;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod types;

use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

use application::auth::service::AuthService;
use application::config::service::ConfigService;
use application::flags::service::FlagService;
use application::metrics::service::FlagMetricsService;
use application::sending::service::SendingService;
use cors::CORS;
use domain::auth::entities::AuthEntity;
use infrastructure::config::file_repository::FileConfigRepo;
use infrastructure::flags::postgres_repository::PostgresFlagRepo;
use presentation::api_docs::ApiDoc;
use presentation::auth::controllers::check_auth;
use presentation::config::controllers::{get_config, start_sploit};
use presentation::flags::controllers::{
    get_flags, get_flags_per_page, get_stats_flags_by_status, get_total_flags, post_flag,
    post_flags,
};
use presentation::sending::controllers::{
    force_update_waiting_flags, get_flags_for_senders, update_flags_from_sending,
};
use rocket::serde::json::Json;
use rocket::{get, routes};
use rocket_prometheus::PrometheusMetrics;
use utoipa::OpenApi;

#[tokio::main]
async fn main() {
    let config_repo_path =
        env::var("CONFIG_REPO").unwrap_or_else(|_| "./config_test.json".to_string());
    let config_repo = Arc::new(FileConfigRepo::new(&config_repo_path));
    let config_service = Arc::new(ConfigService::new(config_repo));

    let config = config_service.get_config().unwrap();
    let flag_repo = Arc::new(RwLock::new(
        PostgresFlagRepo::new(&config.database.database_url.clone()).await,
    ));
    let flag_service = Arc::new(FlagService::new(flag_repo, config_service.clone()));

    let sending_service = SendingService::new(flag_service.clone(), config_service.clone());

    let auth_entity = AuthEntity::new(config.auth.password.clone());
    let auth_service = Arc::new(AuthService::new(auth_entity));

    let prometheus = PrometheusMetrics::new();
    let metrics_service = FlagMetricsService::new(&prometheus);
    metrics_service.update_flags_count(&flag_service).await;

    rocket::build()
        .attach(CORS)
        .attach(prometheus.clone())
        .manage(metrics_service)
        .manage(config_service)
        .manage(flag_service)
        .manage(sending_service)
        .manage(auth_service)
        .mount(
            "/api",
            routes![
                get_flags,
                post_flag,
                post_flags,
                get_config,
                start_sploit,
                get_flags_for_senders,
                force_update_waiting_flags,
                update_flags_from_sending,
                check_auth,
                get_flags_per_page,
                get_total_flags,
                get_stats_flags_by_status,
            ],
        )
        .mount("/api-docs", routes![serve_api_docs])
        .mount("/metrics", prometheus)
        .launch()
        .await
        .unwrap();
}

#[get("/openapi.json")]
fn serve_api_docs() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
