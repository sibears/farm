use regex::Regex;

use crate::application::config::service::ConfigService;
use crate::domain::config::repository::ConfigRepo;
use crate::domain::flags::entities::{Flag, FlagStatus, NewFlag, SaveFlag};
use crate::domain::flags::repository::FlagRepo;
use std::sync::Arc;

pub struct FlagService<T: FlagRepo, C: ConfigRepo> {
    repo: Arc<T>,
    config_service: Arc<ConfigService<C>>,
}

impl<T: FlagRepo, C: ConfigRepo> FlagService<T, C> {
    pub fn new(repo: Arc<T>, config_service: Arc<ConfigService<C>>) -> Self {
        FlagService {
            repo,
            config_service,
        }
    }

    pub async fn get_flag(&self, id: i32) -> Result<Flag, T::FlagRepoError> {
        self.repo.get(id).await
    }

    pub async fn get_all_flags(&self) -> Result<Vec<Flag>, T::FlagRepoError> {
        self.repo.get_all().await
    }

    pub async fn get_flags_per_page_from_start(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, T::FlagRepoError> {
        self.repo
            .get_limit_with_offset_from_start(limit, offset)
            .await
    }

    pub async fn get_flags_per_page_from_end(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, T::FlagRepoError> {
        self.repo
            .get_limit_with_offset_from_end(limit, offset)
            .await
    }

    pub async fn next_send_flags(&self) -> Result<Vec<Flag>, T::FlagRepoError> {
        let config = self.config_service.get_config().unwrap();
        self.repo
            .get_limit_by_status(FlagStatus::QUEUED, config.ctf.submit_flag_limit)
            .await
    }

    pub async fn get_waiting_flags(&self) -> Result<Vec<Flag>, T::FlagRepoError> {
        self.repo.get_all_by_status(FlagStatus::WAITING).await
    }

    pub async fn save_flag(&self, new_flag: &NewFlag) -> Result<usize, T::FlagRepoError> {
        let flag_regex = self.config_service.get_config().unwrap().ctf.flag_format;
        let re = Regex::new(&flag_regex).unwrap();
        if !new_flag.match_regex(&re) {
            info!("skipped flag: {:?}", new_flag);
            return Ok(0);
        }
        let save_flag = SaveFlag::from(new_flag);
        self.repo.save(&save_flag).await
    }

    pub async fn save_all_flags(&self, new_flags: &[NewFlag]) -> Result<usize, T::FlagRepoError> {
        let flag_regex = self.config_service.get_config().unwrap().ctf.flag_format;
        let re = Regex::new(&flag_regex).unwrap();
        let save_flags: Vec<SaveFlag> = new_flags
            .iter()
            .filter(|next_flag| next_flag.match_regex(&re))
            .map(SaveFlag::from)
            .collect();
        self.repo.save_all(&save_flags).await
    }

    pub async fn get_full_flags(&self, flags: &[Flag]) -> Result<Vec<Flag>, T::FlagRepoError> {
        let ids = flags.iter().map(|flag| flag.id).collect::<Vec<i32>>();
        self.repo.get_all_by_id(&ids).await
    }

    pub async fn delete_flag(&self, id: i32) -> Result<usize, T::FlagRepoError> {
        self.repo.delete(id).await
    }

    pub async fn update_flag(&self, flag: &Flag) -> Result<usize, T::FlagRepoError> {
        self.repo.update(flag).await
    }

    pub async fn update_all_flags(&self, flags: &[Flag]) -> Result<usize, T::FlagRepoError> {
        self.repo.update_all(flags).await
    }

    pub async fn get_total_flags(&self) -> Result<i64, T::FlagRepoError> {
        self.repo.get_total_flags().await
    }

    pub async fn get_total_flags_by_status(
        &self,
        flag_status: FlagStatus,
    ) -> Result<i64, T::FlagRepoError> {
        self.repo.get_total_flags_by_status(flag_status).await
    }
}
