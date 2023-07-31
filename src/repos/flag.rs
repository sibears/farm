use std::ops::Deref;

use diesel::QueryDsl;
use r2d2_diesel::ConnectionManager;
use r2d2::PooledConnection;
use r2d2::Pool;
use diesel::Connection;
use crate::models::flag::Flag;
use crate::db::schema::flags;
use crate::diesel::RunQueryDsl;
use crate::repos::flag::flags::dsl::flags as flags_dsl;
use crate::db::connection::*;
use crate::diesel::ExpressionMethods;


pub trait FlagRepo {
    fn find_all(&self) -> Vec<Flag>;
    fn save_new(&self, flag: &Flag) -> usize;
    fn delete_by_id(&self, id: i32) -> usize;
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

    fn delete_by_id(&self, id: i32) -> usize {
        let conn = self.db_conn.master.deref();
    
        let count = diesel::delete(flags_dsl.filter(flags::dsl::id.eq(id)))
            .execute(conn)
            .unwrap();
        count
    }
}