use std::ops::Deref;

use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;

use crate::{settings::Config, models::auth::BasicAuth};


#[openapi(tag = "Config", ignore = "config", ignore = "_auth")]
#[get("/get_config")]
pub fn get_config(config: &State<Config>, _auth: BasicAuth) -> Json<&Config> {
    Json(config.deref())
}
