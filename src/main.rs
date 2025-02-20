use std::sync::{Arc, Mutex};

use rocket::serde::json::Json;
use rocket::{get, routes};
use sibears_farm::application::config::service::ConfigService;
use sibears_farm::application::sending::service::SendingService;
use sibears_farm::cors::CORS;
use sibears_farm::infrastructure::config::file_repository::FileConfigRepo;
use sibears_farm::presentation::api_docs::ApiDoc;
use sibears_farm::presentation::config::controllers::get_config;
use sibears_farm::presentation::flags::controllers::{get_flags, get_flags_count, post_flag, post_flags};
use sibears_farm::presentation::auth::controllers::check_auth;
use sibears_farm::presentation::sending::controllers::{
    force_update_waiting_flags, get_flags_for_senders, update_flags_from_sending,
};
use sibears_farm::{
    application::flags::service::FlagService,
    infrastructure::flags::postgres_repository::PostgresFlagRepo,
};
use utoipa::OpenApi;

#[tokio::main]
async fn main() {
    let config_repo = Arc::new(Mutex::new(FileConfigRepo::new("./config.json")));
    let config_service = Arc::new(ConfigService::new(config_repo));

    let config = config_service.get_config().unwrap();
    let flag_repo = Arc::new(Mutex::new(PostgresFlagRepo::new(
        &config.database.database_url.clone(),
    )));
    let flag_service = Arc::new(FlagService::new(flag_repo, config_service.clone()));

    let sending_service = SendingService::new(flag_service.clone(), config_service.clone());

    rocket::build()
        .attach(CORS)
        .manage(config_service)
        .manage(flag_service)
        .manage(sending_service)
        .mount(
            "/api",
            routes![
                get_flags,
                post_flag,
                post_flags,
                get_config,
                get_flags_for_senders,
                force_update_waiting_flags,
                update_flags_from_sending,
                check_auth,
                get_flags_count,
            ],
        )
        .mount("/api-docs", routes![serve_api_docs])
        .launch()
        .await
        .unwrap();
}

#[get("/openapi.json")]
fn serve_api_docs() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
