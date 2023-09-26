use std::{ops::Deref, collections::HashMap};

use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;
use serde_json::Value;

use crate::{settings::Config, models::auth::BasicAuth};


#[openapi(tag = "Config", ignore = "config", ignore = "_auth")]
#[get("/get_config")]
pub fn get_config(config: &State<Config>, _auth: BasicAuth) -> Json<&Config> {
    Json(config.deref())
}

// TODO: remove this 
#[openapi(tag = "Auth", ignore = "config")]
#[post("/check_auth", data = "<passwd>")]
pub fn check_auth(config: &State<Config>, passwd: Json<HashMap<String, String>>) -> Value {
    debug!("{:?}", passwd);
    debug!("{:?}", config.auth.password.eq(passwd.get("passwd").unwrap_or(&"".to_string())));
    if config.auth.password.eq(passwd.get("passwd").unwrap_or(&"".to_string())) {
        json!("ok")
    } else {
        json!("err")
    }
}