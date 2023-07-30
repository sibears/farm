use std::borrow::Cow;
use std::time::{SystemTime};
use chrono::Utc;
use chrono::naive::NaiveDateTime;


use serde::Serialize;
use serde::Deserialize;
use crate::db::schema::flags;


#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[diesel(primary_key(id))]
#[table_name = "flags"]
pub struct Flag {
    #[diesel(deserialize_as = "i32")]
    pub id: Option<i32>,
    pub flag: Cow<'static, str>,
    sploit: Option<Cow<'static, str>>,
    team: Option<Cow<'static, str>>,
    time: NaiveDateTime,
    status: Option<Cow<'static, str>>,
    checksystem_response: Option<Cow<'static, str>>
}

impl Flag {
    pub fn new<S>(flag: S) -> Self 
        where S: Into<Cow<'static, str>>
    {
        Flag {
            id: None,
            flag: flag.into(),
            sploit: None,
            team: None,
            time: Utc::now().naive_local(),
            status: None,
            checksystem_response: None
        }
    }
}