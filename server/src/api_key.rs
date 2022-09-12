use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use std::fs;

#[derive(FromForm)]
pub struct ApiKey<'a>(&'a str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey<'a> {
    type Error = ApiKeyError;

    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        /* .. */
        fn is_valid(key: &str) -> bool {
            let key_on_disk = fs::read_to_string("/etc/image-server/auth.key").unwrap();
            let key = key.replace("Bearer ", "");
            println!("keys are equal: {}", key_on_disk == key);
            key == key_on_disk
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}
