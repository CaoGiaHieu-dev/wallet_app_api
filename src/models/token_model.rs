use jsonwebtoken::errors::Error;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

use crate::utils::ErrorResponse;
use crate::{models::base_response_model::BaseResponseModel, utils::helper::decode_jwt};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub id: ObjectId,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ErrorResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key), None)?)
        }

        match req.headers().get_one("authorization") {
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    BaseResponseModel::invalid_token(None),
                ));
            }
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        return Outcome::Failure((
                            Status::Unauthorized,
                            BaseResponseModel::expired_token(None),
                        ));
                    }
                    _ => {
                        return Outcome::Failure((
                            Status::Unauthorized,
                            BaseResponseModel::invalid_token(None),
                        ));
                    }
                },
            },
        }
    }
}
