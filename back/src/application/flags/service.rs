use regex::Regex;

use crate::application::config::ConfigService;
use crate::domain::config::ConfigRepo;
use crate::domain::flags::{
    Flag, FlagRepo, FlagRepoError, FlagServiceError, FlagStatus, NewFlag, SaveFlag,
};
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

    pub async fn get_flag(&self, id: i32) -> Result<Flag, FlagServiceError> {
        let repo = self.repo.read().await;
        let flag = repo.get(id).await?;
        Ok(flag)
    }

    pub async fn get_all_flags(&self) -> Result<Arc<[Flag]>, FlagRepoError> {
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
        let flags = repo.get_all_by_status(FlagStatus::WAITING).await?;
        Ok(flags)
    }

    pub async fn save_flag(&self, new_flag: &NewFlag) -> Result<usize, FlagServiceError> {
        let flag_regex = self.config_service.get_config().unwrap().ctf.flag_format;
        let re = Regex::new(&flag_regex).unwrap();
        if !new_flag.match_regex(&re) {
            info!("skipped flag: {:?}", new_flag);
            return Ok(0);
        }
        let save_flag = SaveFlag::from(new_flag);
        let mut repo = self.repo.write().await;
        let result = repo.save(&save_flag).await?;
        Ok(result)
    }

    pub async fn save_all_flags(&self, new_flags: &[NewFlag]) -> Result<usize, FlagServiceError> {
        let mut repo = self.repo.write().await;
        let flag_regex = self.config_service.get_config().unwrap().ctf.flag_format;
        let re = Regex::new(&flag_regex).unwrap();
        let save_flags: Vec<SaveFlag> = new_flags
            .iter()
            .filter(|next_flag| next_flag.match_regex(&re))
            .map(SaveFlag::from)
            .collect();
        let result = repo.save_all(&save_flags).await?;
        Ok(result)
    }

    pub async fn get_full_flags(&self, flags: &[Flag]) -> Result<Vec<Flag>, FlagServiceError> {
        let repo = self.repo.read().await;
        let ids = flags.iter().map(|flag| flag.id).collect::<Vec<i32>>();
        let flags = repo.get_all_by_id(&ids).await?;
        Ok(flags)
    }

    pub async fn delete_flag(&self, id: i32) -> Result<usize, FlagServiceError> {
        let mut repo = self.repo.write().await;
        let result = repo.delete(id).await?;
        Ok(result)
    }

    pub async fn update_flag(&self, flag: &Flag) -> Result<usize, FlagServiceError> {
        let mut repo = self.repo.write().await;
        let result = repo.update(flag).await?;
        Ok(result)
    }

    pub async fn update_all_flags(&self, flags: &[Flag]) -> Result<usize, FlagServiceError> {
        let mut repo = self.repo.write().await;
        let result = repo.update_all(flags).await?;
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
