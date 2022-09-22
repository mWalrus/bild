use crate::{gen, UPLOADS_DIR};
use image::io::Reader;
use image::DynamicImage;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{fs::File, path::PathBuf};
use webp::{Encoder, WebPMemory};

pub fn image_to_webp(path: &PathBuf) -> Option<String> {
    let image: DynamicImage = Reader::open(path)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    // Create the encoder from the DynamicImage
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    // Encode image into WebPMemory
    let encoded_webp: WebPMemory = encoder.encode(90f32);

    // Generate a unique file name for the new converted file
    let webp_file_name = gen::file_name();

    // Put webp-image in a separate webp-folder in the location of the original
    let webp_file_path: PathBuf = PathBuf::from(UPLOADS_DIR).join(&webp_file_name);

    // Create the parent directory if it doesn't exist
    let parent_directory: &Path = webp_file_path.parent().unwrap();
    match std::fs::create_dir_all(parent_directory) {
        Ok(_) => {}
        Err(e) => {
            println!("Error {e}");
            return None;
        }
    }

    // Create the image file
    let mut webp_image = File::create(&webp_file_path.with_extension("webp")).unwrap();

    // Write to file
    match webp_image.write_all(&encoded_webp) {
        Ok(_) => {
            // remove old file before finishing
            std::fs::remove_file(path).unwrap();
            Some(webp_file_name)
        }
        Err(e) => {
            println!("Error: {e}");
            None
        }
    }
}

pub fn video_to_mp4(path: &PathBuf, mime_type: &str) -> Option<String> {
    let new_vid_name = gen::file_name();
    let new_vid_path = PathBuf::from(UPLOADS_DIR)
        .join(&new_vid_name)
        .with_extension("mp4");

    // do nothing if the file is already mp4
    if mime_type == "video/mp4" {
        std::fs::copy(path, new_vid_path).unwrap();
        // delete temp file
        std::fs::remove_file(path).unwrap();
        return Some(new_vid_name);
    }

    // just switch the containers if mkv
    // https://askubuntu.com/a/396906
    let ffmpeg_command = if mime_type == "video/x-matroska" {
        format!(
            "ffmpeg -i {} -codec copy {}",
            path.to_str().unwrap(),
            new_vid_path.to_str().unwrap()
        )
    } else {
        format!(
            "ffmpeg -i {} {}",
            path.to_str().unwrap(),
            new_vid_path.to_str().unwrap()
        )
    };

    let output = Command::new("sh").args(["-c", &ffmpeg_command]).output();

    match output {
        Ok(_) => {
            return Some(new_vid_name);
        }
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    }
}
