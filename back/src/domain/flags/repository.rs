use super::entities::{Flag, FlagStatus, SaveFlag};
use super::errors::FlagRepoError;
use std::sync::Arc;

#[async_trait]
pub trait FlagRepo: Send + Sync {
    async fn get(&self, id: i32) -> Result<Flag, FlagRepoError>;
    async fn get_all(&self) -> Result<Arc<[Flag]>, FlagRepoError>;
    async fn get_all_by_status(&self, flag_status: FlagStatus) -> Result<Vec<Flag>, FlagRepoError>;
    async fn save(&mut self, flag: &SaveFlag) -> Result<usize, FlagRepoError>;
    async fn save_all(&mut self, flag: &[SaveFlag]) -> Result<usize, FlagRepoError>;
    async fn delete(&mut self, id: i32) -> Result<usize, FlagRepoError>;
    async fn delete_all(&mut self, flags: &[Flag]) -> Result<usize, FlagRepoError>;
    async fn update(&mut self, flag: &Flag) -> Result<usize, FlagRepoError>;
    async fn update_all(&mut self, flags: &[Flag]) -> Result<usize, FlagRepoError>;
    async fn get_limit(&self, limit: u32) -> Result<Vec<Flag>, FlagRepoError>;
    async fn get_limit_with_offset_from_start(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, FlagRepoError>;
    async fn get_limit_with_offset_from_end(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, FlagRepoError>;
    async fn get_last_id(&self) -> Result<i32, FlagRepoError>;
    async fn get_limit_by_status(
        &self,
        flag_status: FlagStatus,
        limit: u32,
    ) -> Result<Vec<Flag>, FlagRepoError>;
    async fn get_all_by_id(&self, ids: &[i32]) -> Result<Vec<Flag>, FlagRepoError>;
    async fn get_total_flags(&self) -> Result<i64, FlagRepoError>;
    async fn get_total_flags_by_status(
        &self,
        flag_status: FlagStatus,
    ) -> Result<i64, FlagRepoError>;
}
