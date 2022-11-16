use crate::types::ConversionError;
use crate::{gen, UPLOADS_DIR};
use image::codecs::gif::GifDecoder;
use image::io::Reader;
use image::{AnimationDecoder, ImageDecoder};
use std::env::temp_dir;
use std::io::Cursor;
use std::process::Command;
use std::{fs::File, io::Write, path::PathBuf};
use webp::{Encoder, WebPMemory};
use webp_animation::Encoder as AWebPEncoder;

pub fn image_to_webp(bytes: &[u8]) -> Result<String, ConversionError> {
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
    let webp_file_name = gen::file_name();

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
    Ok(webp_file_name)
}

pub fn gif_to_webp(bytes: &[u8]) -> Result<String, ConversionError> {
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

    let mut total_time_ms = 0i32;
    frames.iter().for_each(|f| {
        let (n, d) = f.delay().numer_denom_ms();
        let d = n.checked_div(d).unwrap_or(0);
        total_time_ms += d as i32;
    });

    let frame_ms = (total_time_ms as f32 / frames.len() as f32) as i32;
    for (i, frame) in frames.iter().enumerate() {
        let frame_buffer = frame.buffer();
        // FIXME: this encodes each frame with same delay
        //        while gif might have differing delays.
        encoder
            .add_frame(frame_buffer, (i as i32 * frame_ms) as i32)
            .map_err(ConversionError::AWebPEncoder)?;
    }

    let final_timestamp = (frames.len() as i32 * frame_ms) as i32;

    let webp_data = encoder
        .finalize(final_timestamp)
        .map_err(ConversionError::AWebPEncoder)?;
    let new_webp_file_name = gen::file_name();
    let output = format!("{UPLOADS_DIR}/{new_webp_file_name}.webp");

    std::fs::write(output, webp_data).map_err(ConversionError::IO)?;

    Ok(new_webp_file_name)
}

pub fn video_to_mp4(bytes: &[u8], mime_type: &str) -> Result<String, ConversionError> {
    let new_vid_name = gen::file_name();
    let new_vid_path = PathBuf::from(UPLOADS_DIR)
        .join(&new_vid_name)
        .with_extension("mp4");

    // do nothing if the file is already mp4
    if mime_type == "video/mp4" {
        std::fs::write(new_vid_path, bytes).map_err(ConversionError::IO)?;
        return Ok(new_vid_name);
    }

    // FIXME: temp solution until I can figure out how to feed ffmpeg
    //        the image bytes.
    let tmp_path = temp_dir().join(format!("{new_vid_name}.bild"));
    std::fs::write(&tmp_path, bytes).map_err(ConversionError::IO)?;

    // just switch the containers if mkv
    // https://askubuntu.com/a/396906
    let ffmpeg_command = if mime_type == "video/x-matroska" {
        format!(
            "ffmpeg -i {} -codec copy {}",
            tmp_path.to_str().unwrap(),
            new_vid_path.to_str().unwrap()
        )
    } else {
        format!(
            "ffmpeg -i {} {}",
            tmp_path.to_str().unwrap(),
            new_vid_path.to_str().unwrap()
        )
    };

    let status = Command::new("sh")
        .args(["-c", &ffmpeg_command])
        .status()
        .map_err(ConversionError::IO)?;

    if !status.success() {
        Err(ConversionError::Ffmpeg)?;
    }
    std::thread::spawn(move || {
        // try to remove the temp file
        std::fs::remove_file(tmp_path).unwrap_or_else(|_| ());
    });
    Ok(new_vid_name)
}
