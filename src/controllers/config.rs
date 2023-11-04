use std::{ops::{Deref, DerefMut}, collections::HashMap, sync::{Mutex, Arc}, borrow::BorrowMut, cell::RefCell};

use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;
use serde_json::Value;

use crate::{settings::{Config, RawConfig}, models::auth::BasicAuth};


#[openapi(tag = "Config", ignore = "config", ignore = "_auth")]
#[get("/get_config")]
pub fn get_config(config: &State<Config>, _auth: BasicAuth) -> Json<&Config> {
    Json(config.deref())
}

#[openapi(tag = "Config", ignore = "config", ignore = "_auth")]
#[post("/set_config", data = "<new_config>")]
pub fn set_config(config: &State<Config>, _auth: BasicAuth, new_config: Json<RawConfig>) -> Json<&Config> {
    let mut lock_auth = config.auth.lock().unwrap();
    let mut lock_ctf = config.ctf.lock().unwrap();
    let mut lock_database = config.database.lock().unwrap();

    lock_auth.copy(&new_config.auth);
    lock_ctf.copy(&new_config.ctf);
    lock_database.copy(&new_config.database);
    Json(config.deref())
}

// TODO: remove this 
#[openapi(tag = "Auth", ignore = "config")]
#[post("/check_auth", data = "<passwd>")]
pub fn check_auth(config: &State<Config>, passwd: Json<HashMap<String, String>>) -> Value {
    let lock_auth = config.auth.lock().unwrap();
    debug!("{:?} compare {:?}", passwd, lock_auth.password.eq(passwd.get("passwd").unwrap_or(&"".to_string())));
    if lock_auth.password.eq(passwd.get("passwd").unwrap_or(&"".to_string())) {
        json!("ok")
    } else {
        json!("err")
    }
}