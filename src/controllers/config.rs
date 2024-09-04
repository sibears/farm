use futures::lock;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::status::BadRequest;
use rocket::{serde::json::Json, State};
use serde_json::Value;
use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{
    models::auth::BasicAuth,
    settings::{Config, RawConfig},
};

#[get("/get_config")]
pub fn get_config(config: &State<Arc<Config>>, _auth: BasicAuth) -> Json<&Config> {
    Json(config.deref())
}

#[post("/set_config", data = "<new_config>")]
pub fn set_config(
    config: &State<Arc<Config>>,
    _auth: BasicAuth,
    new_config: Json<Config>,
) -> Json<&Config> {

    Json(config.deref())
}

#[post("/check_auth", data = "<passwd>")]
pub fn check_auth(config: &State<Arc<Config>>, passwd: Json<HashMap<String, String>>) -> Value {
    let lock_auth = config.auth.lock().unwrap();
    debug!(
        "{:?} compare {:?}",
        passwd,
        lock_auth
            .password
            .eq(passwd.get("passwd").unwrap_or(&"".to_string()))
    );
    if lock_auth
        .password
        .eq(passwd.get("passwd").unwrap_or(&"".to_string()))
    {
        json!("ok")
    } else {
        json!("err")
    }
}

#[get("/start_sploit.py")]
pub async fn start_sploit() -> Result<(ContentType, NamedFile), BadRequest<String>> {
    NamedFile::open("./start_sploit.py")
        .await
        .map_err(|err| BadRequest(Some(err.to_string())))
        .map(|file| (ContentType::new("application", "x-python-code"), file))
}
