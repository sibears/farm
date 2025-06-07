use crate::domain::flags::entities::{FlagStatus, SaveFlag};
use crate::domain::flags::{entities::Flag, repository::FlagRepo};

// Mock репозиторий
#[allow(dead_code)]
struct MockFlagRepo {
    should_fail: bool,
}

impl FlagRepo for MockFlagRepo {
    type FlagRepoError = diesel::result::Error;

    fn get(&self, id: i32) -> Result<Flag, Self::FlagRepoError> {
        if self.should_fail {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(Flag {
                id,
                flag: String::from("Test Flag"),
                sploit: Some(String::from("test.py")),
                team: Some(String::from("Test Team")),
                created_time: chrono::Utc::now().naive_utc(),
                start_waiting_time: None,
                status: FlagStatus::QUEUED,
                checksystem_response: None,
            })
        }
    }

    fn get_all(&self) -> Result<Vec<Flag>, Self::FlagRepoError> {
        if self.should_fail {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(vec![
                Flag {
                    id: 1,
                    flag: String::from("Flag 1"),
                    sploit: Some(String::from("test1.py")),
                    team: Some(String::from("Team 1")),
                    created_time: chrono::Utc::now().naive_utc(),
                    start_waiting_time: None,
                    status: FlagStatus::QUEUED,
                    checksystem_response: None,
                },
                Flag {
                    id: 2,
                    flag: String::from("Flag 2"),
                    sploit: Some(String::from("test2.py")),
                    team: Some(String::from("Team 2")),
                    created_time: chrono::Utc::now().naive_utc(),
                    start_waiting_time: None,
                    status: FlagStatus::QUEUED,
                    checksystem_response: None,
                },
            ])
        }
    }

    fn save(&self, _new_flag: &SaveFlag) -> Result<usize, Self::FlagRepoError> {
        if self.should_fail {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(1)
        }
    }

    fn delete(&self, _id: i32) -> Result<usize, Self::FlagRepoError> {
        if self.should_fail {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(1)
        }
    }

    fn update(&self, _flag: &Flag) -> Result<usize, Self::FlagRepoError> {
        if self.should_fail {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(1)
        }
    }

    fn get_all_by_status(
        &self,
        _flag_status: crate::domain::flags::entities::FlagStatus,
    ) -> Result<Vec<crate::domain::flags::entities::Flag>, Self::FlagRepoError> {
        todo!()
    }

    fn save_all(
        &self,
        _flag: &[crate::domain::flags::entities::SaveFlag],
    ) -> Result<usize, Self::FlagRepoError> {
        todo!()
    }

    fn delete_all(
        &self,
        _flags: &[crate::domain::flags::entities::Flag],
    ) -> Result<usize, Self::FlagRepoError> {
        todo!()
    }

    fn update_all(
        &self,
        _flags: &[crate::domain::flags::entities::Flag],
    ) -> Result<usize, Self::FlagRepoError> {
        todo!()
    }

    fn get_limit(
        &self,
        _limit: u32,
    ) -> Result<Vec<crate::domain::flags::entities::Flag>, Self::FlagRepoError> {
        todo!()
    }

    fn get_last_id(&self) -> Result<i32, Self::FlagRepoError> {
        todo!()
    }

    fn get_limit_by_status(
        &self,
        _flag_status: crate::domain::flags::entities::FlagStatus,
        _limit: u32,
    ) -> Result<Vec<crate::domain::flags::entities::Flag>, Self::FlagRepoError> {
        todo!()
    }

    fn get_all_by_id(
        &self,
        _ids: &[i32],
    ) -> Result<Vec<crate::domain::flags::entities::Flag>, Self::FlagRepoError> {
        todo!()
    }

    fn get_limit_with_offset_from_start(
        &self,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Flag>, Self::FlagRepoError> {
        todo!()
    }

    fn get_limit_with_offset_from_end(
        &self,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Flag>, Self::FlagRepoError> {
        todo!()
    }

    fn get_total_flags(&self) -> Result<i64, Self::FlagRepoError> {
        todo!()
    }

    fn get_total_flags_by_status(
        &self,
        _flag_status: FlagStatus,
    ) -> Result<i64, Self::FlagRepoError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use crate::{
        application::{
            config::{service::ConfigService, test::MockConfigRepo},
            flags::service::FlagService,
        },
        domain::{
            config::entities::{AuthConfig, Config, CtfConfig, DatabaseConfig, ProtocolConfig},
            flags::entities::NewFlag,
        },
    };

    use super::*;

    fn create_test_config() -> Config {
        Config {
            database: DatabaseConfig {
                database_url: "test_url".to_string(),
            },
            auth: AuthConfig {
                password: "test_password".to_string(),
            },
            ctf: CtfConfig {
                protocol: ProtocolConfig {
                    protocol: "test_protocol".to_string(),
                    team_token: "test_token".to_string(),
                    checksys_host: "test_host".to_string(),
                    checksys_port: 80,
                },
                flag_format: r"\w{31}=".to_string(),
                flag_lifetime: 300,
                submit_period: 5,
                submit_flag_limit: 100,
                teams: HashMap::new(),
                waiting_period: 10,
            },
        }
    }

    #[test]
    fn test_get_flag() {
        let mock_flag_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let mock_config_repo =
            Arc::new(Mutex::new(MockConfigRepo::new(false, create_test_config())));
        let config_service = Arc::new(ConfigService::new(mock_config_repo));
        let service = FlagService::new(mock_flag_repo, config_service);

        let flag = service.get_flag(1).expect("Expected a flag");

        assert_eq!(flag.id, 1);
        assert_eq!(flag.flag, "Test Flag");
        assert_eq!(flag.status, FlagStatus::QUEUED);
    }

    #[test]
    fn test_get_all_flags() {
        let mock_flag_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let mock_config_repo =
            Arc::new(Mutex::new(MockConfigRepo::new(false, create_test_config())));
        let config_service = Arc::new(ConfigService::new(mock_config_repo));
        let service = FlagService::new(mock_flag_repo, config_service);

        let flags = service.get_all_flags().expect("Expected a list of flags");

        assert_eq!(flags.len(), 2);
        assert_eq!(flags[0].flag, "Flag 1");
        assert_eq!(flags[1].flag, "Flag 2");
    }

    #[test]
    fn test_save_flag() {
        let mock_flag_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let mock_config_repo =
            Arc::new(Mutex::new(MockConfigRepo::new(false, create_test_config())));
        let config_service = Arc::new(ConfigService::new(mock_config_repo));
        let service = FlagService::new(mock_flag_repo, config_service);

        let new_flag = NewFlag {
            flag: String::from("DJME8B126RB8HWMHES7QRO4P7F0PB2I="),
            sploit: Some(String::from("test.py")),
            team: Some(String::from("Test Team")),
        };

        let result = service
            .save_flag(&new_flag)
            .expect("Expected successful save");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_delete_flag() {
        let mock_flag_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let mock_config_repo =
            Arc::new(Mutex::new(MockConfigRepo::new(false, create_test_config())));
        let config_service = Arc::new(ConfigService::new(mock_config_repo));
        let service = FlagService::new(mock_flag_repo, config_service);

        let result = service
            .delete_flag(1)
            .expect("Expected successful deletion");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_update_flag() {
        let mock_flag_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let mock_config_repo =
            Arc::new(Mutex::new(MockConfigRepo::new(false, create_test_config())));
        let config_service = Arc::new(ConfigService::new(mock_config_repo));
        let service = FlagService::new(mock_flag_repo, config_service);

        let updated_flag = Flag {
            id: 1,
            flag: String::from("Updated Flag"),
            sploit: Some(String::from("test.py")),
            team: Some(String::from("Test Team")),
            created_time: chrono::Utc::now().naive_utc(),
            start_waiting_time: None,
            status: FlagStatus::QUEUED,
            checksystem_response: None,
        };

        let result = service
            .update_flag(&updated_flag)
            .expect("Expected successful update");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_get_flag_with_error() {
        let mock_flag_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: true }));
        let mock_config_repo =
            Arc::new(Mutex::new(MockConfigRepo::new(false, create_test_config())));
        let config_service = Arc::new(ConfigService::new(mock_config_repo));
        let service = FlagService::new(mock_flag_repo, config_service);

        let result = service.get_flag(1);
        assert!(result.is_err());
    }
}
