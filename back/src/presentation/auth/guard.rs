use crate::application::auth::service::AuthService;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use std::sync::Arc;

pub struct AuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_service = request.rocket().state::<Arc<AuthService>>().unwrap();
        match request.headers().get_one("X-Authorization") {
            Some(input_pass) if auth_service.authenticate(input_pass) => {
                Outcome::Success(AuthGuard)
            }
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
