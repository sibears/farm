use std::ops::Deref;

use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;

use crate::settings::Config;


#[openapi(tag = "Config", ignore = "config")]
#[get("/get_config")]
pub fn get_config(config: &State<Config>) -> Json<&Config> {
    Json(config.deref())
}
