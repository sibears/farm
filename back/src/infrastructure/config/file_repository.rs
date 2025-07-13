use rocket::serde::json::serde_json;
use std::fs;

use crate::domain::config::{Config, ConfigRepo, ConfigRepoError};

pub struct FileConfigRepo {
    file_path: String,
}

impl FileConfigRepo {
    pub fn new(file_path: &str) -> Self {
        FileConfigRepo {
            file_path: file_path.to_string(),
        }
    }
}

impl ConfigRepo for FileConfigRepo {
    fn get_config(&self) -> Result<Config, ConfigRepoError> {
        let config_str = fs::read_to_string(&self.file_path)?;
        let file_config: Config = serde_json::from_str(&config_str)?;
        Ok(file_config)
    }

    fn save_config(&mut self, config: &Config) -> Result<(), ConfigRepoError> {
        let config_str = serde_json::to_string(config)?;
        fs::write(&self.file_path, config_str)?;
        Ok(())
    }
}
