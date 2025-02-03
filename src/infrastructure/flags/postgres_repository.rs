use crate::domain::flags::entities::{Flag, FlagStatus, SaveFlag};
use crate::domain::flags::repository::FlagRepo;
use crate::schema::flags::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_migrations::MigrationHarness;
use std::sync::Mutex;

use super::MIGRATIONS;

pub struct PostgresFlagRepo {
    pub conn: Mutex<PgConnection>,
}

impl PostgresFlagRepo {
    pub fn new(database_url: &str) -> Self {
        let mut conn =
            PgConnection::establish(database_url).expect("Error connecting to the database");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
        PostgresFlagRepo {
            conn: Mutex::new(conn),
        }
    }
}

impl FlagRepo for PostgresFlagRepo {
    type FlagRepoError = DieselError;

    fn get(&self, id_arg: i32) -> Result<Flag, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        flags.filter(id.eq(id_arg)).first::<Flag>(&mut *conn)
    }

    fn get_all(&self) -> Result<Vec<Flag>, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        flags.load::<Flag>(&mut *conn)
    }

    fn get_all_by_status(&self, flag_status: FlagStatus) -> Result<Vec<Flag>, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        flags
            .filter(status.eq(flag_status))
            .load::<Flag>(&mut *conn)
    }

    fn save(&self, flag_arg: &SaveFlag) -> Result<usize, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        diesel::insert_into(flags)
            .values(flag_arg)
            .execute(&mut *conn)
    }

    fn save_all(&self, flags_arg: &[SaveFlag]) -> Result<usize, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        diesel::insert_into(flags)
            .values(flags_arg)
            .execute(&mut *conn)
    }

    fn delete(&self, id_arg: i32) -> Result<usize, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        diesel::delete(flags.filter(id.eq(id_arg))).execute(&mut *conn)
    }

    fn delete_all(&self, flags_arg: &[Flag]) -> Result<usize, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        let ids: Vec<i32> = flags_arg.iter().map(|item| item.id).collect();
        diesel::delete(flags.filter(id.eq_any(ids))).execute(&mut *conn)
    }

    fn update(&self, flag_arg: &Flag) -> Result<usize, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        diesel::update(flags.filter(id.eq(flag_arg.id)))
            .set(flag_arg)
            .execute(&mut *conn)
    }

    fn update_all(&self, flags_arg: &[Flag]) -> Result<usize, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        let mut total_updated = 0;
        for next_flag in flags_arg {
            total_updated += diesel::update(
                crate::schema::flags::dsl::flags
                    .filter(crate::schema::flags::dsl::id.eq(next_flag.id)),
            )
            .set(next_flag)
            .execute(&mut *conn)?;
        }
        Ok(total_updated)
    }

    fn get_limit(&self, limit: u32) -> Result<Vec<Flag>, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        flags.limit(limit.into()).load::<Flag>(&mut *conn)
    }

    fn get_limit_with_offset(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        flags
            .limit(limit.into())
            .offset(offset.into())
            .load::<Flag>(&mut *conn)
    }

    fn get_last_id(&self) -> Result<i32, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        flags.select(id).order(id.desc()).first::<i32>(&mut *conn)
    }

    fn get_limit_by_status(
        &self,
        flag_status: FlagStatus,
        limit: u32,
    ) -> Result<Vec<Flag>, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        flags
            .filter(status.eq(flag_status))
            .limit(limit.into())
            .load::<Flag>(&mut *conn)
    }

    fn get_all_by_id(&self, ids: &[i32]) -> Result<Vec<Flag>, Self::FlagRepoError> {
        let mut conn = self.conn.lock().unwrap();
        flags.filter(id.eq_any(ids)).load::<Flag>(&mut *conn)
    }
}
