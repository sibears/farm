use diesel::result::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReposError {
    #[error("Not find flag in database")]
    NotFindFlagError(Error),
    #[error("Failed to save flag in database")]
    FailSaveFlagError(Error),
    #[error("Failed to delete flag in database")]
    DeleteFlagError(Error),
    #[error("Failed to update flag in database")]
    UpdateFlagError(Error),
    #[error("Unknown error: {0}")]
    OtherError(String)
}