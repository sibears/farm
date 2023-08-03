#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

use rocket::{routes, Rocket, Build};
use sibears_farm::controllers::flag::get_flag_by_id;
use sibears_farm::{db::connection::init_db, controllers::flag::get_flags};
use sibears_farm::config::{self, get_config};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    let config = get_config();
    rocket::build()
        .manage(init_db(&config.database))
        .mount("/", routes![hello])
        .mount("/api", routes![get_flags, get_flag_by_id])
}