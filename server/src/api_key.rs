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
        fn is_valid(key: &str) -> bool {
            key.replace("Bearer ", "") == fs::read_to_string("/etc/bild-server/auth.key").unwrap()
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) => {
                if is_valid(key) {
                    return Outcome::Success(ApiKey(key));
                }
                Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
            }
        }
    }
}
