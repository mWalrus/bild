use std::time::Duration;
use std::{fs, thread};

use crate::UPLOAD_MAX_AGE;

static SLEEP_TIMER: Duration = Duration::new(60 * 60 * 2, 0); // 2 hours

// Deletes files older than the specified MAX_AGE value, which is 2 weeks by default.
// This loop runs every 2 hours, as indicated by the SLEEP_TIMER.
pub fn run_collector() {
    thread::spawn(move || -> Result<(), std::io::Error> {
        loop {
            if let Ok(rd) = fs::read_dir("./uploads") {
                for entry in rd {
                    let entry = entry?;
                    // skip the file if it shouldn't be dealt with.
                    if entry.metadata()?.created()?.elapsed().unwrap() < *UPLOAD_MAX_AGE {
                        continue;
                    }
                    fs::remove_file(entry.path())?;
                    println!(
                        "[Garbage collector]: deleted file {}",
                        entry.file_name().into_string().unwrap()
                    );
                }
            }
            thread::sleep(SLEEP_TIMER);
        }
    });
}
