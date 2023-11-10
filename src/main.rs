#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;


use std::ops::Deref;
use rocket::{Rocket, Build};
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*, mount_endpoints_and_merged_docs};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::openapi_get_routes_spec;
use sibears_farm::handlers::flag_handler::flag_handler;
use std::sync::{Mutex, Arc};
use std::thread;
use rocket::http::Method;
use sibears_farm::controllers::metrics::okapi_add_operation_for_get_metrics_;


use sibears_farm::db::connection::init_db;
use sibears_farm::config::get_config;
use sibears_farm::controllers::flag::*;
use sibears_farm::controllers::config::*;
use sibears_farm::controllers::metrics::get_metrics;
use sibears_farm::middleware::cors::CORS;

#[openapi]
#[get("/")]
fn hello() -> &'static str {
    "Hello, SiBears Farm!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    let _ = dotenv::dotenv().map_err(|err| error!("Dotenv: {:?}", err));
    let config = Arc::new(get_config());
    let config_handler = config.clone();
    thread::spawn(move || {
        flag_handler(config_handler);
    });
    let database_url = config.database.lock().unwrap().database_url.to_string();
    let mut rocket_app = rocket::build()
        .attach(CORS)
        .manage(init_db(database_url))
        .manage(config)
        .mount("/", openapi_get_routes![hello])
        .mount("/api", openapi_get_routes![
            get_flags, 
            get_flag_by_id, 
            create_flag, 
            update_flag, 
            delete_flag_by_id,
            get_config,
            post_flags,
            post_simple,
            check_auth,
            set_config,
            get_metrics,
            start_sploit
        ])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
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
            delete_flag_by_id, 
            create_flag, 
            update_flag,
            get_config,
            post_flags,
            post_simple,
            check_auth,
            set_config,
            get_metrics,
            start_sploit
        ]
    };
    rocket_app
}