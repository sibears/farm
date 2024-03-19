extern crate diesel;
extern crate serde_json;

use rocket::{Build, Rocket};
use rocket_okapi::openapi_get_routes_spec;
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi, openapi_get_routes, swagger_ui::*};
use rocket_prometheus::PrometheusMetrics;
use std::sync::Arc;

use crate::controllers::config::*;
use crate::controllers::flag::*;
use crate::controllers::statistic::*;
use crate::db::connection::init_db;
use crate::middleware::cors::CORS;
use crate::middleware::metrics::FLAG_COUNTER;
use crate::settings::Config;

#[openapi]
#[get("/")]
pub fn hello() -> &'static str {
    "Hello, SiBears Farm!"
}

pub fn rocket(config: Arc<Config>) -> Rocket<Build> {
    let _ = dotenv::dotenv().map_err(|err| error!("Dotenv: {:?}", err));
    let database_url = config.database.lock().unwrap().database_url.to_string();
    let prometheus = PrometheusMetrics::new();
    prometheus
        .registry()
        .register(Box::new(FLAG_COUNTER.clone()))
        .unwrap();
    let mut rocket_app = rocket::build()
        .attach(prometheus.clone())
        .attach(CORS)
        .manage(init_db(database_url))
        .manage(config)
        .mount("/", openapi_get_routes![hello])
        .mount("/metrics", prometheus)
        .mount(
            "/api",
            openapi_get_routes![
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
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        rocket_app, "/v1", openapi_settings,
        "" => openapi_get_routes_spec![openapi_settings: hello],
        "/api" => openapi_get_routes_spec![openapi_settings:
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
        ]
    }
    rocket_app
}
