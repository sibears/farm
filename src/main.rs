use std::sync::{Arc, Mutex};

use rocket::routes;
use sibears_farm::{application::flags::service::FlagService, config::get_config, infrastructure::flags::postgres_repository::PostgresFlagRepo};
use sibears_farm::presentation::flags::controllers::{get_flags, post_flag}; 


#[tokio::main]
async fn main() {
    let config = Arc::new(get_config("./config_test.json"));
    let flag_repo = Arc::new(Mutex::new(PostgresFlagRepo::new(&config.database.lock().unwrap().database_url.clone())));
    let flag_service = FlagService::new(flag_repo);
    // thread::spawn(move || {
    //     flag_handler(config_handler);
    // });
    // watch_config_file(config.clone(), "./config.json");

    rocket::build()
        .manage(config)
        .manage(flag_service)
        .mount("/flag", routes![get_flags, post_flag])
        .launch()
        .await
        .unwrap();
}
