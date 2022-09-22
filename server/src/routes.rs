use crate::api_key::ApiKey;
use crate::{converter, gen};
use crate::{RATE_LIMIT, SERVER_URL};
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::{json, Value};
use rocket::{Config, Request};
use rocket_governor::{Method, Quota, RocketGovernable, RocketGovernor};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

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

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("Something went wrong: {status} ({})", req.uri())
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
    mut file: Form<TempFile<'_>>,
    _lg: RocketGovernor<'_, RateLimitGuard>,
    _key: ApiKey<'_>,
    config: &Config,
) -> status::Custom<Value> {
    // infer needs at most 39 bytes to figure out the video format
    let tmp_file_path = config.temp_dir.relative().join(gen::file_name());
    file.persist_to(&tmp_file_path).await.unwrap();

    // read 39 bytes into a buffer since thats all `infer` needs
    // to figure out the format of the uploaded file
    let mut f = File::open(&tmp_file_path).unwrap();
    let mut byte_buffer = [0; 39];
    f.read(&mut byte_buffer).unwrap();

    let mime_type = infer::get(&byte_buffer).unwrap().mime_type();

    let file_name = if mime_type == "image/gif" {
        converter::gif_to_webp(&tmp_file_path)
    } else if infer::is_video(&byte_buffer) {
        converter::video_to_mp4(&tmp_file_path, mime_type)
    } else if infer::is_image(&byte_buffer) {
        converter::image_to_webp(&tmp_file_path)
    } else {
        None
    };

    if let Some(file_name) = file_name {
        let url = format!("{}/{file_name}", *SERVER_URL);
        return status::Custom(Status::Ok, json!({ "url": url }));
    }
    status::Custom(
        Status::InternalServerError,
        json!({"message": "failed to upload file"}),
    )
}
