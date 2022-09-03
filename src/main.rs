use rocket::fs::{relative, FileServer};

#[macro_use]
extern crate rocket;
mod convert_image;
mod routes;

#[launch]
fn rocket() -> _ {
    let _ = convert_image::to_webp();
    rocket::build()
        .mount("/", FileServer::from(relative!("static/index")))
        .mount("/i", routes![routes::upload, routes::index])
}
