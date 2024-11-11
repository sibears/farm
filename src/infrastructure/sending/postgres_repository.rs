use crate::domain::flags::entities::Flag;
use crate::domain::sending::repository::SendingDomain;
use diesel::result::Error as DieselError;
use diesel::PgConnection;
use std::sync::Mutex;

pub struct PostgresSendingRepo {
    pub conn: Mutex<PgConnection>,
}

impl SendingDomain for PostgresSendingRepo {
    type SendingError = DieselError;

    // pub fn get_flags(&self, limit: i32) -> Result<Vec<Flag>, Self::SendingError> {}
}
