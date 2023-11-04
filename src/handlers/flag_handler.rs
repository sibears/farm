use std::{thread, ops::Deref};
use std::sync::MutexGuard;
use diesel::update;
use futures::executor;

use chrono::{Utc, NaiveDate, NaiveDateTime, Duration};
use rocket::log::private::debug;

use crate::{settings::Config, db::connection::{init_db, DbConn}, repos::flag::{PostgresFlagRepo, FlagRepo}, models::flag::Flag};
use crate::config::DbFlagRepo;
use crate::settings::CtfConfig;

pub fn flag_handler(config: Config) {
    let db_pool = init_db(std::env::var("DATABASE_URL").unwrap()).db_conn_pool;
    // TODO: Оптимизировать lock мутекса
    loop {
        let conn = DbConn { master: db_pool.get().unwrap() };
        let flag_repo = DbFlagRepo::new(conn);
        let lock_ctf_config = config.ctf.lock().unwrap();

        let submit_start_time = Utc::now().naive_local();
        let flag_lifetime = Duration::seconds(lock_ctf_config.flag_lifetime as i64);
        let submit_period = Duration::seconds(lock_ctf_config.submit_period as i64);
        let skip_time = submit_start_time.checked_sub_signed(flag_lifetime).unwrap();
        
        flag_repo.skip_flags(skip_time);
        
        let queue_flags = flag_repo.get_limit(lock_ctf_config.submit_flag_limit as i64);
        info!("Queue flags: {:?}", queue_flags);
        if queue_flags.len() > 0 {
            let updated_flags = submit_flags(queue_flags, &lock_ctf_config);
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

fn submit_flags(queue_flags: Vec<Flag>, config: &MutexGuard<CtfConfig>) -> Vec<Flag> {
    let handler = config.protocol.get_protocol_handler();
    handler.send_flags(queue_flags, &config.protocol)
}