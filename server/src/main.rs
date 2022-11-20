#[macro_use]
extern crate rocket;
mod converter;
mod garbage;
mod gen;
mod guards;
mod routes;
mod types;

use lazy_static::lazy_static;
use rocket::data::{Limits, ToByteUnit};
use rocket::fs::FileServer;
use rocket::Config;
use routes::{default, not_found};
use std::time::Duration;

macro_rules! bild_env {
    ( $v:expr, $d:expr ) => {
        std::env::var($v).unwrap_or_else(|_| $d.into())
    };
}

lazy_static! {
    pub static ref RATE_LIMIT: u32 = bild_env!("ROCKET_RATE_LIMIT", "10").parse().unwrap();
    pub static ref SERVER_URL: String = bild_env!("ROCKET_SERVER_URL", "http://localhost:1337");
    pub static ref UPLOAD_MAX_AGE: Duration = {
        let num_weeks: u64 = bild_env!("ROCKET_FILE_AGE_WEEKS", "2").parse().unwrap();
        Duration::new(60 * 60 * 24 * 7 * num_weeks, 0)
    };
    pub static ref GARBAGE_COLLECTOR: bool = bild_env!("ROCKET_GARBAGE_COLLECTOR", "1") == "1";
    pub static ref UPLOAD_MAX_SIZE: u8 = bild_env!("ROCKET_UPLOAD_MAX_SIZE", "20").parse().unwrap();
}

pub static UPLOADS_DIR: &str = "uploads/";

#[launch]
fn rocket() -> _ {
    if *GARBAGE_COLLECTOR {
        garbage::run_collector();
    }

    let config = Config {
        limits: Limits::default()
            .limit("data-form", (*UPLOAD_MAX_SIZE).mebibytes())
            .limit("upload/image", (*UPLOAD_MAX_SIZE).mebibytes()),
        port: 1337,
        ..Default::default()
    };

    rocket::custom(config)
        .register("/", catchers![not_found, default])
        .mount(
            "/",
            routes![
                routes::index,
                routes::upload,
                routes::file,
                routes::token_validation,
                routes::delete_upload
            ],
        )
        .mount(
            "/404",
            FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/404")).rank(-2),
        )
        .mount(
            "/upload",
            FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/upload")).rank(-3),
        )
}
