use crate::domain::flags::{Flag, FlagRepo, FlagRepoError, FlagStatus, SaveFlag};
use async_trait::async_trait;
use std::sync::Arc;

pub struct InMemoryFlagRepository {
    flags: Vec<Flag>,
}

impl InMemoryFlagRepository {
    pub fn new() -> Self {
        InMemoryFlagRepository { flags: Vec::new() }
    }
}

impl Default for InMemoryFlagRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FlagRepo for InMemoryFlagRepository {
    async fn get(&self, id: i32) -> Result<Flag, FlagRepoError> {
        self.flags
            .iter()
            .find(|flag| flag.id == id)
            .cloned()
            .ok_or(FlagRepoError::NotFound(id))
    }

    async fn get_all(&self) -> Result<Arc<[Flag]>, FlagRepoError> {
        Ok(self.flags.clone().into())
    }
    async fn get_all_by_status(&self, flag_status: FlagStatus) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn save(&mut self, flag: &SaveFlag) -> Result<usize, FlagRepoError> {
        self.flags.push(Flag {
            id: self.flags.len() as i32 + 1,
            flag: flag.flag.clone(),
            sploit: flag.sploit.clone(),
            team: flag.team.clone(),
            created_time: flag.created_time,
            start_waiting_time: None,
            status: flag.status,
            checksystem_response: flag.checksystem_response.clone(),
        });
        Ok(1)
    }

    async fn save_all(&mut self, flags: &[SaveFlag]) -> Result<usize, FlagRepoError> {
        todo!()
    }

    async fn delete(&mut self, id: i32) -> Result<usize, FlagRepoError> {
        todo!()
    }

    async fn delete_all(&mut self, flags: &[Flag]) -> Result<usize, FlagRepoError> {
        todo!()
    }

    async fn update(&mut self, flag: &Flag) -> Result<usize, FlagRepoError> {
        todo!()
    }

    async fn update_all(&mut self, flags: &[Flag]) -> Result<usize, FlagRepoError> {
        todo!()
    }

    async fn get_limit(&self, limit: u32) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_limit_with_offset_from_start(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_limit_with_offset_from_end(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_last_id(&self) -> Result<i32, FlagRepoError> {
        self.flags
            .last()
            .map(|flag| flag.id)
            .ok_or(FlagRepoError::NotFound(0))
    }

    async fn get_limit_by_status(
        &self,
        flag_status: FlagStatus,
        limit: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_all_by_id(&self, ids: &[i32]) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_total_flags(&self) -> Result<i64, FlagRepoError> {
        todo!()
    }

    async fn get_total_flags_by_status(
        &self,
        flag_status: FlagStatus,
    ) -> Result<i64, FlagRepoError> {
        todo!()
    }
}
