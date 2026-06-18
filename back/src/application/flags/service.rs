use regex::Regex;

use crate::application::config::ConfigService;
use crate::domain::config::ConfigRepo;
use crate::domain::flags::{Flag, FlagRepo, FlagServiceError, FlagStatus, NewFlag, SaveFlag};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct FlagService<T: FlagRepo, C: ConfigRepo> {
    repo: Arc<RwLock<T>>,
    config_service: Arc<ConfigService<C>>,
}

impl<T: FlagRepo, C: ConfigRepo> FlagService<T, C> {
    pub fn new(repo: Arc<RwLock<T>>, config_service: Arc<ConfigService<C>>) -> Self {
        FlagService {
            repo,
            config_service,
        }
    }

    pub async fn get_flags(&self, id: &[i32]) -> Result<Vec<Flag>, FlagServiceError> {
        let repo = self.repo.read().await;
        let flag = repo.get(id).await?;
        Ok(flag)
    }

    pub async fn get_all_flags(&self) -> Result<Arc<[Flag]>, FlagServiceError> {
        let repo = self.repo.read().await;
        let flags = repo.get_all().await?;
        Ok(flags)
    }

    pub async fn get_flags_per_page_from_start(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, FlagServiceError> {
        let repo = self.repo.read().await;
        let flags = repo.get_limit_with_offset_from_start(limit, offset).await?;
        Ok(flags)
    }

    pub async fn get_flags_per_page_from_end(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, FlagServiceError> {
        let repo = self.repo.read().await;
        let flags = repo.get_limit_with_offset_from_end(limit, offset).await?;
        Ok(flags)
    }

    pub async fn next_send_flags(&self) -> Result<Vec<Flag>, FlagServiceError> {
        let repo = self.repo.read().await;
        let config = self.config_service.get_config().unwrap();
        let flags = repo
            .get_limit_by_status(FlagStatus::QUEUED, config.ctf.submit_flag_limit)
            .await?;
        Ok(flags)
    }

    pub async fn get_waiting_flags(&self) -> Result<Vec<Flag>, FlagServiceError> {
        let repo = self.repo.read().await;
        let flags = repo.get_by_status(FlagStatus::WAITING).await?;
        Ok(flags)
    }

    pub async fn save_flags(&self, new_flags: &[NewFlag]) -> Result<usize, FlagServiceError> {
        let mut repo = self.repo.write().await;
        let flag_regex = self.config_service.get_config().unwrap().ctf.flag_format;
        let re = Regex::new(&flag_regex).unwrap();
        let save_flags: Vec<SaveFlag> = new_flags
            .iter()
            .filter(|next_flag| next_flag.match_regex(&re))
            .map(SaveFlag::from)
            .collect();
        let result = repo.save(&save_flags).await?;
        Ok(result)
    }

    pub async fn delete_flag(&self, id: i32) -> Result<usize, FlagServiceError> {
        let mut repo = self.repo.write().await;
        let result = repo.delete(&[id]).await?;
        Ok(result)
    }

    pub async fn update_flag(&self, flag: &Flag) -> Result<usize, FlagServiceError> {
        let mut repo = self.repo.write().await;
        let result = repo.update(std::slice::from_ref(flag)).await?;
        Ok(result)
    }

    pub async fn update_all_flags(&self, flags: &[Flag]) -> Result<usize, FlagServiceError> {
        let mut repo = self.repo.write().await;
        let result = repo.update(flags).await?;
        Ok(result)
    }

    pub async fn get_total_flags(&self) -> Result<i64, FlagServiceError> {
        let repo = self.repo.read().await;
        let result = repo.get_total_flags().await?;
        Ok(result)
    }

    pub async fn get_total_flags_by_status(
        &self,
        flag_status: FlagStatus,
    ) -> Result<i64, FlagServiceError> {
        let repo = self.repo.read().await;
        let result = repo.get_total_flags_by_status(flag_status).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::config::Config;
    use crate::infrastructure::config::InMemoryConfigRepository;
    use crate::infrastructure::flags::InMemoryFlagRepository;

    fn service() -> FlagService<InMemoryFlagRepository, InMemoryConfigRepository> {
        let repo = Arc::new(RwLock::new(InMemoryFlagRepository::new()));
        let mut config = Config::test_config();
        config.ctf.flag_format = "^DUPLICATE_FLAG=$".to_string();
        let config_repo = Arc::new(InMemoryConfigRepository::new(&config));
        let config_service = Arc::new(ConfigService::new(config_repo));

        FlagService::new(repo, config_service)
    }

    #[tokio::test]
    async fn save_flags_ignores_duplicate_flags() {
        let service = service();
        let duplicate_flag = NewFlag {
            flag: "DUPLICATE_FLAG=".to_string(),
            sploit: Some("sploit".to_string()),
            team: Some("team".to_string()),
        };

        let saved_count = service
            .save_flags(&[duplicate_flag.clone(), duplicate_flag.clone()])
            .await
            .expect("Duplicate flags should be ignored");
        let repeated_saved_count = service
            .save_flags(std::slice::from_ref(&duplicate_flag))
            .await
            .expect("Previously saved duplicate flag should be ignored");
        let flags = service.get_all_flags().await.unwrap();

        assert_eq!(saved_count, 1);
        assert_eq!(repeated_saved_count, 0);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0].flag, duplicate_flag.flag);
    }
}
