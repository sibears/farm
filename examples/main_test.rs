use std::sync::Arc;
use std::thread;
use sibears_farm::config::get_config;
use sibears_farm::db::connection::DbCollection;
use sibears_farm::handlers::flag_handler::flag_handler;
use sibears_farm::rocket_init::rocket;

#[tokio::main]
async fn main() {
    let config = Arc::new(get_config("./config_test.json"));
    let config_handler = config.clone();
    let config_db = config_handler.database.lock().unwrap();
    let db_pool = DbCollection::init_db(config_db.database_url.to_string());
    db_pool.run_migrations();
    drop(config_db);
    drop(config_handler);
    let config_handler = config.clone();
    thread::spawn(move || {
        flag_handler(config_handler);
    });
    let _ = rocket(config).launch().await;
}