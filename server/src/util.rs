use std::path::{Path, PathBuf};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::UPLOADS_DIR;

pub fn file_name() -> String {
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
    } else if file_path.with_extension("mp4").exists() {
        Some(file_path.with_extension("mp4"))
    } else {
        None
    }
}
