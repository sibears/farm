use std::sync::Arc;
use regex::Regex;
use rocket::{log::private::debug, response::status::Created, serde::json::Json, State};
use crate::config::{DbFlagRepo, DbPool};
use crate::controllers::errors::ApiError;
use crate::middleware::metrics::FLAG_COUNTER;
use crate::models::flag::FlagStatus;

use crate::{
    models::{
        auth::BasicAuth,
        flag::{Flag, NewFlag},
    },
    repos::flag::FlagRepo,
    settings::Config,
};

#[get("/flag")]
pub fn get_flags<'a>(db: &State<DbPool>, _auth: BasicAuth) -> Result<Json<Vec<Flag>>, ApiError> {
    let flag_repo = DbFlagRepo::new();
    let mut db_conn = db.inner().get().unwrap();
    let flags_result = flag_repo.find_all(&mut db_conn)?;
    Ok(Json(flags_result))
}

#[get("/flag/<id>")]
pub fn get_flag_by_id<'a>(id: i32, db: &State<DbPool>, _auth: BasicAuth) -> Result<Json<Flag>, ApiError> {
    let flag_repo = DbFlagRepo::new();
    let mut db_conn = db.inner().get().unwrap();
    let flag_result = flag_repo.find_by_id(&mut db_conn, id)?;
    Ok(Json(flag_result))
}

#[post("/flag", data = "<new_flags>")]
pub fn create_flag<'a>(
    new_flags: Json<Vec<NewFlag>>,
    db: &State<DbPool>,
    config: &State<Arc<Config>>,
    _auth: BasicAuth,
) -> Result<Created<Json<Vec<Flag>>>, ApiError> {
    let flag_repo = DbFlagRepo::new();
    let mut db_conn = db.inner().get().unwrap();
    let re = Regex::new(&config.ctf.lock().unwrap().flag_format).unwrap();
    let mut matched_flags: Vec<NewFlag> = new_flags
        .into_inner()
        .into_iter()
        .filter(|x| x.match_regex(&re))
        .collect();
    matched_flags.sort_unstable();
    matched_flags.dedup();
    let matched_flags = flag_repo.skip_duplicate(&mut db_conn, matched_flags)?;
    debug!("{:?}", &matched_flags);
    FLAG_COUNTER
        .with_label_values(&[FlagStatus::QUEUED.to_string().as_str()])
        .add(matched_flags.len() as i64);
    flag_repo.save_all(&mut db_conn, matched_flags.as_slice())?;
    Ok(Created::new("/").body(Json(Vec::new())))
}

#[post("/post_flags", data = "<new_flags>")]
pub fn post_flags<'a>(
    new_flags: Json<Vec<NewFlag>>,
    db: &State<DbPool>,
    config: &State<Arc<Config>>,
    auth: BasicAuth,
) -> Result<Created<Json<Vec<Flag>>>, ApiError> {
    create_flag(new_flags, db, config, auth)
}

#[post("/post_simple", data = "<new_flags>")]
pub fn post_simple<'a>(
    new_flags: Json<Vec<String>>,
    db: &State<DbPool>,
    _auth: BasicAuth,
) -> Result<Created<Json<Vec<String>>>, ApiError> {
    let flag_repo = DbFlagRepo::new();
    let mut db_conn = db.inner().get().unwrap();
    let new_flags = new_flags.to_vec();
    let mut new_flags: Vec<NewFlag> = new_flags.into_iter().map(|x| NewFlag::new(x)).collect();
    new_flags.sort_unstable();
    new_flags.dedup();
    let new_flags = flag_repo.skip_duplicate(&mut db_conn, new_flags)?;
    FLAG_COUNTER
        .with_label_values(&[FlagStatus::QUEUED.to_string().as_str()])
        .add(new_flags.len() as i64);
    flag_repo.save_all(&mut db_conn, new_flags.as_slice())?;
    Ok(Created::new("/").body(Json(Vec::new())))
}
