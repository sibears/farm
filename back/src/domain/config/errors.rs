use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigServiceError {
    #[error(transparent)]
    Repository(#[from] ConfigRepoError),
}

#[derive(Debug, Error)]
pub enum ConfigRepoError {
    #[error(transparent)]
    Serialization(#[from] rocket::serde::json::serde_json::Error),
    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),
}

impl From<std::io::Error> for ConfigRepoError {
    fn from(err: std::io::Error) -> Self {
        ConfigRepoError::Infrastructure(InfrastructureError::FileIO(err))
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum InfrastructureError {
    FileIO(#[from] std::io::Error),
}
