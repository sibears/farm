use super::entities::Config;

pub trait ConfigRepo: Send + Sync {
    type ConfigRepoError;

    fn get_config(&self) -> Result<Config, Self::ConfigRepoError>;
    fn save_config(&self, config: &Config) -> Result<(), Self::ConfigRepoError>;
}