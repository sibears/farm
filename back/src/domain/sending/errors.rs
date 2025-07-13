use crate::domain::flags::FlagServiceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SendingServiceError {
    #[error(transparent)]
    FlagService(#[from] FlagServiceError),
}

