use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("IO operation failed")]
    IO(#[from] io::Error),
    #[error("Decoder failed")]
    Decoder(#[from] image::ImageError),
    #[error("Encoder failed: {0}")]
    Encoder(String),
    #[error("Encoder failed to encode image")]
    AWebPEncoder(#[from] webp_animation::Error),
    #[error("Failed to find parent directory for {0}")]
    ParentNotFound(PathBuf),
    #[error("Failed to convert video file")]
    Ffmpeg,
}

pub struct FileData<'a> {
    pub data: &'a [u8],
    pub file_type: FileType<'a>,
}

pub enum FileType<'a> {
    Image,
    Gif,
    Video(&'a str),
}
