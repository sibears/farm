use chrono::naive::NaiveDateTime;
use chrono::{Local};

use crate::db::schema::flags;
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Eq)]
pub struct NewFlag {
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
}

impl NewFlag {
    pub fn new(flag: String) -> Self {
        NewFlag {
            flag,
            sploit: None,
            team: None,
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
#[diesel(table_name = flags)]
pub struct SavedFlag {
    flag: String,
    sploit: Option<String>,
    team: Option<String>,
    time: NaiveDateTime,
    status: String,
}

impl From<&NewFlag> for SavedFlag {
    fn from(new_flag: &NewFlag) -> Self {
        SavedFlag {
            flag: new_flag.flag.clone(),
            sploit: new_flag.sploit.clone(),
            team: new_flag.team.clone(),
            time: Local::now().naive_local(),
            status: FlagStatus::QUEUED.to_string().into(),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, AsChangeset)]
#[diesel(table_name = flags)]
pub struct UpdateFlag {
    pub id: i32,
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
    pub status: String,
    pub checksystem_response: Option<String>,
}

#[derive(
    Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, AsChangeset, JsonSchema, Clone,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = flags)]
pub struct Flag {
    pub id: i32,
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
    pub time: NaiveDateTime,
    pub status: String,
    pub checksystem_response: Option<String>,
}

impl Flag {
    pub fn update_time(&mut self) {
        self.time = Local::now().naive_local();
    }

    pub fn match_regex(&self, regex: &Regex) -> bool {
        regex.is_match(&self.flag)
    }
}

#[derive(Debug, EnumIter, Display, EnumString)]
pub enum FlagStatus {
    QUEUED,
    SKIPPED,
    ACCEPTED,
    REJECTED,
}
