use crate::gen;
use image::io::Reader;
use image::DynamicImage;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use webp::{Encoder, WebPMemory};

pub fn to_webp(path: &str) -> Option<String> {
    let image: DynamicImage = Reader::open(path).unwrap().decode().unwrap();

    // Create the encoder from the DynamicImage
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    // Encode image into WebPMemory
    let encoded_webp: WebPMemory = encoder.encode(90f32);

    // Generate a unique file name for the new converted file
    let webp_file_name = gen::file_name();

    // Put webp-image in a separate webp-folder in the location of the original
    let path: &Path = &Path::new("static/uploads/").join(&webp_file_name);

    // Create the parent directory if it doesn't exist
    let parent_directory: &Path = path.parent().unwrap();
    match std::fs::create_dir_all(parent_directory) {
        Ok(_) => {}
        Err(e) => {
            println!("Error {e}");
            return None;
        }
    }

    // Create the new file path
    let original_filename = path.file_stem().unwrap().to_str().unwrap();
    let webp_file_path = format!(
        "{}/{original_filename}.webp",
        parent_directory.to_str().unwrap()
    );

    // Create the image file
    let mut webp_image = File::create(&webp_file_path).unwrap();

    // Write to file
    match webp_image.write_all(&encoded_webp) {
        Ok(_) => return Some(webp_file_name),
        Err(e) => {
            println!("Error: {e}");
            return None;
        }
    }
}
