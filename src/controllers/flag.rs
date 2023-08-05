use rocket::{serde::json::Json, response::status::{NotFound, NoContent}, log::private::debug};
use rocket_okapi::{openapi};


use crate::{models::flag::Flag, db::connection::DbConn, repos::flag::{FlagRepo, SqliteFlagRepo}};
use crate::errors::ApiError;


#[openapi(tag = "Flag", ignore = "db")]
#[get("/flag")]
pub fn get_flags(db: DbConn) -> Json<Vec<Flag>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let flags_result = flag_repo.find_all();
    match flags_result {
        Ok(flags) => Json(flags),
        Err(e) => {
            debug!("{}", e.to_string());
            Json(Vec::new())
        }
    }
}

#[openapi(tag = "Flag", ignore = "db")]
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

#[openapi(tag = "Flag", ignore = "db")]
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