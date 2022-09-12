use colored::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::PathBuf;

static CFG_PATH: &str = "/etc/image-server/";

#[derive(Serialize)]
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
        let toml = toml::to_string(&self).unwrap();

        let path = PathBuf::from(CFG_PATH);
        if !path.exists() {
            create_dir_all(&path)?;
        }

        let mut file = File::create(path.join("auth.toml"))?;

        file.write_all(toml.as_ref())?;
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

    println!(
        "{}",
        "It has also been saved to /etc/image-server/auth.toml".bright_black()
    );

    Ok(())
}
