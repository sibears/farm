use std::sync::Arc;

use crate::errors::BasicAuthError;
use crate::models::auth::BasicAuth;
use crate::settings::Config;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request::Outcome, Request, State};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = BasicAuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = request.guard::<&State<Arc<Config>>>().await.unwrap();

        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => {
                error!("Unauthorized: add Authorization header with password");
                Outcome::Forward(())
            }
            1 => {
                if keys[0] == config.auth.lock().unwrap().password {
                    Outcome::Success(BasicAuth)
                } else {
                    error!("Auth error");
                    Outcome::Failure((Status::BadRequest, BasicAuthError::Invalid))
                }
            }
            _ => {
                error!("Auth error");
                Outcome::Failure((Status::BadRequest, BasicAuthError::BadCount))
            }
        }
    }
}
