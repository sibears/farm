use crate::application::sending::service::SendingService;
use crate::domain::flags::entities::Flag;
use rocket::{serde::json::Json, State};

#[get("/sending_flags")]
pub fn get_flags_for_senders(sending_service: &State<SendingService>) -> Json<Vec<Flag>> {
    sending_service.update_waiting_flags().unwrap();
    let res = sending_service.get_flags_for_senders().unwrap();
    Json(res)
}

#[post("/force_update_waiting_flags")]
pub fn force_update_waiting_flags(sending_service: &State<SendingService>) {
    sending_service.update_waiting_flags().unwrap();
}
