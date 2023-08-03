use rocket::{serde::json::Json, log::private::debug, response::status::{NotFound, NoContent}};

use crate::{models::flag::Flag, db::connection::DbConn, repos::flag::{FlagRepo, SqliteFlagRepo}};
use crate::errors::ApiError;


#[get("/flags")]
pub fn get_flags(db: DbConn) -> Json<Vec<Flag>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let flags = flag_repo.find_all();
    Json(flags)
}


#[get("/flag/<id>")]
pub fn get_flag_by_id(id: i32, db: DbConn) -> Result<Json<Flag>, NotFound<Json<ApiError>>> {
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

#[delete("/flag/<id>")]
pub fn delete_flag_by_id(id: i32, db: DbConn) -> Result<NoContent, NotFound<Json<ApiError>>> {
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