use crate::api_key::ApiKey;
use crate::{convert_image, gen};
use lazy_static::lazy_static;
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::{json, Value};
use rocket::Request;
use rocket_governor::{Method, Quota, RocketGovernable, RocketGovernor};
use std::path::{Path, PathBuf};

lazy_static! {
    static ref RATE_LIMIT: u32 = std::env::var("ROCKET_RATE_LIMIT")
        .unwrap_or_else(|_| "2".into())
        .parse()
        .unwrap();
    static ref SERVER_URL: String =
        std::env::var("ROCKET_SERVER_URL").unwrap_or_else(|_| "http://localhost:1337".to_string());
}

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_second(Self::nonzero(*RATE_LIMIT))
    }
}

#[catch(404)]
pub fn not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("error_pages/404.html"))
}
#[catch(500)]
pub fn internal_error() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("error_pages/500.html"))
}

#[catch(429)]
pub fn too_many_requests() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("error_pages/429.html"))
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("Something went wrong: {status} ({})", req.uri())
}

#[get("/<file..>")]
pub async fn file(file: PathBuf) -> Option<NamedFile> {
    let mut file_path = Path::new("static/uploads/").join(file);
    if file_path.extension().is_none() {
        file_path = file_path.with_extension("webp");
    }
    if !file_path.exists() {
        return None;
    }
    NamedFile::open(file_path).await.ok()
}

#[post("/upload", data = "<file>")]
pub async fn upload(
    mut file: Form<TempFile<'_>>,
    _lg: RocketGovernor<'_, RateLimitGuard>,
    _key: ApiKey<'_>,
) -> status::Custom<Value> {
    let tmp_file_path = format!("/tmp/{}", gen::file_name(".png"));
    file.persist_to(&tmp_file_path).await.unwrap();
    // FIXME: handle image conversion in separate thread
    if let Some(file_name) = convert_image::to_webp(&tmp_file_path) {
        let url = format!("{}/{file_name}", *SERVER_URL);
        return status::Custom(Status::Ok, json!({ "url": url }));
    }
    status::Custom(
        Status::InternalServerError,
        json!({"message": "failed to upload image"}),
    )
}
