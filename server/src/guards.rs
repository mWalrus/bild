use crate::types::{ApiKeyError, FileData, FileType};
use crate::RATE_LIMIT;
use rocket::form::{self, DataField, FromFormField};
use rocket::{
    data::ToByteUnit,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use rocket_governor::{Method, Quota, RocketGovernable};
use std::fs;

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_second(Self::nonzero(*RATE_LIMIT))
    }
}

#[derive(FromForm)]
pub struct ApiKey<'a>(&'a str);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey<'a> {
    type Error = ApiKeyError;

    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            key.replace("Bearer ", "") == fs::read_to_string("/etc/bild-server/auth.key").unwrap()
        }

        match req.headers().get_one("Authorization") {
            None => {
                req.local_cache(|| "API key is missing");
                Outcome::Failure((Status::BadRequest, ApiKeyError::Missing))
            }
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => {
                req.local_cache(|| "API key is invalid");
                Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for FileData<'r> {
    async fn from_data(mut field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let peeked = field.data.peek(39).await;
        let is_video = infer::is_video(peeked);
        let is_image = infer::is_image(peeked);

        if is_video || !is_image {
            Err(form::Error::validation(
                *field
                    .request
                    .local_cache(|| "Upload is neither an image nor a gif"),
            ))?;
        }

        let limit = field
            .request
            .limits()
            .get("upload/image")
            .unwrap_or_else(|| 20.mebibytes()); // defaults

        let mime_type = match infer::get(peeked) {
            Some(t) => t.mime_type(),
            None => Err(form::Error::validation(
                *field
                    .request
                    .local_cache(|| "Could not determine mime type of file"),
            ))?,
        };

        let file_type = if is_image && mime_type == "image/gif" {
            FileType::Gif
        } else {
            FileType::Image
        };

        let capped_bytes = match field.data.open(limit).into_bytes().await {
            Ok(bytes) => bytes,
            Err(_) => Err(form::Error::validation(
                *field.request.local_cache(|| "Failed to read file"),
            ))?,
        };

        if !capped_bytes.is_complete() {
            Err(form::Error::validation(
                *field.request.local_cache(|| "File too large"),
            ))?;
        }

        let bytes = capped_bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);
        Ok(FileData {
            data: bytes,
            file_type,
        })
    }
}
