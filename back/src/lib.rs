extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate diesel_enum;
extern crate serde_json;

pub mod application;
pub mod cors;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod schema;
