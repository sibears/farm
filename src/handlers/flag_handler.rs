use std::{thread, ops::Deref};

use chrono::{Utc, NaiveDate, NaiveDateTime, Duration};
use rocket::log::private::debug;

use crate::{settings::Config, db::connection::{init_db, DbConn}, repos::flag::{SqliteFlagRepo, FlagRepo}};

pub fn flag_handler(config: Config) {
    let db_pool = init_db(&config.database).db_conn_pool;
    loop {
        debug!("Start loop");
        let conn = DbConn { master: db_pool.get().unwrap() };
        let flag_repo = SqliteFlagRepo::new(&conn);

        let submit_start_time = Utc::now().naive_local();
        let flag_lifetime = Duration::seconds(config.ctf.flag_lifetime as i64);
        let submit_period = Duration::seconds(config.ctf.submit_period as i64);
        let skip_time = submit_start_time.checked_sub_signed(flag_lifetime).unwrap();
        
        flag_repo.skip_flags(skip_time);

        let submit_spent = Utc::now().naive_local() - submit_start_time;
        if submit_period > submit_spent {
            thread::sleep((submit_period-submit_spent).to_std().unwrap());
        }
    }
}