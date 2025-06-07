use std::sync::Arc;

use crate::domain::config::{entities::Config, repository::ConfigRepo};

pub struct ConfigService<T: ConfigRepo> {
    repo: Arc<T>,
}

impl<T: ConfigRepo> ConfigService<T> {
    pub fn new(repo: Arc<T>) -> Self {
        ConfigService { repo }
    }

    pub fn get_config(&self) -> Result<Config, T::ConfigRepoError> {
        let config = self.repo.get_config()?;
        Ok(config)
    }
}
