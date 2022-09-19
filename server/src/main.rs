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
use std::time::Duration;

macro_rules! env {
    ( $v:expr, $d:expr ) => {
        std::env::var($v).unwrap_or_else(|_| $d.into())
    };
}

lazy_static! {
    pub static ref RATE_LIMIT: u32 = env!("ROCKET_RATE_LIMIT", "2").parse().unwrap();
    pub static ref SERVER_URL: String = env!("ROCKET_SERVER_URL", "http://localhost:1337");
    pub static ref UPLOAD_MAX_AGE: Duration = {
        let num_weeks: u64 = env!("ROCKET_FILE_AGE_WEEKS", "2").parse().unwrap();
        Duration::new(60 * 60 * 24 * 7 * num_weeks, 0)
    };
    pub static ref GARBAGE_COLLECTOR: bool = env!("ROCKET_GARBAGE_COLLECTOR", "1") == "1";
    pub static ref UPLOAD_MAX_SIZE: u8 = env!("ROCKET_UPLOAD_MAX_SIZE", "5").parse().unwrap();
}

#[launch]
fn rocket() -> _ {
    if *GARBAGE_COLLECTOR {
        garbage::run_collector();
    }

    let config = Config {
        limits: Limits::default().limit("image", (*UPLOAD_MAX_SIZE).mebibytes()),
        ..Default::default()
    };

    rocket::custom(config)
        .register("/", catchers![not_found, internal_error, default])
        .mount("/", routes![routes::upload, routes::file])
}
