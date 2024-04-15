use std::sync::Arc;

use regex::Regex;
use rocket::response::status::BadRequest;
use rocket::{
    log::private::{debug, error},
    response::status::{Created, NotFound},
    serde::json::Json,
    State,
};
use rocket_okapi::openapi;

use crate::config::DbFlagRepo;
use crate::middleware::metrics::FLAG_COUNTER;
use crate::models::flag::FlagStatus;
use crate::{
    db::connection::DbConn,
    models::{
        auth::BasicAuth,
        flag::{Flag, NewFlag},
    },
    repos::flag::FlagRepo,
    settings::Config,
};

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth")]
#[get("/flag")]
pub fn get_flags(db: DbConn, _auth: BasicAuth) -> Result<Json<Vec<Flag>>, BadRequest<String>> {
    let flag_repo = DbFlagRepo::new(db);
    let flags_result = flag_repo.find_all();
    flags_result.map(Json).map_err(|e| {
        error!("{}", e.to_string());
        BadRequest(Some(e.to_string()))
    })
}

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth")]
#[get("/flag/<id>")]
pub fn get_flag_by_id(
    id: i32,
    db: DbConn,
    _auth: BasicAuth,
) -> Result<Json<Flag>, NotFound<String>> {
    let flag_repo = DbFlagRepo::new(db);
    let flag_result = flag_repo.find_by_id(id);
    flag_result.map(Json).map_err(|e| NotFound(e.to_string()))
}

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth", ignore = "config")]
#[post("/flag", data = "<new_flags>")]
pub fn create_flag(
    new_flags: Json<Vec<NewFlag>>,
    db: DbConn,
    config: &State<Arc<Config>>,
    _auth: BasicAuth,
) -> Result<Created<Json<Vec<Flag>>>, BadRequest<String>> {
    let flag_repo = DbFlagRepo::new(db);
    let re = Regex::new(&config.ctf.lock().unwrap().flag_format).unwrap();
    let mut matched_flags: Vec<NewFlag> = new_flags
        .into_inner()
        .into_iter()
        .filter(|x| x.match_regex(&re))
        .collect();
    matched_flags.sort_unstable();
    matched_flags.dedup();
    let matched_flags = flag_repo.skip_duplicate(matched_flags);
    debug!("{:?}", &matched_flags);
    FLAG_COUNTER
        .with_label_values(&[FlagStatus::QUEUED.to_string().as_str()])
        .add(matched_flags.len() as i64);
    let result = flag_repo.save_all(matched_flags.as_slice());
    result
        .map(|_| Created::new("/").body(Json(Vec::new())))
        .map_err(|e| BadRequest(Some(e.to_string())))
}

#[openapi(tag = "Flag", ignore = "db", ignore = "auth", ignore = "config")]
#[post("/post_flags", data = "<new_flags>")]
pub fn post_flags(
    new_flags: Json<Vec<NewFlag>>,
    db: DbConn,
    config: &State<Arc<Config>>,
    auth: BasicAuth,
) -> Result<Created<Json<Vec<Flag>>>, BadRequest<String>> {
    create_flag(new_flags, db, config, auth)
}

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth")]
#[post("/post_simple", data = "<new_flags>")]
pub fn post_simple(
    new_flags: Json<Vec<String>>,
    db: DbConn,
    _auth: BasicAuth,
) -> Result<Created<Json<Vec<String>>>, BadRequest<String>> {
    let flag_repo = DbFlagRepo::new(db);
    let new_flags = new_flags.to_vec();
    let mut new_flags: Vec<NewFlag> = new_flags.into_iter().map(|x| NewFlag::new(x)).collect();
    new_flags.sort_unstable();
    new_flags.dedup();
    let new_flags = flag_repo.skip_duplicate(new_flags);
    FLAG_COUNTER
        .with_label_values(&[FlagStatus::QUEUED.to_string().as_str()])
        .add(new_flags.len() as i64);
    let result = flag_repo.save_all(new_flags.as_slice());
    result
        .map(|_| Created::new("/").body(Json(Vec::new())))
        .map_err(|e| BadRequest(Some(e.to_string())))
}
