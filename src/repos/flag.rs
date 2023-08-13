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
    fn find_all(&self) -> Result<Vec<Flag>, Error>;
    fn find_by_id(&self, id: i32) -> Result<Flag, Error>;
    fn save(&self, flag: &mut NewFlag) -> Result<(), Error>;
    fn save_all(&self, flag: &mut Vec<NewFlag>) -> Result<(), Error>;
    fn delete_by_id(&self, id: i32) -> Result<(), Error>;
    fn update(&self, flag: &UpdateFlag) -> Result<(), Error>;
    fn skip_flags(&self, skip_time: NaiveDateTime);
}

pub struct SqliteFlagRepo<'a> {
    db_conn: &'a DbConn,
}

impl<'a> SqliteFlagRepo<'a> {
    pub fn new(conn: &'a DbConn) -> SqliteFlagRepo<'a> {
        SqliteFlagRepo { db_conn: conn }
    }
}

impl<'a> FlagRepo for SqliteFlagRepo<'a> {
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
}