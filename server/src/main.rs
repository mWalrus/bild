#[macro_use]
extern crate rocket;
mod api_key;
mod convert_image;
mod garbage;
mod gen;
mod routes;

use rocket::data::{Limits, ToByteUnit};
use rocket::Config;
use routes::{default, internal_error, not_found};

#[launch]
fn rocket() -> _ {
    garbage::run_collector();

    let mut config = Config::default();
    let limits = Limits::default().limit("image", 5.mebibytes());
    config.limits = limits;

    rocket::custom(config)
        .register("/", catchers![not_found, internal_error, default])
        .mount("/", routes![routes::upload, routes::file])
}
