use std::{thread, ops::Deref};
use diesel::update;
use futures::executor;

use chrono::{Utc, NaiveDate, NaiveDateTime, Duration};
use rocket::log::private::debug;

use crate::{settings::Config, db::connection::{init_sqlite_db, DbConn}, repos::flag::{SqliteFlagRepo, FlagRepo}, models::flag::Flag};

pub fn flag_handler(config: Config) {
    let db_pool = init_sqlite_db(&config.database).db_conn_pool;
    loop {
        let conn = DbConn { master: db_pool.get().unwrap() };
        let flag_repo = SqliteFlagRepo::new(&conn);

        let submit_start_time = Utc::now().naive_local();
        let flag_lifetime = Duration::seconds(config.ctf.flag_lifetime as i64);
        let submit_period = Duration::seconds(config.ctf.submit_period as i64);
        let skip_time = submit_start_time.checked_sub_signed(flag_lifetime).unwrap();
        
        flag_repo.skip_flags(skip_time);
        
        let queue_flags = flag_repo.get_limit(config.ctf.submit_flag_limit as i64);
        debug!("Queue flags: {:?}", queue_flags);
        if queue_flags.len() > 0 {
            let updated_flags = submit_flags(queue_flags, &config);
            if updated_flags.len() > 0 {
                flag_repo.update_status(updated_flags);
            }
        }

        let submit_spent = Utc::now().naive_local() - submit_start_time;
        if submit_period > submit_spent {
            thread::sleep((submit_period-submit_spent).to_std().unwrap());
        }
    }
}

fn submit_flags(queue_flags: Vec<Flag>, config: &Config) -> Vec<Flag> {
    let handler = config.ctf.protocol.get_protocol_handler();
    handler.send_flags(queue_flags, &config.ctf.protocol)
}