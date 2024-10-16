use rocket::{serde::json::Json, State};

use crate::{application::flags::service::FlagService, domain::flags::entities::{Flag, NewFlag}};


#[get("/")]
pub fn get_flags(flag_service: &State<FlagService>) -> Json<Vec<Flag>> {
    let res = flag_service.get_all_flags().unwrap();
    Json(res)
}

#[post("/", data = "<flag>")]
pub fn post_flag(flag_service: &State<FlagService>, flag: Json<NewFlag>) -> Json<usize> {
    let res = flag_service.save_flag(&flag.into_inner()).unwrap();
    Json(res)
}