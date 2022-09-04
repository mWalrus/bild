#[macro_use]
extern crate rocket;
mod convert_image;
mod gen;
mod routes;

use routes::{default, internal_error, not_found};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, internal_error, default])
        .mount("/", routes![routes::upload, routes::file])
}
