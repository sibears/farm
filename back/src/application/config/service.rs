use std::sync::Arc;

use crate::domain::config::{Config, ConfigRepo, ConfigServiceError};

pub struct ConfigService<T: ConfigRepo> {
    repo: Arc<T>,
}

impl<T: ConfigRepo> ConfigService<T> {
    pub fn new(repo: Arc<T>) -> Self {
        ConfigService { repo }
    }

    pub fn get_config(&self) -> Result<Config, ConfigServiceError> {
        let config = self.repo.get_config()?;
        Ok(config)
    }
}
