use crate::application::sending::service::SendingService;
use crate::domain::flags::entities::Flag;
use rocket::{serde::json::Json, State};

/// Get flags for senders
#[utoipa::path(
    get,
    path = "/api/get_sending_flags",
    responses(
        (status = 200, description = "Get flags for sending", body = Vec<Flag>)
    )
)]
#[get("/get_sending_flags")]
pub fn get_flags_for_senders(sending_service: &State<SendingService>) -> Json<Vec<Flag>> {
    sending_service.update_waiting_flags().unwrap();
    let res = sending_service.get_flags_for_senders().unwrap();
    debug!("Sending flags: {:?}", res);
    Json(res)
}

/// Force update waiting flags
#[utoipa::path(
    post,
    path = "/api/force_update_waiting_flags",
    responses(
        (status = 200, description = "Update waiting flags")
    )
)]
#[post("/force_update_waiting_flags")]
pub fn force_update_waiting_flags(sending_service: &State<SendingService>) {
    debug!("Force updating waiting flags");
    sending_service.update_waiting_flags().unwrap();
}

/// Update flags from sending
#[utoipa::path(
    post,
    path = "/api/update_flags_from_sending",
    request_body = Vec<Flag>,
    responses(
        (status = 200, description = "Update flags from sending")
    )
)]
#[post("/update_flags_from_sending", data = "<flags>")]
pub fn update_flags_from_sending(sending_service: &State<SendingService>, flags: Json<Vec<Flag>>) {
    debug!("Updating flags from sending: {:?}", flags);
    sending_service.update_flags_from_sending(&flags).unwrap();
}
