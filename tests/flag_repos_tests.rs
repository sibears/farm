#![feature(custom_test_frameworks)]
#![feature(test)]
#![test_runner(custom_test_runner)]


extern crate test;

use sibears_farm::config::get_config;
use sibears_farm::db::connection::init_db;
use sibears_farm::db::connection::MIGRATIONS;
use diesel_migrations::MigrationHarness;
use test::{test_main_static, TestDescAndFn};

pub fn custom_test_runner(tests: &[&TestDescAndFn]) {
    let config = get_config("./config_test.json");
    let url = config.database.lock().unwrap().database_url.to_string();
    let db_pool = init_db(url);
    let mut conn = db_pool.get().unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    test_main_static(tests);

}

#[cfg(test)]
mod tests {
    use sibears_farm::config::{get_config, DbPool, DbPooled};
    use sibears_farm::db::connection::init_db;
    use sibears_farm::models::flag::{FlagStatus, NewFlag, UpdateFlag};
    use sibears_farm::repos::errors::ReposError;
    use sibears_farm::repos::flag::{FlagRepo, PostgresFlagRepo};
    use diesel::connection::Connection;


    fn setup() -> (PostgresFlagRepo, DbPool) {
        let config = get_config("./config_test.json");
        let url = config.database.lock().unwrap().database_url.to_string();
        let db_pool = init_db(url);
        (PostgresFlagRepo::new(), db_pool)
    }

    // функция выполняющая rollback после транзакции внутри теста
    fn test_transaction<F>(conn: &mut DbPooled, f: F)
    where
        F: FnOnce(&mut DbPooled) -> Result<(), ReposError>,
    {
        let _ = conn.transaction::<(), diesel::result::Error, _>(|conn| {
            let _ = f(conn).unwrap();
            Err(diesel::result::Error::RollbackTransaction)
        });
    }

    #[test]
    fn test_save() {
        let (repo, db_pool) = setup();
        let mut db_conn = db_pool.get().unwrap();
        let new_flag = NewFlag {
            flag: "USHIRTI010N54GII784SB4TQ2JHUJYZ=".to_string(),
            sploit: None,
            team: None,
        };
        test_transaction(&mut db_conn, |db_conn| {
            let result = repo.save(db_conn, &new_flag);
            assert!(result.is_ok());
            assert!(result.unwrap() >= 1);
            Ok(())
        });
    }
    
    #[test]
    fn test_save_all() {
        let (repo, db_pool) = setup();
        let mut db_conn = db_pool.get().unwrap();
        test_transaction(&mut db_conn, |db_conn| {
            let new_flags = vec![
                NewFlag {
                    flag: "IUHLAKILOAGIDJ4RFB1N5FBGBRUCDT7=".to_string(),
                    sploit: None,
                    team: None,
                },
                NewFlag {
                    flag: "GOQU9C4L2XE3OKQ9NA4C7RS3SHRQW0A=".to_string(),
                    sploit: None,
                    team: None,
                },
            ];
            let result = repo.save_all(db_conn, &new_flags);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 2);
            Ok(())
        });
    }

    #[test]
    fn test_find_by_id() {
        let (repo, db_pool) = setup();
        let mut db_conn = db_pool.get().unwrap();
        test_transaction(&mut db_conn, |db_conn| {
            let new_flag = NewFlag {
                flag: "USHIRTI010N54GII784SB4TQ2JHUJYA=".to_string(),
                sploit: None,
                team: None,
            };
            let result = repo.save(db_conn, &new_flag);
            assert!(result.is_ok());
    
            let last_id = repo.last_id(db_conn).unwrap();
            let result = repo.find_by_id(db_conn, last_id);
            assert!(result.is_ok());
            let flag = result.unwrap();
            assert_eq!(flag.flag, new_flag.flag);
            Ok(())
        });
    }

    #[test]
    fn test_find_all() {
        let (repo, db_pool) = setup();
        let mut db_conn = db_pool.get().unwrap();
        test_transaction(&mut db_conn, |db_conn| {
            let new_flag = NewFlag {
                flag: "USHIRTI010N54GII784SB4TQ2JHUJYC=".to_string(),
                sploit: None,
                team: None,
            };
            let result = repo.save(db_conn, &new_flag);
            assert!(result.is_ok());
    
            let new_flag = NewFlag {
                flag: "USHIRTI010N54GII784SB4TQ2JHUJYD=".to_string(),
                sploit: None,
                team: None,
            };
            let result = repo.save(db_conn, &new_flag);
            assert!(result.is_ok());
    
            let result = repo.find_all(db_conn);
            assert!(result.is_ok());
            let flags = result.unwrap();
            assert!(flags.len() >= 2);
            Ok(())
        });
    }
    
    #[test]
    fn test_update() {
        let (repo, db_pool) = setup();
        let mut db_conn = db_pool.get().unwrap();
        test_transaction(&mut db_conn, |db_conn| {
            let new_flag = NewFlag {
                flag: "USHIRTI010N54GII784SB4TQ2JHUJYX=".to_string(),
                sploit: None,
                team: None,
            };
            let result = repo.save(db_conn, &new_flag);
            assert!(result.is_ok());
    
            let last_id = repo.last_id(db_conn).unwrap();
            // Предполагаем, что в базе данных существует флаг с id = 1
            let update_flag = UpdateFlag {
                id: last_id,
                flag: "OZOTRO6VS7PYG3A4E0PI6CV5GVE6EAI=".to_string(),
                sploit: Some("updated".to_string()),
                team: None,
                status: FlagStatus::ACCEPTED.to_string(),
                checksystem_response: None,
            };
            let result = repo.update(db_conn, &update_flag);
            assert!(result.is_ok());
            Ok(())
        });
    }
    
    #[test]
    fn test_delete_by_id() {
        let (repo, db_pool) = setup();
        let mut db_conn = db_pool.get().unwrap();
        test_transaction(&mut db_conn, |db_conn| {
            let new_flag = NewFlag {
                flag: "USHIRTI010N54GII784SB4TQ2JHUJYY=".to_string(),
                sploit: None,
                team: None,
            };
            let result = repo.save(db_conn, &new_flag);
            assert!(result.is_ok());
    
            let last_id = repo.last_id(db_conn).unwrap();
            let result = repo.delete_by_id(db_conn, last_id);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);
            Ok(())
        });
    }
    
    #[test]
    fn test_get_limit() {
        let (repo, db_pool) = setup();
        let mut db_conn = db_pool.get().unwrap();
        test_transaction(&mut db_conn, |db_conn| {
            let new_flag = NewFlag {
                flag: "USHIRTI010N54GII784SB4TQ2JHUJYG=".to_string(),
                sploit: None,
                team: None,
            };
            let result = repo.save(db_conn, &new_flag);
            assert!(result.is_ok());
            let new_flag = NewFlag {
                flag: "USHIRTI010N54GII784SB4TQ2JHUJYS=".to_string(),
                sploit: None,
                team: None,
            };
            let result = repo.save(db_conn, &new_flag);
            assert!(result.is_ok());
    
            let result = repo.get_limit(db_conn, 2);
            assert!(result.is_ok());
            let flags = result.unwrap();
            assert!(flags.len() == 2);
            Ok(())
        });
    }
}
