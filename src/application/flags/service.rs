use regex::Regex;

use crate::application::config::service::ConfigService;
use crate::domain::flags::entities::{Flag, FlagStatus, NewFlag, SaveFlag};
use crate::domain::flags::repository::FlagRepo;
use std::sync::{Arc, Mutex};

pub struct FlagService {
    repo: Arc<Mutex<dyn FlagRepo<FlagRepoError = diesel::result::Error>>>,
    config_service: Arc<ConfigService>
}

// TODO: Заменить diesel::result::Error на свои ошибки 
impl FlagService {
    pub fn new(repo: Arc<Mutex<dyn FlagRepo<FlagRepoError = diesel::result::Error>>>, config_service: Arc<ConfigService>) -> Self {
        FlagService { repo, config_service }
    }

    pub fn get_flag(&self, id: i32) -> Result<Flag, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        repo.get(id)
    }

    pub fn get_all_flags(&self) -> Result<Vec<Flag>, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        repo.get_all()
    }

    pub fn next_send_flags(&self) -> Result<Vec<Flag>, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        let config = self.config_service.get_config().unwrap();
        repo.get_limit_by_status(FlagStatus::QUEUED, config.ctf.submit_flag_limit)
    }

    pub fn save_flag(&self, new_flag: &NewFlag) -> Result<usize, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        let flag_regex = self.config_service.get_config().unwrap().ctf.flag_format;
        let re = Regex::new(&flag_regex).unwrap();
        if !new_flag.match_regex(&re) {
            info!("skipped flag: {:?}", new_flag);
            return Ok(0);
        }
        let save_flag = SaveFlag::from(new_flag);
        repo.save(&save_flag)
    }

    pub fn save_all_flags(&self, new_flags: &[NewFlag]) -> Result<usize, diesel::result::Error> {
        let repo = self.repo.lock().unwrap();
        let flag_regex = self.config_service.get_config().unwrap().ctf.flag_format;
        let re = Regex::new(&flag_regex).unwrap();
        let save_flags: Vec<SaveFlag> = new_flags.iter().filter(|next_flag| next_flag.match_regex(&re)).map(SaveFlag::from).collect();
        repo.save_all(&save_flags)
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