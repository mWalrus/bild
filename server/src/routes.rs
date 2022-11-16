use crate::converter;
use crate::guards::{ApiKey, RateLimitGuard};
use crate::types::{FileData, FileType};
use crate::SERVER_URL;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::{status, Redirect};
use rocket::serde::json::{json, Value};
use rocket::Request;
use rocket_governor::RocketGovernor;
use std::path::{Path, PathBuf};

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/404")
}

#[catch(default)]
pub fn default(status: Status, req: &Request<'_>) -> String {
    format!("Something went wrong: {status} ({})", req.uri())
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("https://bild.waalrus.xyz"))
}

#[get("/<file..>")]
pub async fn file(file: PathBuf) -> Option<NamedFile> {
    let file_path = Path::new("uploads/").join(file);
    if file_path.extension().is_some() {
        return NamedFile::open(file_path).await.ok();
    } else if file_path.with_extension("webp").exists() {
        return NamedFile::open(file_path.with_extension("webp")).await.ok();
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
        FileType::Video(mime_type) => converter::video_to_mp4(bytes, mime_type),
    };

    if let Err(e) = conversion {
        return status::Custom(Status::InternalServerError, json!({"msg": e.to_string()}));
    }

    let url = format!("{}/{}", *SERVER_URL, conversion.unwrap());
    status::Custom(Status::Ok, json!({ "link": url }))
}
