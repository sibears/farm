use sqlx::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlagRepoError {
    #[error("Not find flag in database")]
    NotFindFlagError(Error),
    #[error("Failed to save flag in database")]
    FailSaveFlagError(Error),
    #[error("Failed to delete flag in database")]
    DeleteFlagError(Error),
    #[error("Failed to update flag in database")]
    UpdateFlagError(Error),
    #[error("Unknown error: {0}")]
    OtherError(String),
}

#[derive(Debug, thiserror::Error)]
#[error("CustomError: {msg}, {status}")]
pub struct FlagStatusError {
    msg: String,
    status: u16,
}

impl FlagStatusError {
    pub fn not_found(msg: String) -> Self {
        Self { msg, status: 404 }
    }
}
