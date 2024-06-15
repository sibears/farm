use crate::config::DbFlagRepo;
use crate::db::connection::DbConn;

use crate::models::auth::BasicAuth;
use crate::models::flag::FlagStatus;
use crate::models::statistic::StatusStatistic;
use crate::repos::flag::FlagRepo;

use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

use std::collections::HashMap;

use strum::IntoEnumIterator;

#[get("/statistic/status")]
pub fn get_status_statistic(
    db: DbConn,
    _auth: BasicAuth,
) -> Result<Json<StatusStatistic>, BadRequest<String>> {
    let mut flag_repo = DbFlagRepo::new(db);
    let flags = flag_repo
        .find_all()
        .map_err(|e| BadRequest(Some(e.to_string())))?;
    let mut hashmap: StatusStatistic = StatusStatistic(HashMap::new());
    for status in FlagStatus::iter() {
        hashmap.0.insert(status.to_string(), 0);
    }
    flags.iter().for_each(|flag| {
        hashmap.0.insert(
            flag.status.clone(),
            *hashmap.0.get(&flag.status).unwrap() + 1,
        );
    });
    Ok(Json(hashmap))
}
