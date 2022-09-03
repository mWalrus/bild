use anyhow::Result;
use image::io::Reader;
use image::{DynamicImage, ImageFormat};
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;
use webp::{Encoder, WebPMemory};

pub fn to_webp(/*raw: &str*/) -> Option<String> {
    // println!("raw: {raw}");
    // let mut reader = Reader::new(Cursor::new(raw))
    //     .with_guessed_format()
    //     .expect("Cursor io never fails");
    let file_path = "./static/uploads/2022-09-03_14-15.png";
    let image = Reader::open(file_path);
    let image: DynamicImage = match image {
        Ok(img) => img.with_guessed_format().unwrap().decode().unwrap(),
        Err(e) => {
            println!("Error: {e}");
            return None;
        }
    };

    // Create the encoder from the DynamicImage
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    // Encode image into WebPMemory
    let encoded_webp: WebPMemory = encoder.encode(90f32);

    // Put webp-image in a separate webp-folder in the location of the original
    let path: &Path = Path::new(file_path);
    let parent_directory: &Path = path.parent().unwrap();
    match std::fs::create_dir_all(parent_directory) {
        Ok(_) => {}
        Err(e) => {
            println!("Error {e}");
            return None;
        }
    }

    let original_filename = path.file_stem().unwrap().to_str().unwrap();
    let webp_file_path = format!(
        "{}/{original_filename}.webp",
        parent_directory.to_str().unwrap()
    );

    let mut webp_image = File::create(&webp_file_path).unwrap();

    match webp_image.write_all(&encoded_webp) {
        Ok(_) => return Some(webp_file_path),
        Err(e) => {
            println!("Error: {e}");
            return None;
        }
    }
}
