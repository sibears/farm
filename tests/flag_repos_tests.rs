#[cfg(test)]
mod tests {
    use sibears_farm::config::get_config;
    use sibears_farm::db::connection::DbCollection;
    use sibears_farm::models::flag::{FlagStatus, NewFlag, UpdateFlag};
    use sibears_farm::repos::flag::{FlagRepo, PostgresFlagRepo};

    fn setup() -> PostgresFlagRepo {
        let config = get_config("./config.json");
        let url = config.database.lock().unwrap().database_url.to_string();
        let conn = DbCollection::init_db(url);
        conn.run_migrations();
        PostgresFlagRepo::new(conn.get_conn())
    }

    #[test]
    fn test_save() {
        let mut repo = setup();
        let new_flag = NewFlag {
            flag: "USHIRTI010N54GII784SB4TQ2JHUJYC=".to_string(),
            sploit: None,
            team: None,
        };
        let result = repo.save(&new_flag);
        assert!(result.is_ok());
        assert!(result.unwrap() >= 1);
    }

    #[test]
    fn test_save_all() {
        let mut repo = setup();
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
        let result = repo.save_all(&new_flags);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_find_all() {
        let mut repo = setup();

        let new_flag = NewFlag {
            flag: "USHIRTI010N54GII784SB4TQ2JHUJYC=".to_string(),
            sploit: None,
            team: None,
        };
        let result = repo.save(&new_flag);
        assert!(result.is_ok());

        let result = repo.find_all();
        assert!(result.is_ok());
        let flags = result.unwrap();
        assert!(flags.len() >= 1);
    }

    #[test]
    fn test_find_by_id() {
        let mut repo = setup();

        let new_flag = NewFlag {
            flag: "USHIRTI010N54GII784SB4TQ2JHUJYC=".to_string(),
            sploit: None,
            team: None,
        };
        let result = repo.save(&new_flag);
        assert!(result.is_ok());

        let last_id = repo.last_id().unwrap();
        let result = repo.find_by_id(last_id);
        assert!(result.is_ok());
        let flag = result.unwrap();
        assert_eq!(flag.id, last_id);
    }

    #[test]
    fn test_update() {
        let mut repo = setup();

        let new_flag = NewFlag {
            flag: "USHIRTI010N54GII784SB4TQ2JHUJYC=".to_string(),
            sploit: None,
            team: None,
        };
        let result = repo.save(&new_flag);
        assert!(result.is_ok());

        let last_id = repo.last_id().unwrap();
        // Предполагаем, что в базе данных существует флаг с id = 1
        let update_flag = UpdateFlag {
            id: last_id,
            flag: "OZOTRO6VS7PYG3A4E0PI6CV5GVE6EAI=".to_string(),
            sploit: Some("updated".to_string()),
            team: None,
            status: FlagStatus::ACCEPTED.to_string(),
            checksystem_response: None,
        };
        let result = repo.update(&update_flag);
        assert!(result.is_ok());
        let result = repo.find_by_id(last_id);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(update_flag.flag, result.flag);
        assert_eq!(update_flag.sploit, result.sploit);
    }

    #[test]
    fn test_delete_by_id() {
        let mut repo = setup();

        let new_flag = NewFlag {
            flag: "USHIRTI010N54GII784SB4TQ2JHUJYC=".to_string(),
            sploit: None,
            team: None,
        };
        let result = repo.save(&new_flag);
        assert!(result.is_ok());

        let last_id = repo.last_id().unwrap();
        let result = repo.delete_by_id(last_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_get_limit() {
        let mut repo = setup();

        let new_flag = NewFlag {
            flag: "USHIRTI010N54GII784SB4TQ2JHUJYC=".to_string(),
            sploit: None,
            team: None,
        };
        let result = repo.save(&new_flag);
        assert!(result.is_ok());
        let new_flag = NewFlag {
            flag: "USHIRTI010N54GII784SB4TQ2JHUJYD=".to_string(),
            sploit: None,
            team: None,
        };
        let result = repo.save(&new_flag);
        assert!(result.is_ok());

        let result = repo.get_limit(2);
        assert!(result.is_ok());
        let flags = result.unwrap();
        assert!(flags.len() == 2);
    }
}