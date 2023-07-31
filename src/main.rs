#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

use rocket::{routes, Rocket, Build};
use sibears_farm::{db::connection::init_db, controllers::flag::get_flags};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(init_db())
        .mount("/", routes![hello])
        .mount("/api", routes![get_flags])
}