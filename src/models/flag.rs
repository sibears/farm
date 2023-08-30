use std::borrow::Cow;
use std::fmt;
use std::time::{SystemTime};
use chrono::Utc;
use chrono::naive::NaiveDateTime;


use regex::Regex;
use schemars::JsonSchema;
use serde::Serialize;
use serde::Deserialize;
use crate::db::schema::flags;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Eq)]
pub struct NewFlag {
    pub flag: Cow<'static, str>,
    pub sploit: Option<Cow<'static, str>>,
    pub team: Option<Cow<'static, str>>,
}

impl NewFlag {
    pub fn new<S>(flag: S) -> Self 
        where S: Into<Cow<'static, str>> 
    {
        NewFlag { 
            flag: flag.into(), 
            sploit: None, 
            team: None 
        } 
    }
    pub fn match_regex(&self, regex: &Regex) -> bool {
        regex.is_match(&self.flag)
    }
}

impl PartialEq for NewFlag {
    fn eq(&self, other: &Self) -> bool {
        self.flag == other.flag
    }
}

impl PartialOrd for NewFlag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.flag.partial_cmp(&other.flag)
    }
}

impl Ord for NewFlag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.flag.cmp(&other.flag)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Insertable, Debug)]
#[table_name = "flags"]
pub struct SavedFlag {
    flag: Cow<'static, str>,
    sploit: Option<Cow<'static, str>>,
    team: Option<Cow<'static, str>>,
    time: NaiveDateTime,
    status: Cow<'static, str>,
}

impl From<&NewFlag> for SavedFlag {
    fn from(new_flag: &NewFlag) -> Self {
        SavedFlag { 
            flag: new_flag.flag.to_owned(), 
            sploit: new_flag.sploit.to_owned(), 
            team: new_flag.team.to_owned(), 
            time: Utc::now().naive_local(), 
            status: FlagStatus::QUEUED.to_string().into()
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, AsChangeset)]
#[table_name = "flags"]
pub struct UpdateFlag {
    pub id: i32,
    pub flag: Cow<'static, str>,
    pub sploit: Option<Cow<'static, str>>,
    pub team: Option<Cow<'static, str>>,
    pub status: Cow<'static, str>,
    pub checksystem_response: Option<Cow<'static, str>>
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, AsChangeset, JsonSchema, Clone)]
#[diesel(primary_key(id))]
#[diesel(table_name = flags)]
pub struct Flag {
    #[diesel(deserialize_as = "i32")]
    pub id: i32,
    pub flag: Cow<'static, str>,
    sploit: Option<Cow<'static, str>>,
    team: Option<Cow<'static, str>>,
    time: NaiveDateTime,
    pub status: Cow<'static, str>,
    pub checksystem_response: Option<Cow<'static, str>>
}

impl Flag {
    pub fn update_time(&mut self) {
        self.time = Utc::now().naive_local();
    }

    pub fn match_regex(&self, regex: &Regex) -> bool {
        regex.is_match(&self.flag)
    }
}



#[derive(Debug)]
pub enum FlagStatus {
    QUEUED,
    SKIPPED,
    ACCEPTED,
    REJECTED
}

impl fmt::Display for FlagStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}