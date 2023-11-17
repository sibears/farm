use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::QueryDsl;
use diesel::result::Error;
use diesel::result::Error::NotFound;
use r2d2_diesel::ConnectionManager;
use r2d2::PooledConnection;
use r2d2::Pool;
use diesel::Connection;
use rocket::log::private::debug;
use rocket::log::private::error;
use crate::config::DbFlagRepo;
use crate::db::schema::flags::status;
use crate::db::schema::flags::time;
use crate::errors::ApiError;
use crate::models::flag::Flag;
use crate::db::schema::flags;
use crate::diesel::RunQueryDsl;
use crate::models::flag::FlagStatus;
use crate::models::flag::NewFlag;
use crate::models::flag::SavedFlag;
use crate::models::flag::UpdateFlag;
use crate::repos::flag::flags::dsl::flags as flags_dsl;
use crate::db::connection::*;
use crate::diesel::ExpressionMethods;


pub trait FlagRepo {
    fn new(conn: DbConn) -> DbFlagRepo;
    fn find_all(&self) -> Result<Vec<Flag>, Error>;
    fn find_by_id(&self, id: i32) -> Result<Flag, Error>;
    fn save(&self, flag: &mut NewFlag) -> Result<(), Error>;
    fn save_all(&self, flag: &mut Vec<NewFlag>) -> Result<(), Error>;
    fn delete_by_id(&self, id: i32) -> Result<(), Error>;
    fn update(&self, flag: &UpdateFlag) -> Result<(), Error>;
    fn skip_flags(&self, skip_time: NaiveDateTime);
    fn get_limit(&self, limit: i64) -> Vec<Flag>;
    fn update_status(&self, flags: Vec<Flag>);
    fn skip_duplicate(&self, flags: Vec<NewFlag>) -> Vec<NewFlag>;
}

impl FlagRepo {
    fn new(conn: DbConn) -> DbFlagRepo {
        DbFlagRepo { db_conn: conn }
    }
}

pub struct PostgresFlagRepo {
    db_conn: DbConn,
}

impl PostgresFlagRepo {
    pub fn new(conn: DbConn) -> PostgresFlagRepo {
        PostgresFlagRepo { db_conn: conn }
    }
}

impl FlagRepo for PostgresFlagRepo {
    fn find_all(&self) -> Result<Vec<Flag>, Error> {
        let conn = self.db_conn.master.deref();
        let all_flags = flags::table.load::<Flag>(conn);
        all_flags
    }

    fn find_by_id(&self, id: i32) -> Result<Flag, Error> {
        let conn = self.db_conn.master.deref();
        let flag = flags_dsl.filter(flags::dsl::id.eq(id)).first(conn);
        flag
    }

    fn save(&self, flag: &mut NewFlag) -> Result<(), Error> {
        let conn = self.db_conn.master.deref();
        let flag = SavedFlag::from(flag.deref());
        let result = diesel::insert_into(flags_dsl)
            .values(flag)
            .execute(conn)
            .unwrap();

        match result {
            1 => Ok(()),
            0 => Err(Error::NotFound),
            _ => Err(Error::__Nonexhaustive)
        }
    }

    fn save_all(&self, flags: &mut Vec<NewFlag>) -> Result<(), Error> {
        let conn = self.db_conn.master.deref();
        let flags: Vec<SavedFlag> = flags.into_iter().map(|item| SavedFlag::from(item.deref())).collect();
        let result = diesel::insert_into(flags_dsl)
            .values(flags.deref())
            .execute(conn)
            .unwrap();

        
        if result == flags.len() { 
            Ok(()) 
        } else { 
            Err(Error::NotFound) 
        }
    }

    fn delete_by_id(&self, id: i32) -> Result<(), Error> {
        let conn = self.db_conn.master.deref();
    
        let result = diesel::delete(flags_dsl.filter(flags::dsl::id.eq(id)))
            .execute(conn)
            .unwrap();
        
        match result {
            1 => Ok(()),
            0 => Err(Error::NotFound),
            _ => Err(Error::__Nonexhaustive)
        }
    }

    fn update(&self, flag: &UpdateFlag) -> Result<(), Error> {
        let conn = self.db_conn.master.deref();

        let result = diesel::update(flags_dsl.find(flag.id))
            .set(flag)
            .execute(conn)
            .unwrap();
        match result {
            1 => Ok(()),
            0 => Err(Error::NotFound),
            _ => Err(Error::__Nonexhaustive)
        }
    }

    fn skip_flags(&self, skip_time: NaiveDateTime) {
        let conn = self.db_conn.master.deref();
        
        let res = diesel::update(flags_dsl.filter(time.lt(skip_time)).filter(status.eq(FlagStatus::QUEUED.to_string())))
            .set(status.eq(FlagStatus::SKIPPED.to_string()))
            .execute(conn)
            .unwrap();
        debug!("Skipped: {} flags", res);
    }

    fn get_limit(&self, limit: i64) -> Vec<Flag> {
        let conn = self.db_conn.master.deref();
        
        let res = flags_dsl.filter(status.eq(FlagStatus::QUEUED.to_string()))
            .limit(limit)
            .load::<Flag>(conn)
            .unwrap();
        res
    }

    fn update_status(&self, flags: Vec<Flag>) {
        let conn = self.db_conn.master.deref();

        for flag in flags {
            diesel::update(flags_dsl.find(flag.id))
                .set(flag)
                .execute(conn)
                .unwrap();
        }
    }

    fn skip_duplicate(&self, mut flags: Vec<NewFlag>) -> Vec<NewFlag> {
        let conn = self.db_conn.master.deref();

        let res = flags_dsl.select(flags::dsl::flag)
            .load::<String>(conn)
            .unwrap();
        flags.retain(|x| !res.contains(&x.flag.to_string()));
        flags
    }
}
