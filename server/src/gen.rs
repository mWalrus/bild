use std::path::PathBuf;

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
