use super::*;
use std::sync::{Arc, Mutex};

// Mock репозиторий
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
                name: String::from("Test Flag"),
                active: true,
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
                    name: String::from("Flag 1"),
                    active: true,
                },
                Flag {
                    id: 2,
                    name: String::from("Flag 2"),
                    active: false,
                },
            ])
        }
    }

    fn save(&self, _new_flag: &NewFlag) -> Result<usize, Self::FlagRepoError> {
        if self.should_fail {
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::Unknown, Box::new(diesel::result::DatabaseErrorInformation::empty())))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_flag() {
        let mock_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let service = FlagService::new(mock_repo);

        let flag = service.get_flag(1).expect("Expected a flag");

        assert_eq!(flag.id, 1);
        assert_eq!(flag.name, "Test Flag");
        assert!(flag.active);
    }

    #[test]
    fn test_get_all_flags() {
        let mock_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let service = FlagService::new(mock_repo);

        let flags = service.get_all_flags().expect("Expected a list of flags");

        assert_eq!(flags.len(), 2);
        assert_eq!(flags[0].name, "Flag 1");
        assert_eq!(flags[1].name, "Flag 2");
    }

    #[test]
    fn test_save_flag() {
        let mock_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let service = FlagService::new(mock_repo);

        let new_flag = NewFlag {
            name: String::from("New Test Flag"),
            active: true,
        };

        let result = service.save_flag(&new_flag).expect("Expected successful save");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_delete_flag() {
        let mock_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let service = FlagService::new(mock_repo);

        let result = service.delete_flag(1).expect("Expected successful deletion");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_update_flag() {
        let mock_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: false }));
        let service = FlagService::new(mock_repo);

        let updated_flag = Flag {
            id: 1,
            name: String::from("Updated Flag"),
            active: false,
        };

        let result = service.update_flag(&updated_flag).expect("Expected successful update");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_get_flag_with_error() {
        let mock_repo = Arc::new(Mutex::new(MockFlagRepo { should_fail: true }));
        let service = FlagService::new(mock_repo);

        let result = service.get_flag(1);
        assert!(result.is_err());
    }
}
