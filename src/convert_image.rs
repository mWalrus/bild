use crate::gen;
use image::io::Reader;
use image::DynamicImage;
use std::io::Write;
use std::path::Path;
use std::{fs::File, path::PathBuf};
use webp::{Encoder, WebPMemory};

pub fn to_webp(path: &str) -> Option<String> {
    let image: DynamicImage = Reader::open(path).unwrap().decode().unwrap();

    // Create the encoder from the DynamicImage
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    // Encode image into WebPMemory
    let encoded_webp: WebPMemory = encoder.encode(90f32);

    // Generate a unique file name for the new converted file
    let webp_file_name = gen::file_name(true);

    // Put webp-image in a separate webp-folder in the location of the original
    let mut webp_file_path: PathBuf = PathBuf::from("static/uploads/").join(&webp_file_name);

    if webp_file_path.exists() {
        webp_file_path = webp_file_path.with_file_name(gen::file_name(true));
    }

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
    let mut webp_image = File::create(&webp_file_path).unwrap();

    // Write to file
    match webp_image.write_all(&encoded_webp) {
        Ok(_) => return Some(webp_file_name.replace(".webp", "")),
        Err(e) => {
            println!("Error: {e}");
            return None;
        }
    }
}
