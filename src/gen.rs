use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn file_name(ext: &str) -> String {
    let name: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    name + ext
}
