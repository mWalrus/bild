use crate::{gen, UPLOADS_DIR};
use image::codecs::gif::GifDecoder;
use image::io::Reader;
use image::DynamicImage;
use image::{AnimationDecoder, ImageDecoder};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{fs::File, path::PathBuf};
use webp::{Encoder, WebPMemory};
use webp_animation::Encoder as AWebPEncoder;

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

pub fn gif_to_webp(path: &PathBuf) -> Option<String> {
    // https://github.com/blaind/webp-animation/blob/main/examples/encode_animation.rs
    // https://docs.rs/webp/latest/webp/struct.Encoder.html
    // https://docs.rs/image/latest/image/codecs/gif/index.html
    let file_in = File::open(path).unwrap();
    let decoder = GifDecoder::new(file_in).unwrap();
    let (w, h) = decoder.dimensions().clone();
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");

    let mut encoder = AWebPEncoder::new((w, h)).unwrap();

    let mut total_time_ms = 0i32;
    frames.iter().for_each(|f| {
        let (n, d) = f.delay().numer_denom_ms();
        let d = n / d;
        total_time_ms += d as i32;
    });

    let frame_ms = (total_time_ms as f32 / frames.len() as f32) as i32;
    for (i, frame) in frames.iter().enumerate() {
        let frame_buffer = frame.buffer();
        // FIXME: this encodes each frame with same delay
        //        while gif might have differing delays.
        encoder
            .add_frame(&frame_buffer, (i as i32 * frame_ms) as i32)
            .unwrap();
    }

    let final_timestamp = (frames.len() as i32 * frame_ms) as i32;

    let webp_data = encoder.finalize(final_timestamp).unwrap();
    let new_webp_file_name = gen::file_name();
    let output = format!("static/uploads/{new_webp_file_name}.webp");
    match std::fs::write(output, webp_data) {
        Ok(_) => Some(new_webp_file_name),
        Err(_) => None,
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
