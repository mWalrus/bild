use crate::guards::{ApiKey, RateLimitGuard};
use crate::types::{FileData, FileType};
use crate::SERVER_URL;
use crate::{converter, util};
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::{status, Redirect};
use rocket::serde::json::{json, Value};
use rocket::Request;
use rocket_governor::RocketGovernor;
use std::fs;
use std::path::PathBuf;

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/404")
}

#[catch(default)]
pub fn default(status: Status, req: &Request<'_>) -> status::Custom<Value> {
    // format!("Something went wrong: {status} ({}) req: {req}", req.uri())
    let err_msg = *req.local_cache(|| "");
    status::Custom(status, json!({ "message": err_msg }))
}

#[get("/bild")]
pub fn bild_home() -> Redirect {
    Redirect::to(uri!("https://bild.waalrus.xyz"))
}

#[get("/<file..>")]
pub async fn file(file: PathBuf) -> Option<NamedFile> {
    match util::evaluate_file_from_path(&file) {
        Some(p) => NamedFile::open(p).await.ok(),
        None => None,
    }
}

#[get("/history")]
pub async fn history(_key: ApiKey<'_>) -> status::Custom<Value> {
    match util::get_upload_history() {
        Ok(history) => status::Custom(Status::Ok, json!({ "history": history })),
        Err(e) => status::Custom(
            Status::InternalServerError,
            json!({ "message": format!("{e}") }),
        ),
    }
}

#[post("/token-validation")]
pub async fn token_validation(_key: ApiKey<'_>) -> status::Accepted<()> {
    status::Accepted::<()>(None)
}

#[get("/delete/<_..>")]
pub async fn get_delete() -> Option<NamedFile> {
    NamedFile::open("server/static/delete/index.html")
        .await
        .ok()
}

#[delete("/delete/<name>")]
pub async fn delete_upload(_key: ApiKey<'_>, name: PathBuf) -> status::Custom<Value> {
    let res = match util::evaluate_file_from_path(&name) {
        Some(path) => fs::remove_file(path),
        None => return status::Custom(Status::NotFound, json!({"message": "No such file"})),
    };

    match res {
        Ok(()) => status::Custom(
            Status::Ok,
            json!({ "message": format!("Deleted {}", name.display()) }),
        ),
        Err(e) => status::Custom(
            Status::InternalServerError,
            json!({"message": e.to_string()}),
        ),
    }
}

#[post("/upload", data = "<file>")]
pub async fn upload(
    _key: ApiKey<'_>,
    _lg: RocketGovernor<'_, RateLimitGuard>,
    file: Form<FileData<'_>>,
) -> status::Custom<Value> {
    let bytes = file.data;
    let ft = &file.file_type;

    let conversion = match ft {
        FileType::Gif => converter::gif_to_webp(bytes),
        FileType::Image => converter::image_to_webp(bytes),
    };

    let (name, size, created_at) = match conversion {
        Ok(file_info) => file_info,
        Err(e) => {
            return status::Custom(
                Status::InternalServerError,
                json!({"message": e.to_string()}),
            )
        }
    };

    status::Custom(
        Status::Ok,
        json!({
            "link": format!("{}/{}", *SERVER_URL, name),
            "delete_link": format!("{}/delete/{}", *SERVER_URL, name),
            "size": size,
            "created": created_at
        }),
    )
}
