use crate::db::connection::*;
use crate::db::schema::flags as flags_schema;
use crate::db::schema::flags::status;
use crate::db::schema::flags::time;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::middleware::metrics::FLAG_COUNTER;
use crate::models::flag::Flag;
use crate::models::flag::FlagStatus;
use crate::models::flag::NewFlag;
use crate::models::flag::SavedFlag;
use crate::models::flag::UpdateFlag;
use crate::repos::flag::flags_schema::dsl::flags as flags_dsl;
use chrono::NaiveDateTime;
use diesel::QueryDsl;

use rocket::log::private::debug;

use crate::repos::errors::ReposError;

pub trait FlagRepo {
    type ReposError;

    fn find_all(&mut self) -> Result<Vec<Flag>, Self::ReposError>;
    fn find_by_id(&mut self, id: i32) -> Result<Flag, Self::ReposError>;
    fn save(&mut self, flag: &NewFlag) -> Result<usize, Self::ReposError>;
    fn save_all(&mut self, flag: &[NewFlag]) -> Result<usize, Self::ReposError>;
    fn delete_by_id(&mut self, id: i32) -> Result<usize, Self::ReposError>;
    fn update(&mut self, flag: &UpdateFlag) -> Result<usize, Self::ReposError>;
    fn skip_flags(&mut self, skip_time: NaiveDateTime) -> Result<usize, Self::ReposError>;
    fn get_limit(&mut self, limit: i64) -> Result<Vec<Flag>, Self::ReposError>;
    fn update_status(&mut self, flags: &[Flag]) -> Result<usize, Self::ReposError>;
    fn skip_duplicate(&mut self, flags: Vec<NewFlag>) -> Result<Vec<NewFlag>, Self::ReposError>;
    fn last_id(&mut self) -> Result<i32, Self::ReposError>;
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
    type ReposError = crate::repos::errors::ReposError;

    fn find_all(&mut self) -> Result<Vec<Flag>, Self::ReposError> {
        let conn = &mut self.db_conn.master;
        let all_flags = flags_schema::table.load::<Flag>(conn);
        all_flags.map_err(ReposError::NotFindFlagError)
    }

    fn find_by_id(&mut self, id: i32) -> Result<Flag, Self::ReposError> {
        let conn = &mut self.db_conn.master;
        let flag = flags_dsl.filter(flags_schema::dsl::id.eq(id)).first(conn);
        flag.map_err(ReposError::NotFindFlagError)
    }

    fn save(&mut self, flag: &NewFlag) -> Result<usize, Self::ReposError> {
        let conn = &mut self.db_conn.master;
        let flag = SavedFlag::from(flag);

        diesel::insert_into(flags_dsl)
            .values(flag)
            .execute(conn)
            .map_err(ReposError::FailSaveFlagError)
    }

    fn save_all(&mut self, flags: &[NewFlag]) -> Result<usize, Self::ReposError> {
        let conn = &mut self.db_conn.master;
        let flags: Vec<SavedFlag> = flags.iter().map(|item| SavedFlag::from(item)).collect();

        diesel::insert_into(flags_dsl)
            .values(&flags)
            .execute(conn)
            .map_err(ReposError::FailSaveFlagError)
    }

    fn delete_by_id(&mut self, id: i32) -> Result<usize, Self::ReposError> {
        let conn = &mut self.db_conn.master;

        diesel::delete(flags_dsl.filter(flags_schema::dsl::id.eq(id)))
            .execute(conn)
            .map_err(ReposError::DeleteFlagError)
    }

    fn update(&mut self, flag: &UpdateFlag) -> Result<usize, Self::ReposError> {
        let conn = &mut self.db_conn.master;

        diesel::update(flags_dsl.find(flag.id))
            .set(flag)
            .execute(conn)
            .map_err(ReposError::UpdateFlagError)
    }

    fn skip_flags(&mut self, skip_time: NaiveDateTime) -> Result<usize, Self::ReposError> {
        let conn = &mut self.db_conn.master;

        let res = diesel::update(
            flags_dsl
                .filter(time.lt(skip_time))
                .filter(status.eq(FlagStatus::QUEUED.to_string())),
        )
        .set(status.eq(FlagStatus::SKIPPED.to_string()))
        .execute(conn)
        .map_err(ReposError::UpdateFlagError)?;

        FLAG_COUNTER
            .with_label_values(&[FlagStatus::SKIPPED.to_string().as_str()])
            .add(res as i64);
        FLAG_COUNTER
            .with_label_values(&[FlagStatus::QUEUED.to_string().as_str()])
            .sub(res as i64);
        debug!("Skipped: {} flags", res);
        Ok(res)
    }

    fn get_limit(&mut self, limit: i64) -> Result<Vec<Flag>, Self::ReposError> {
        let conn = &mut self.db_conn.master;

        flags_dsl
            .filter(status.eq(FlagStatus::QUEUED.to_string()))
            .limit(limit)
            .load::<Flag>(conn)
            .map_err(ReposError::NotFindFlagError)
    }

    fn update_status(&mut self, flags: &[Flag]) -> Result<usize, Self::ReposError> {
        let conn = &mut self.db_conn.master;

        let mut final_res = 0;
        for flag in flags {
            let res = diesel::update(flags_dsl.find(flag.id))
                .set(flag)
                .execute(conn)
                .map_err(ReposError::UpdateFlagError)?;
            final_res += res;
        }
        Ok(final_res)
    }

    fn skip_duplicate(&mut self, mut flags: Vec<NewFlag>) -> Result<Vec<NewFlag>, ReposError> {
        let conn = &mut self.db_conn.master;

        let res = flags_dsl
            .select(flags_schema::dsl::flag)
            .load::<String>(conn)
            .map_err(ReposError::NotFindFlagError)?;
        flags.retain(|x| !res.contains(&x.flag.to_string()));
        Ok(flags)
    }

    fn last_id(&mut self) -> Result<i32, Self::ReposError> {
        let conn = &mut self.db_conn.master;

        let res = flags_dsl
            .select(flags_schema::dsl::id)
            .order(flags_schema::dsl::id.desc())
            .first::<i32>(conn)
            .map_err(ReposError::NotFindFlagError);

        return res
    }
}
