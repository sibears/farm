use std::ops::Deref;

use r2d2_diesel::ConnectionManager;
use r2d2::PooledConnection;
use r2d2::Pool;
use diesel::Connection;
use crate::models::flag::Flag;
use crate::db::schema::flags;
use crate::diesel::RunQueryDsl;
use crate::repos::flags_repo::flags::dsl::flags as flags_dsl;
use crate::db::connection::*;


pub trait FlagRepo {
    fn find_all(&self) -> Vec<Flag>;
    fn save_new(&self, flag: &Flag) -> usize;
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
    fn find_all(&self) -> Vec<Flag> {
        let conn = self.db_conn.master.deref();
        let all_flags = flags::table.load::<Flag>(conn).unwrap();
        all_flags
    }

    fn save_new(&self, flag: &Flag) -> usize {
        let conn = self.db_conn.master.deref();
        
        let count = diesel::insert_into(flags_dsl)
            .values(flag)
            .execute(conn)
            .unwrap();
        count
    }
}