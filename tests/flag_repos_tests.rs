use diesel::r2d2::{ConnectionManager, Pool};

use sibears_farm::config::DieselConnection;
use sibears_farm::db::connection::DbConn;

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use sibears_farm::config::get_test_config;
    use sibears_farm::db::connection::init_db;
    use sibears_farm::models::flag::{FlagStatus, NewFlag, UpdateFlag};
    use sibears_farm::repos::flag::{FlagRepo, PostgresFlagRepo};

    use super::*;

    static mut LAST_ID: i32 = 0;

    fn setup() -> PostgresFlagRepo {
        let config = get_test_config();
        let url = config.database.lock().unwrap().database_url.to_string();
        let conn = init_db(url);
        PostgresFlagRepo::new(conn)
    }

    #[test]
    fn test_save() {
        let mut repo = setup();
        let new_flag = NewFlag {
            flag: "FLAG{test_save}".to_string(),
            sploit: None,
            team: None,
        };
        let result = repo.save(&new_flag);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        unsafe { LAST_ID = repo.last_id().unwrap(); }
    }

    #[test]
    fn test_save_all() {
        let mut repo = setup();
        let new_flags = vec![
            NewFlag {
                flag: "FLAG{test_save_all_1}".to_string(),
                sploit: None,
                team: None,
            },
            NewFlag {
                flag: "FLAG{test_save_all_2}".to_string(),
                sploit: None,
                team: None,
            },
        ];
        let result = repo.save_all(&new_flags);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_find_all() {
        let mut repo = setup();
        let result = repo.find_all();
        assert!(result.is_ok());
        let flags = result.unwrap();
        assert!(flags.len() >= 0);
    }

    #[test]
    fn test_find_by_id() {
        let mut repo = setup();
        // Предполагаем, что в базе данных существует флаг с id = 1
        let result = repo.find_by_id(1);
        assert!(result.is_ok());
        let flag = result.unwrap();
        assert_eq!(flag.id, 1);
    }

    #[test]
    fn test_delete_by_id() {
        let mut repo = setup();
        // Предполагаем, что в базе данных существует флаг с id = 1
        let result = repo.delete_by_id(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_update() {
        let mut repo = setup();
        // Предполагаем, что в базе данных существует флаг с id = 1
        let update_flag = UpdateFlag {
            id: 1,
            flag: "FLAG{test_update}".to_string(),
            sploit: None,
            team: None,
            status: FlagStatus::ACCEPTED.to_string(),
            checksystem_response: None,
        };
        let result = repo.update(&update_flag);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
    
    #[test]
    fn test_skip_flags() {
        let mut repo = setup();
        let skip_time = NaiveDateTime::from_timestamp(chrono::Utc::now().timestamp(), 0);
        let result = repo.skip_flags(skip_time);
        assert!(result.is_ok());
        assert!(result.unwrap() >= 0);
    }

    #[test]
    fn test_get_limit() {
        let mut repo = setup();
        let result = repo.get_limit(5);
        assert!(result.is_ok());
        let flags = result.unwrap();
        assert!(flags.len() <= 5);
    }

    // #[test]
    // fn test_update_status() {
    //     let repo = setup();
    //     // Предполагаем, что в базе данных существует флаг с id = 1
    //     let flags = vec![
    //         Flag {
    //             id: 1,
    //             flag: "FLAG{test_update_status}".to_string(),
    //             status: FlagStatus::SUBMITTED,
    //             created_at: NaiveDateTime::from_timestamp(chrono::Utc::now().timestamp(), 0),
    //         },
    //     ];
    //     let result = repo.update_status(&flags);
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), 1);
    // }
    
    // #[test]
    // fn skip_duplicate_with_duplicates() {
    //     let flag = NewFlag {
    //         flag: "FLAG{test_skip_duplicate}".to_string(),
    //         status: "QUEUED".to_string(),
    //         time: Utc::now().naive_utc(),
    //     };
    //
    //     let repo = setup();
    //
    //     // Добавляем флаг в базу данных дважды (создаем дубликаты)
    //     let _ = repo.save(&flag);
    //     let _ = repo.save(&flag);
    //
    //     // Получаем все флаги из базы данных
    //     let all_flags_before = repo.find_all().unwrap();
    //
    //     // Пропускаем дубликаты
    //     let unique_flags = repo.skip_duplicate(vec![flag]).unwrap();
    //
    //     // Проверяем, что количество уникальных флагов равно 1 (только один флаг добавлен)
    //     assert_eq!(unique_flags.len(), 1);
    //
    //     // Получаем все флаги из базы данных после пропуска дубликатов
    //     let all_flags_after = repo.find_all().unwrap();
    //
    //     // Проверяем, что количество флагов в базе данных осталось неизменным после пропуска дубликатов
    //     assert_eq!(all_flags_before.len(), all_flags_after.len());
    // }
}