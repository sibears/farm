use std::sync::{Arc, Mutex};

use crate::domain::config::{entities::Config, repository::ConfigRepo};

pub struct ConfigService {
    repo: Arc<Mutex<dyn ConfigRepo<ConfigRepoError = std::io::Error>>>
}

impl ConfigService {
    pub fn new(repo: Arc<Mutex<dyn ConfigRepo<ConfigRepoError = std::io::Error>>>) -> Self {
        ConfigService { repo }
    }
    
    pub fn get_config(&self) -> Result<Config, std::io::Error> {
        let repo = self.repo.lock().unwrap();
        let config = repo.get_config()?;
        Ok(config)
    }
}