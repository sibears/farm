use crate::domain::flags::entities::{Flag, NewFlag};
use crate::domain::flags::repository::FlagRepo;
use std::sync::{Arc, Mutex};

pub struct FlagService {
    repo: Arc<Mutex<dyn FlagRepo<FlagRepoError = diesel::result::Error>>>,
}

impl FlagService {
    pub fn new(repo: Arc<Mutex<dyn FlagRepo<FlagRepoError = diesel::result::Error>>>) -> Self {
        FlagService { repo }
    }

    pub fn get_flag(&self, id: i32) -> Result<Flag, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        repo.get(id)
    }

    pub fn get_all_flags(&self) -> Result<Vec<Flag>, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        repo.get_all()
    }

    pub fn save_flag(&self, new_flag: &NewFlag) -> Result<usize, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        repo.save(new_flag)
    }

    pub fn delete_flag(&self, id: i32) -> Result<usize, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        repo.delete(id)
    }

    pub fn update_flag(&self, flag: &Flag) -> Result<usize, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        repo.update(flag)
    }
}