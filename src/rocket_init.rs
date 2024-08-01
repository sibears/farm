extern crate diesel;
extern crate serde_json;

use rocket::{Build, Rocket};
use rocket_prometheus::PrometheusMetrics;
use std::sync::Arc;

use crate::controllers::config::*;
use crate::controllers::flag::*;
use crate::controllers::statistic::*;
use crate::db::connection::init_db;
use crate::middleware::cors::CORS;
use crate::middleware::metrics::FLAG_COUNTER;
use crate::settings::Config;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, SiBears Farm!"
}

pub fn rocket(config: Arc<Config>) -> Rocket<Build> {
    let _ = dotenv::dotenv().map_err(|err| error!("Dotenv: {:?}", err));
    let database_url = config.database.lock().unwrap().database_url.to_string();
    let db_collection = init_db(database_url);
    let prometheus = PrometheusMetrics::new();
    prometheus
        .registry()
        .register(Box::new(FLAG_COUNTER.clone()))
        .unwrap();
    let rocket_app = rocket::build()
        .attach(prometheus.clone())
        .attach(CORS)
        .manage(CORS)
        .manage(db_collection)
        .manage(config)
        .mount("/", routes![hello])
        .mount("/metrics", prometheus)
        .mount(
            "/api",
            routes![
                get_flags,
                get_flag_by_id,
                create_flag,
                get_config,
                post_flags,
                post_simple,
                check_auth,
                set_config,
                start_sploit,
                get_status_statistic,
            ],
        );

    rocket_app
}
