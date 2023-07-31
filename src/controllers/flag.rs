use rocket::{serde::json::Json, log::private::debug};

use crate::{models::flag::Flag, db::connection::DbConn, repos::flag::{FlagRepo, SqliteFlagRepo}};


#[get("/flag")]
pub fn get_flags(db: DbConn) -> Json<Vec<Flag>> {
    let flag_repo = SqliteFlagRepo::new(&db);
    let flags = flag_repo.find_all();
    Json(flags)
}