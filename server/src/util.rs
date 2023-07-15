use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::serde::json::{json, Value};
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::{SERVER_URL, UPLOADS_DIR};

pub fn generate_file_name() -> String {
    let get_name = || {
        thread_rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>()
    };
    let mut name = get_name();
    if PathBuf::from(UPLOADS_DIR).join(&name).exists() {
        name = get_name();
    }
    name
}

pub fn evaluate_file_from_path(file: &PathBuf) -> Option<PathBuf> {
    let file_path = Path::new("uploads/").join(file);
    if file_path.extension().is_some() {
        Some(file_path)
    } else if file_path.with_extension("webp").exists() {
        Some(file_path.with_extension("webp"))
    } else {
        None
    }
}

pub fn get_upload_history() -> Result<Vec<Value>, io::Error> {
    let mut data = Vec::new();
    for entry in fs::read_dir(UPLOADS_DIR)? {
        let e = entry?;
        let file_name = e.file_name().to_str().unwrap().replace(".webp", "");
        let metadata = e.metadata()?;
        let size = metadata.len();
        let created = metadata.created().unwrap();
        data.push((file_name, size, created));
    }

    data.sort_unstable_by_key(|(_, _, c)| c.elapsed().unwrap().as_millis());

    let mut history = Vec::new();
    for (name, size, created) in data {
        history.push(json!({
            "link": format!("{}/{}", *SERVER_URL, name),
            "deleteLink": format!("{}/delete/{}", *SERVER_URL, name),
            "size": size,
            "created": created
        }))
    }
    Ok(history)
}
