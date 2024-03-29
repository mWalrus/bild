use crate::types::ConversionError;
use crate::util::FrameHelpers;
use crate::{util, UPLOADS_DIR};
use image::codecs::gif::GifDecoder;
use image::io::Reader;
use image::{AnimationDecoder, ImageDecoder};
use std::io::Cursor;
use std::time::SystemTime;
use std::{fs::File, io::Write, path::PathBuf};
use webp::{Encoder, WebPMemory};
use webp_animation::Encoder as AWebPEncoder;

pub fn image_to_webp(bytes: &[u8]) -> Result<(String, usize, SystemTime), ConversionError> {
    let image = Reader::new(Cursor::new(bytes));

    let guessed_format = image.with_guessed_format().map_err(ConversionError::IO)?;

    let decoded_img = guessed_format.decode().map_err(ConversionError::Decoder)?;

    // Create the encoder from the DynamicImage
    let encoder = Encoder::from_image(&decoded_img);
    if let Err(e) = encoder {
        return Err(ConversionError::Encoder(e.into()));
    }

    // Encode image into WebPMemory
    let encoded_webp: WebPMemory = encoder.unwrap().encode(90f32);

    // Generate a unique file name for the new converted file
    let webp_file_name = util::generate_file_name();

    // Put webp-image in a separate webp-folder in the location of the original
    let webp_file_path: PathBuf = PathBuf::from(UPLOADS_DIR).join(&webp_file_name);

    // Create the parent directory if it doesn't exist
    if let Some(parent_directory) = webp_file_path.parent() {
        std::fs::create_dir_all(parent_directory).map_err(ConversionError::IO)?;
    } else {
        return Err(ConversionError::ParentNotFound(webp_file_path));
    }

    // Create the image file
    let mut webp_image =
        File::create(&webp_file_path.with_extension("webp")).map_err(ConversionError::IO)?;

    // Write to file
    webp_image
        .write_all(&encoded_webp)
        .map_err(ConversionError::IO)?;
    Ok((webp_file_name, encoded_webp.len(), SystemTime::now()))
}

pub fn gif_to_webp(bytes: &[u8]) -> Result<(String, usize, SystemTime), ConversionError> {
    // https://github.com/blaind/webp-animation/blob/main/examples/encode_animation.rs
    // https://docs.rs/webp/latest/webp/struct.Encoder.html
    // https://docs.rs/image/latest/image/codecs/gif/index.html
    let decoder = GifDecoder::new(bytes).map_err(ConversionError::Decoder)?;
    let (w, h) = decoder.dimensions();
    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(ConversionError::Decoder)?;

    let mut encoder = AWebPEncoder::new((w, h)).map_err(ConversionError::AWebPEncoder)?;

    let mut timestamp = 0i32;
    let mut timestamps = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        timestamps.push(timestamp);
        timestamp += frame.delay_to_ms();
    }

    for (frame, timestamp) in frames.iter().zip(timestamps) {
        encoder
            .add_frame(frame.buffer(), timestamp)
            .map_err(ConversionError::AWebPEncoder)?;
    }

    let webp_data = encoder
        .finalize(timestamp)
        .map_err(ConversionError::AWebPEncoder)?;
    let new_webp_file_name = util::generate_file_name();
    let output = format!("{UPLOADS_DIR}/{new_webp_file_name}.webp");

    std::fs::write(output, &webp_data).map_err(ConversionError::IO)?;

    Ok((new_webp_file_name, webp_data.len(), SystemTime::now()))
}
