#[macro_use]
extern crate rocket;
mod api_key;
mod convert_image;
mod garbage;
mod gen;
mod routes;

use lazy_static::lazy_static;
use rocket::data::{Limits, ToByteUnit};
use rocket::Config;
use routes::{default, internal_error, not_found};

macro_rules! env {
    ( $v:expr, $d:expr ) => {
        std::env::var($v).unwrap_or_else(|_| $d.into())
    };
}

lazy_static! {
    pub static ref RATE_LIMIT: u32 = env!("ROCKET_RATE_LIMIT", "2").parse().unwrap();
    pub static ref SERVER_URL: String = env!("ROCKET_SERVER_URL", "http://localhost:1337");
}

#[launch]
fn rocket() -> _ {
    garbage::run_collector();

    let mut config = Config::default();
    config.limits = Limits::default().limit("image", 5.mebibytes());

    rocket::custom(config)
        .register("/", catchers![not_found, internal_error, default])
        .mount("/", routes![routes::upload, routes::file])
}
