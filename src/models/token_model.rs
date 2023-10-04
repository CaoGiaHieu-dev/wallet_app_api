use jsonwebtoken::errors::Error;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

use crate::models::base_model::BaseModel;
use crate::utils::helper::decode_jwt;
use crate::utils::ErrorResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: ObjectId,
    pub exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ErrorResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                return Outcome::Failure((Status::Unauthorized, BaseModel::invalid_token(None)));
            }
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        return Outcome::Failure((
                            Status::Unauthorized,
                            BaseModel::expired_token(None),
                        ));
                    }
                    _ => {
                        return Outcome::Failure((
                            Status::Unauthorized,
                            BaseModel::invalid_token(None),
                        ));
                    }
                },
            },
        }
    }
}
