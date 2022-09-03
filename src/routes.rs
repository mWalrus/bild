use crate::convert_image;
use anyhow::Result;
use lazy_static::lazy_static;
use rocket::data::Capped;
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use std::path::{Path, PathBuf};

// #[derive(FromForm)]
// struct Upload<'r> {
//     save: bool,
//     upload: TempFile<'r>,
// }

#[get("/<file..>")]
pub async fn index(file: PathBuf) -> Option<NamedFile> {
    let file_path = Path::new("static/uploads/").join(file);
    if file_path.exists() {
        return NamedFile::open(file_path).await.ok();
    }
    None
}

#[post("/upload", data = "<file>")]
pub async fn upload(file: Capped<&'_ str>) {
    if file.is_complete() {
        println!("poggers the file is sent");
        // 1. convert raw data to webp
        if let Some(img) = convert_image::to_webp(/*&file*/) {
            // 2. save webp file locally
            return;
        }
        println!("image conversion failed");
    }
    // let tmp_file: File = form.upload.into();
}
