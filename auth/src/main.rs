use colored::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::PathBuf;

static CFG_PATH: &str = "/etc/bild-server/";
static KEY_FILE: &str = "auth.key";

struct Auth {
    token: String,
}

impl Auth {
    fn new() -> Self {
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        Self { token }
    }

    fn persist(&self) -> Result<(), io::Error> {
        let path = PathBuf::from(CFG_PATH);
        if !path.exists() {
            create_dir_all(&path)?;
        }

        File::create(path.join(KEY_FILE))?.write_all(self.token.as_ref())?;
        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    let auth = Auth::new();
    auth.persist()?;
    let header = format!("Authorization: Bearer {}", auth.token);

    println!(
        "{}\n{header}",
        "Authorization header has been generated, paste this in chatterino:"
            .bright_green()
            .bold()
    );

    let hint = format!("The token has also been saved to {CFG_PATH}{KEY_FILE}");

    println!("{}", hint.bright_black());

    Ok(())
}
