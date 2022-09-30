use crate::converter;
use crate::guards::{ApiKey, RateLimitGuard};
use crate::types::{FileData, FileType};
use crate::SERVER_URL;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::{json, Value};
use rocket::Request;
use rocket_governor::RocketGovernor;
use std::path::{Path, PathBuf};

#[catch(404)]
pub fn not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("error_pages/404.html"))
}

#[catch(default)]
pub fn default(status: Status, req: &Request<'_>) -> String {
    format!("Something went wrong: {status} ({})", req.uri(),)
}

#[get("/<file..>")]
pub async fn file(file: PathBuf) -> Option<NamedFile> {
    let mut file_path = Path::new("static/uploads/").join(file);
    if !file_path.with_extension("mp4").exists() && !file_path.with_extension("webp").exists() {
        return None;
    }

    file_path = file_path.with_extension("webp");
    if file_path.exists() {
        return NamedFile::open(file_path).await.ok();
    }

    file_path = file_path.with_extension("mp4");
    if file_path.exists() {
        return NamedFile::open(file_path).await.ok();
    }
    None
}

#[post("/upload", data = "<file>")]
pub async fn upload(
    file: Form<FileData<'_>>,
    _lg: RocketGovernor<'_, RateLimitGuard>,
    _key: ApiKey<'_>,
) -> status::Custom<Value> {
    let bytes = file.data;
    let ft = &file.file_type;

    let conversion = match ft {
        FileType::Gif => converter::gif_to_webp(bytes),
        FileType::Image => converter::image_to_webp(bytes),
    };

    if let Err(e) = conversion {
        return status::Custom(Status::InternalServerError, json!({"msg": e.to_string()}));
    }

    let url = format!("{}/{}", *SERVER_URL, conversion.unwrap());
    status::Custom(Status::Ok, json!({ "url": url }))
}
