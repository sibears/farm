use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlagServiceError {
    #[error(transparent)]
    Repository(#[from] FlagRepoError),
}

#[derive(Debug, Error)]
pub enum FlagRepoError {
    #[error("Not find flag with id {0}")]
    NotFound(i32),
    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum InfrastructureError {
    #[non_exhaustive]
    Sqlx(#[from] sqlx::Error),
}

impl From<sqlx::Error> for FlagRepoError {
    fn from(error: sqlx::Error) -> Self {
        Self::Infrastructure(InfrastructureError::Sqlx(error))
    }
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
