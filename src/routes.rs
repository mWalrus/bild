use crate::{convert_image, gen};
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::content;
use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Value;
use rocket::Request;
use std::path::{Path, PathBuf};

#[catch(404)]
pub fn not_found() -> content::RawHtml<String> {
    content::RawHtml(include_str!("error_pages/404.html").to_string())
}
#[catch(500)]
pub fn internal_error() -> content::RawHtml<String> {
    content::RawHtml(include_str!("error_pages/500.html").to_string())
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("Something went wrong: {status} ({})", req.uri())
}

#[get("/<file..>")]
pub async fn file(file: PathBuf) -> Option<NamedFile> {
    let mut file_path = Path::new("static/uploads/").join(file);
    // sometimes the user might open the image without
    // the file extension present, in that case we want
    // to redirect the user to the file.
    if file_path.extension().is_none() {
        file_path = file_path.with_extension("webp");
    }
    if file_path.exists() {
        return NamedFile::open(file_path).await.ok();
    }
    None
}

#[post("/upload", data = "<file>")]
pub async fn upload(mut file: Form<TempFile<'_>>) -> status::Custom<Value> {
    let tmp_file_path = format!("/tmp/{}.png", gen::file_name());
    file.persist_to(&tmp_file_path).await.unwrap();
    if let Some(file_name) = convert_image::to_webp(&tmp_file_path) {
        let url = format!("http://localhost:8000/i/{file_name}");
        return status::Custom(Status::Ok, json!({ "url": url }));
    }
    status::Custom(
        Status::InternalServerError,
        json!({"message": "failed to upload image"}),
    )
}
