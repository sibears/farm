use std::{ops::Deref, borrow::BorrowMut};

use regex::Regex;
use rocket::{serde::json::Json, response::status::{NotFound, NoContent, Created}, log::private::{debug, info, error}, State};
use rocket_okapi::openapi;


use crate::{models::{flag::{Flag, NewFlag, UpdateFlag}, auth::BasicAuth}, db::{connection::DbConn, schema::flags::flag}, repos::flag::{FlagRepo, SqliteFlagRepo}, settings::Config};
use crate::errors::ApiError;


#[openapi(tag = "Flag", ignore = "db", ignore = "_auth")]
#[get("/flag")]
pub fn get_flags(db: DbConn, _auth: BasicAuth) -> Json<Vec<Flag>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let flags_result = flag_repo.find_all();
    match flags_result {
        Ok(flags) => Json(flags),
        Err(e) => {
            error!("{}", e.to_string());
            Json(Vec::new())
        }
    }
}

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth")]
#[get("/flag/<id>")]
pub fn get_flag_by_id(id: i32, db: DbConn, _auth: BasicAuth) -> Result<Json<Flag>, NotFound<Json<ApiError>>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let flag_result = flag_repo.find_by_id(id);
    flag_result
        .map(Json)
        .map_err(|e| { 
            NotFound(Json(ApiError::new(
                e.to_string()
            )))
        })
}

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth", ignore = "config")]
#[post("/flag", data = "<new_flags>")]
pub fn create_flag(new_flags: Json<Vec<NewFlag>>, db: DbConn, config: &State<Config>, _auth: BasicAuth) -> Result<Created<Json<Vec<Flag>>>, Json<ApiError>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let re = Regex::new(&config.ctf.flag_format).unwrap();
    let mut matched_flags: Vec<NewFlag> = new_flags.into_inner().into_iter().filter(|x| x.match_regex(&re)).collect();
    matched_flags.sort_unstable();
    matched_flags.dedup();
    debug!("{:?}", &matched_flags);
    let mut matched_flags = flag_repo.skip_duplicate(matched_flags);
    debug!("{:?}", &matched_flags);
    let result = flag_repo.save_all(&mut matched_flags);
    result
        .map(|_| Created::new("/").body(Json(Vec::new())))
        .map_err(|e| {
            Json(ApiError::new(
                e.to_string()
            ))
        })
}

#[openapi(tag = "Flag", ignore = "db", ignore = "auth", ignore = "config")]
#[post("/post_flags", data = "<new_flags>")]
pub fn post_flags(new_flags: Json<Vec<NewFlag>>, db: DbConn, config: &State<Config>, auth: BasicAuth) -> Result<Created<Json<Vec<Flag>>>, Json<ApiError>> {
    create_flag(new_flags, db, config, auth)
}

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth")]
#[post("/post_simple", data = "<new_flags>")]
pub fn post_simple(new_flags: Json<Vec<String>>, db: DbConn, _auth: BasicAuth) -> Result<Created<Json<Vec<String>>>, Json<ApiError>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let new_flags = new_flags.to_vec();
    let mut new_flags: Vec<NewFlag> = new_flags.into_iter().map(|x| NewFlag::new(x)).collect();
    new_flags.sort_unstable();
    new_flags.dedup();
    let mut new_flags = flag_repo.skip_duplicate(new_flags);
    let result = flag_repo.save_all(&mut new_flags);
    result
        .map(|_| Created::new("/").body(Json(Vec::new())))
        .map_err(|e| {
            Json(ApiError::new(
                e.to_string()
            ))
        })
}

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth")]
#[put("/flag", data = "<updated_flag>")]
pub fn update_flag(updated_flag: Json<UpdateFlag>, db: DbConn, _auth: BasicAuth) -> Result<Json<UpdateFlag>, NotFound<Json<ApiError>>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let result = flag_repo.update(&updated_flag);
    result
        .map(|_| updated_flag)
        .map_err(|e| {
            NotFound(Json(ApiError::new(
                e.to_string()
            )))
        })
}

#[openapi(tag = "Flag", ignore = "db", ignore = "_auth")]
#[delete("/flag/<id>")]
pub fn delete_flag_by_id(id: i32, db: DbConn, _auth: BasicAuth) -> Result<NoContent, NotFound<Json<ApiError>>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let result = flag_repo.delete_by_id(id);
    result
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError::new(
                e.to_string()
            )))
        })
}