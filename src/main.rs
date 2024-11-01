use std::sync::{Arc, Mutex};

use rocket::routes;
use sibears_farm::application::config::service::ConfigService;
use sibears_farm::infrastructure::config::file_repository::FileConfigRepo;
use sibears_farm::presentation::config::controllers::get_config;
use sibears_farm::{application::flags::service::FlagService, infrastructure::flags::postgres_repository::PostgresFlagRepo};
use sibears_farm::presentation::flags::controllers::{get_flags, post_flag, post_flags}; 

#[tokio::main]
async fn main() {
    let config_repo = Arc::new(Mutex::new(FileConfigRepo::new("./config_test.json")));
    let config_service = Arc::new(ConfigService::new(config_repo));

    let config = config_service.get_config().unwrap();
    let flag_repo = Arc::new(Mutex::new(PostgresFlagRepo::new(&config.database.database_url.clone())));
    let flag_service = FlagService::new(flag_repo, config_service.clone());
    // thread::spawn(move || {
    //     flag_handler(config_handler);
    // });
    // watch_config_file(config.clone(), "./config.json");

    rocket::build()
        .manage(config_service)
        .manage(flag_service)
        .mount("/api", routes![get_flags, post_flag, post_flags, get_config])
        .launch()
        .await
        .unwrap();
}
