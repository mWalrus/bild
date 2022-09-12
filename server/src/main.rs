#[macro_use]
extern crate rocket;
mod api_key;
mod convert_image;
mod gen;
mod routes;

use routes::{default, internal_error, not_found, too_many_requests};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register(
            "/",
            catchers![not_found, internal_error, too_many_requests, default],
        )
        .mount("/i", routes![routes::upload, routes::file, routes::latest])
}
