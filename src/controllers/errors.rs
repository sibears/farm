use thiserror::Error;

use crate::repos::errors::ReposError;
use rocket::{
    http::Status,
    response::{self, Responder},
    Request,
};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Repository error {0:?}")]
    Repos(#[from] ReposError),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
