use sibears_farm::config::get_config;
use sibears_farm::handlers::flag_handler::flag_handler;
use sibears_farm::rocket_init::rocket;
use std::sync::Arc;
use std::thread;

#[tokio::main]
async fn main() {
    let config = Arc::new(get_config());
    let config_handler = config.clone();
    thread::spawn(move || {
        flag_handler(config_handler);
    });
    let _ = rocket(config).launch().await;
}
