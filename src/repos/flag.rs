use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::QueryDsl;
use diesel::result::Error;
use rocket::log::private::debug;
use crate::db::schema::flags::status;
use crate::db::schema::flags::time;
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
use crate::middleware::metrics::FLAG_COUNTER;


pub trait FlagRepo {
    fn find_all(&self) -> Result<Vec<Flag>, Error>;
    fn find_by_id(&self, id: i32) -> Result<Flag, Error>;
    fn save(&self, flag: &NewFlag) -> Result<(), Error>;
    fn save_all(&self, flag: &[NewFlag]) -> Result<(), Error>;
    fn delete_by_id(&self, id: i32) -> Result<(), Error>;
    fn update(&self, flag: &UpdateFlag) -> Result<(), Error>;
    fn skip_flags(&self, skip_time: NaiveDateTime);
    fn get_limit(&self, limit: i64) -> Vec<Flag>;
    fn update_status(&self, flags: &[Flag]);
    fn skip_duplicate(&self, flags: Vec<NewFlag>) -> Vec<NewFlag>;
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
        all_flags.map_err(|_| Error::new(ErrorKind::Other, "Failed to find all flags."))
    }

    fn find_by_id(&self, id: i32) -> Result<Flag, Error> {
        let conn = self.db_conn.master.deref();
        let flag = flags_dsl.filter(flags::dsl::id.eq(id)).first(conn);
        flag.map_err(|_| Error::new(ErrorKind::Other, "Failed to find flag by ID."))
    }

    fn save(&self, flag: &NewFlag) {
        let conn = self.db_conn.master.deref();
        let flag = SavedFlag::from(flag);

        match diesel::insert_into(flags_dsl)
            .values(flag)
            .execute(conn)
        {
            Ok(result) => {
                println!("Flag saved successfully.");
            }
            Err(diesel::result::Error::NotFound) => {
                println!("Failed to save flag in database.");
            }
            Err(_) => {
                println!("Unknown error while flag saving.");
            }
        }
    }


    fn save_all(&self, flags: &[NewFlag]) {
        let conn = self.db_conn.master.deref();
        let flags: Vec<SavedFlag> = flags.iter().map(|item| SavedFlag::from(item)).collect();

        match diesel::insert_into(flags_dsl)
            .values(&flags)
            .execute(conn)
        {
            Ok(result) => {
                println!("{} flags saved successfully.", result);
            }
            Err(diesel::result::Error::NotFound) => {
                println!("Failed to save flags in database.");
            }
            Err(_) => {
                println!("Unknown error while flags saving.");
            }
        }
    }

    fn delete_by_id(&self, id: i32) -> Result<(), Error> {
        let conn = self.db_conn.master.deref();
        let result = diesel::delete(flags_dsl.filter(flags::dsl::id.eq(id)))
            .execute(conn)?;
        
        if result == 1 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Failed to delete flag by ID."))
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
        FLAG_COUNTER.with_label_values(&[FlagStatus::SKIPPED.to_string().as_str()]).add(res as i64);
        FLAG_COUNTER.with_label_values(&[FlagStatus::QUEUED.to_string().as_str()]).sub(res as i64);
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

    fn update_status(&self, flags: &[Flag]) {
        let conn = self.db_conn.master.deref();

        for flag in flags {
            if let Err(err) = diesel::update(flags_dsl.find(flag.id))
                .set(flag)
                .execute(conn)
            {
                println!("Failed to update flag: {}", err);
            }
        }
    }

    fn skip_duplicate(&self, mut flags: Vec<NewFlag>) -> Vec<NewFlag> {
        let conn = self.db_conn.master.deref();

        match flags_dsl.select(flags::dsl::flag)
            .load::<String>(conn)
        {
            Ok(res) => {
                flags.retain(|x| !res.contains(&x.flag.to_string()));
                flags
            }
            Err(err) => {
                println!("Failed to skip duplicates: {}", err);
                Vec::new()
            }
        }
    }
}
