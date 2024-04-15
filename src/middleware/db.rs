use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{request::Outcome, Request, State};

use crate::db::connection::{DbCollection, DbConn};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let pool = request.guard::<&State<DbCollection>>().await.unwrap();
        let db_pool = &pool.db_conn_pool;
        match db_pool.get() {
            Ok(conn) => Outcome::Success(DbConn { master: conn }),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
