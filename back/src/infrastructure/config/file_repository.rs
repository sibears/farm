use std::fs;

use crate::domain::config::{entities::Config, repository::ConfigRepo};

pub struct FileConfigRepo {
    file_path: String,
}

impl FileConfigRepo {
    pub fn new(file_path: &str) -> Self {
        FileConfigRepo {
            file_path: file_path.to_string(),
        }
    }

    fn read_config(&self) -> Result<Config, std::io::Error> {
        let config_str = fs::read_to_string(&self.file_path)?;
        let file_config: Config = serde_json::from_str(&config_str)?;
        Ok(file_config)
    }

    fn write_config(&self, config: &Config) -> Result<(), std::io::Error> {
        let config_str = serde_json::to_string(config)?;
        fs::write(&self.file_path, config_str)?;
        Ok(())
    }
}

impl ConfigRepo for FileConfigRepo {
    type ConfigRepoError = std::io::Error;

    fn get_config(&self) -> Result<Config, Self::ConfigRepoError> {
        let file_config = self.read_config()?;
        Ok(file_config)
    }

    fn save_config(&mut self, config: &Config) -> Result<(), Self::ConfigRepoError> {
        let file_config = config.clone();
        self.write_config(&file_config)
    }
}
