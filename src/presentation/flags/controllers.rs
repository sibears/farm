use rocket::{serde::json::Json, State};
use std::sync::Arc;

use crate::{
    application::flags::service::FlagService,
    domain::flags::entities::{Flag, NewFlag},
};

#[get("/flags")]
pub fn get_flags(flag_service: &State<Arc<FlagService>>) -> Json<Vec<Flag>> {
    let res = flag_service.get_all_flags().unwrap();
    Json(res)
}

#[post("/flag", data = "<new_flag>")]
pub fn post_flag(flag_service: &State<Arc<FlagService>>, new_flag: Json<NewFlag>) -> Json<usize> {
    info!("post_flag: {:?}", &new_flag);
    let res = flag_service.save_flag(&new_flag.into_inner()).unwrap();
    Json(res)
}

#[post("/flags", data = "<new_flags>")]
pub fn post_flags(
    flag_service: &State<Arc<FlagService>>,
    new_flags: Json<Vec<NewFlag>>,
) -> Json<usize> {
    info!("post_flags: {:?}", &new_flags);
    let res = flag_service.save_all_flags(&new_flags).unwrap();
    Json(res)
}
