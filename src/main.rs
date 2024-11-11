use std::sync::{Arc, Mutex};

use rocket::routes;
use sibears_farm::application::config::service::ConfigService;
use sibears_farm::application::sending::service::SendingService;
use sibears_farm::infrastructure::config::file_repository::FileConfigRepo;
use sibears_farm::presentation::config::controllers::get_config;
use sibears_farm::presentation::flags::controllers::{get_flags, post_flag, post_flags};
use sibears_farm::presentation::sending::controllers::get_flags_for_senders;
use sibears_farm::{
    application::flags::service::FlagService,
    infrastructure::flags::postgres_repository::PostgresFlagRepo,
};

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
                get_flags_for_senders
            ],
        )
        .launch()
        .await
        .unwrap();
}
