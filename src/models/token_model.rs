use std::future::{ready, Ready};

use actix_web::web::Payload;
use actix_web::{FromRequest, HttpRequest};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use websocket::native_tls::Identity;

use crate::models::base_response_model::BaseResponseModel;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub id: ObjectId,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JWT {
    pub claims: Claims,
}

// impl FromRequest for JWT {
//     type Error = Error;
//     type Future = Ready<Result<Claims, Error>>;

//     fn extract(req: &HttpRequest) -> Self::Future {
//         Self::from_request(req, &mut actix_web::dev::Payload::None)
//     }

//     fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
//         if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
//             if let Ok(user_json) = identity.id() {
//                 if let Ok(user) = serde_json::from_str(&user_json) {
//                     return ready(Ok(user));
//                 }
//             }
//         }

//         ready(BaseResponseModel::expired_token(None))
//     }

//     // async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorResponse> {
//     //     fn is_valid(key: &str) -> Result<Claims, Error> {
//     //         Ok(decode_jwt(String::from(key), None)?)
//     //     }

//     //     match req.headers().get_one("authorization") {
//     //         None => {
//     //             return Outcome::Failure((
//     //                 Status::Unauthorized,
//     //                 BaseResponseModel::invalid_token(None),
//     //             ));
//     //         }
//     //         Some(key) => match is_valid(key) {
//     //             Ok(claims) => Outcome::Success(JWT { claims }),
//     //             Err(err) => match &err.kind() {
//     //                 jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
//     //                     return Outcome::Failure((
//     //                         Status::Unauthorized,
//     //                         BaseResponseModel::expired_token(None),
//     //                     ));
//     //                 }
//     //                 _ => {
//     //                     return Outcome::Failure((
//     //                         Status::Unauthorized,
//     //                         BaseResponseModel::invalid_token(None),
//     //                     ));
//     //                 }
//     //             },
//     //         },
//     //     }
//     // }
// }
