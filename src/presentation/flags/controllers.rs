use chrono::offset;
use rocket::{serde::json::Json, State};
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::{
    application::flags::service::FlagService,
    domain::flags::entities::{Flag, FlagsQuery, NewFlag},
};

#[derive(Serialize)]
pub struct FlagsResponse {
    pub total: i64,
    pub flags: Vec<Flag>,
}

#[utoipa::path(
    get,
    path = "/api/flags",
    responses(
        (status = 200, description = "List all flags", body = Vec<Flag>)
    )
)]

#[get("/flags?<flags_query..>")]
pub fn get_flags(flag_service: &State<Arc<FlagService>>, flags_query: Option<FlagsQuery>) -> Json<FlagsResponse> {
    let flags = match flags_query {
        Some(query) => {
            flag_service.get_flags_per_page(query.limit, query.offset).unwrap()
        }
        None => {
            flag_service.get_all_flags().unwrap()
        }
    };
    
    let total = flag_service.get_total_flags_count().unwrap();
    
    Json(FlagsResponse {
        total,
        flags,
    })
}

#[get("/flags/count")]
pub fn get_flags_count(flag_service: &State<Arc<FlagService>>) -> Json<i64> {
    let count = flag_service.get_total_flags_count().unwrap();
    Json(count)
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
pub fn post_flag(flag_service: &State<Arc<FlagService>>, new_flag: Json<NewFlag>) -> Json<usize> {
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
    flag_service: &State<Arc<FlagService>>,
    new_flags: Json<Vec<NewFlag>>,
) -> Json<usize> {
    info!("post_flags: {:?}", &new_flags);
    let res = flag_service.save_all_flags(&new_flags).unwrap();
    Json(res)
}
