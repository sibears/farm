use std::borrow::Cow;
use std::time::{SystemTime};
use chrono::Utc;
use chrono::naive::NaiveDateTime;


use schemars::JsonSchema;
use serde::Serialize;
use serde::Deserialize;
use crate::db::schema::flags;

#[derive(Serialize, Deserialize, JsonSchema, Insertable, Debug)]
#[table_name = "flags"]
pub struct NewFlag {
    pub flag: Cow<'static, str>,
    pub sploit: Option<Cow<'static, str>>,
    pub team: Option<Cow<'static, str>>,
    time: Option<NaiveDateTime>
}

impl NewFlag {
    pub fn update_time(&mut self) {
        self.time = Some(Utc::now().naive_local());
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, AsChangeset)]
#[table_name = "flags"]
pub struct UpdateFlag {
    pub id: i32,
    pub flag: Cow<'static, str>,
    pub sploit: Option<Cow<'static, str>>,
    pub team: Option<Cow<'static, str>>,
    pub status: Option<Cow<'static, str>>,
    pub checksystem_response: Option<Cow<'static, str>>
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, AsChangeset, JsonSchema)]
#[diesel(primary_key(id))]
#[table_name = "flags"]
pub struct Flag {
    #[diesel(deserialize_as = "i32")]
    pub id: i32,
    pub flag: Cow<'static, str>,
    sploit: Option<Cow<'static, str>>,
    team: Option<Cow<'static, str>>,
    time: NaiveDateTime,
    status: Option<Cow<'static, str>>,
    checksystem_response: Option<Cow<'static, str>>
}

impl Flag {
    pub fn update_time(&mut self) {
        self.time = Utc::now().naive_local();
    }
}
