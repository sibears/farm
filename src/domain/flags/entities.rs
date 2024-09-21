use chrono::NaiveDateTime;

pub struct Flag {
    pub id: i32,
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
    pub time: NaiveDateTime,
    pub status: String,
    pub checksystem_response: Option<String>,
}

pub struct NewFlag {
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
}