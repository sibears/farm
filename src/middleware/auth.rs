use std::sync::Arc;

use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request::Outcome, Request, State};
use crate::errors::BasicAuthError;
use crate::models::auth::BasicAuth;
use crate::settings::Config;


#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = BasicAuthError;
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = request.guard::<&State<Arc<Config>>>().await.unwrap();

        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Forward(()),
            1 => if keys[0] == config.auth.lock().unwrap().password {
                Outcome::Success(BasicAuth)
            } else {
                Outcome::Failure((Status::BadRequest, BasicAuthError::Invalid))
            }
            _ => Outcome::Failure((Status::BadRequest, BasicAuthError::BadCount))
        }
    }
}