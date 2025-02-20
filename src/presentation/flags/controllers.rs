use chrono::offset;
use rocket::{serde::json::Json, State};
use strum::IntoEnumIterator;
use std::sync::Arc;

use crate::{
    application::flags::service::FlagService,
    domain::flags::entities::{Flag, FlagStatus, FlagsQuery, NewFlag}, presentation::auth::guard::AuthGuard,
};

#[utoipa::path(
    get,
    path = "/api/flags",
    responses(
        (status = 200, description = "List all flags", body = Vec<Flag>)
    )
)]
#[get("/flags")]
pub fn get_flags(_auth: AuthGuard, flag_service: &State<Arc<FlagService>>) -> Json<Vec<Flag>> {
    let res = flag_service.get_all_flags().unwrap();
    Json(res)
}

#[get("/flags?<flags_query..>")]
pub fn get_flags_per_page(_auth: AuthGuard, flag_service: &State<Arc<FlagService>>, flags_query: FlagsQuery) -> Json<Vec<Flag>> {
    let res = flag_service.get_flags_per_page_from_end(flags_query.limit, flags_query.offset).unwrap();
    Json(res)
}

#[get("/flags/total")]
pub fn get_total_flags(_auth: AuthGuard, flag_service: &State<Arc<FlagService>>) -> Json<i64> {
    let res = flag_service.get_total_flags().unwrap();
    Json(res)
}

#[utoipa::path(
    post,
    path = "/api/flag",
    request_body = NewFlag,
    responses(
        (status = 200, description = "Flag added successfully", body = usize)
    )
)]
#[post("/flag", data = "<new_flag>")]
pub fn post_flag(_auth: AuthGuard, flag_service: &State<Arc<FlagService>>, new_flag: Json<NewFlag>) -> Json<usize> {
    info!("post_flag: {:?}", &new_flag);
    let res = flag_service.save_flag(&new_flag.into_inner()).unwrap();
    Json(res)
}

#[utoipa::path(
    post,
    path = "/api/flags",
    request_body = Vec<NewFlag>,
    responses(
        (status = 200, description = "Flags added successfully", body = usize)
    )
)]
#[post("/flags", data = "<new_flags>")]
pub fn post_flags(
    _auth: AuthGuard,
    flag_service: &State<Arc<FlagService>>,
    new_flags: Json<Vec<NewFlag>>,
) -> Json<usize> {
    info!("post_flags: {:?}", &new_flags);
    let res = flag_service.save_all_flags(&new_flags).unwrap();
    Json(res)
}

#[utoipa::path(
    get,
    path = "/api/flags/stats",
    responses(
        (status = 200, description = "Statistics of flags by status", body = Vec<(String, i64)>)
    )
)]
#[get("/flags/stats")]
pub fn get_stats_flags_by_status(
    _auth: AuthGuard,
    flag_service: &State<Arc<FlagService>>,
) -> Json<Vec<(String, i64)>> {
    let mut stats = Vec::new();

    // Iterate through all possible flag statuses.
    for status in FlagStatus::iter() {
        // Query the total flags for the given status. If query fails, use 0.
        let count = flag_service.get_total_flags_by_status(status.clone()).unwrap_or(0);
        stats.push((status.to_string(), count));
    }

    Json(stats)
}