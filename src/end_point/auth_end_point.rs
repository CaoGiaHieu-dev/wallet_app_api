use crate::models::user_model::UserModel;
use crate::repositories::mongo_repository::MongoRepo;
use crate::service::user_service::UserService;
use crate::utils::helper::{create_jwt, decode_jwt};
use crate::{models::base_response_model::BaseResponseModel, utils::constants};
use actix_web::body::BoxBody;
use actix_web::{body::MessageBody, web::Json, HttpRequest, ResponseError};
use jsonwebtoken::{Algorithm, Validation};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

use actix_web::{
    get, post, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder, Result,
};

#[post("/register")]
pub async fn register(
    user_service: web::Data<UserService>,
    new_user: web::Json<UserModel>,
) -> impl Responder {
    let validate_email = user_service.validate_email(&new_user.email);

    if validate_email.is_err() {
        return UserModel::bad_request(validate_email.err());
    }

    let validate_password = user_service.validate_password(&new_user.password);

    if validate_password.is_err() {
        return UserModel::bad_request(validate_password.err());
    };

    match user_service.register_user(&new_user.0) {
        Ok(user) => return user.success(),
        Err(e) => return e,
    }
}

#[post["/renew_token"]]
pub async fn renew_token(user_service: web::Data<UserService>, req: HttpRequest) -> impl Responder {
    match req.headers().get(constants::AUTHORIZATION) {
        Some(token_raw) => {
            let token = token_raw.to_str().expect("Cannot parse token");
            let mut validate = Validation::new(Algorithm::HS512);
            validate.validate_exp = false;
            match decode_jwt(token.to_string(), Some(validate)) {
                Ok(claims) => match user_service.renew_token(claims.id.clone()) {
                    Ok(user_result) => return HttpResponse::Ok().json(user_result).body(),
                    Err(error) => return HttpResponse::BadRequest(),
                },
                Err(_) => HttpResponse::BadRequest(),
            }
        }
        None => HttpResponse::BadRequest(),
    }

    // let mut validate = Validation::new(Algorithm::HS512);
    // validate.validate_exp = false;

    // match decode_jwt(token.to_string(), Some(validate)) {
    //     Ok(user_claims) => {
    //         let query = &UserModel {
    //             id: Some(user_claims.id),
    //             ..Default::default()
    //         };
    //         println!("{:?}", query);

    //         match db.find_user(query) {
    //             Ok(mut result) => match create_jwt(result.id.unwrap()) {
    //                 Ok(new_token) => {
    //                     println!("{:?}", new_token);

    //                     result.token = Some(new_token.clone());

    //                     match db.update_user(query, &result) {
    //                         Ok(success) => {
    //                             return Ok(BaseResponseModel::success(success.token));
    //                         }
    //                         Err(_) => return Ok(BaseResponseModel::internal_error(None)),
    //                     }
    //                 }
    //                 Err(_) => return Ok(BaseResponseModel::internal_error(None)),
    //             },
    //             Err(_) => return Ok(BaseResponseModel::not_found(None)),
    //         };
    //     }
    //     Err(error) => match error {
    //         _ => return Err(Status::BadRequest),
    //     },
    // }
}

#[post("/login")]
pub async fn login(
    user_service: web::Data<UserService>,
    user: web::Json<UserModel>,
) -> impl Responder {
    let validate_email = user_service.validate_email(&user.email);

    if validate_email.is_err() {
        return UserModel::bad_request(validate_email.err());
    }

    let validate_password = user_service.validate_password(&user.password);

    if validate_password.is_err() {
        return UserModel::bad_request(validate_password.err());
    }

    match user_service.login(&user) {
        Ok(finder) => return finder.success(),
        Err(e) => return e,
    }
}

// #[get("/<id>")]
// pub async fn find_user(db: &State<web::Data<AppStateWithCounter>>, id: String) -> impl Responder {
//     if id.is_empty() {
//         return Err(Status::BadRequest);
//     };

//     let user_service = UserService::new(db);

//     match ObjectId::parse_str(id) {
//         Ok(user_id) => {
//             let user_info = UserModel {
//                 id: Some(user_id),
//                 ..Default::default()
//             };

//             return user_service.find_in_db(user_info);
//         }
//         Err(_) => {
//             return Err(Status::BadRequest);
//         }
//     }
// }

// #[get["/info"]]
// pub async fn info(db: &State<web::Data<AppStateWithCounter>>, token: Result<JWT, ErrorResponse>) -> impl Responder {

// let token = req
//     .headers()
//     .get(constants::AUTHORIZATION)
//     .expect("Missing token")
//     .to_str()
//     .expect("Cannot parse token");
//     let user_id = match validate_token(token.clone()) {
//         Ok(id) => id,
//         Err(e) => return Ok(e.clone()),
//     };

//     println!("{:?}", user_id);

//     let user_service = UserService::new(db);

//     let user_info = UserModel {
//         id: Some(user_id),
//         ..Default::default()
//     };

//     match user_service.find_in_db(user_info) {
//         Ok(user_in_db) => Ok(user_in_db),
//         Err(error) => Err(error),
//     }
// }

// #[post["/update_user"]]
// pub async fn update_user(
//     db: &State<web::Data<AppStateWithCounter>>,
//     user: Json<UserModel>,
//     token: Result<JWT, ErrorResponse>,
// ) -> impl Responder {
//     let user_id = match validate_token(token.clone()) {
//         Ok(id) => id,
//         Err(e) => return Ok(e.clone()),
//     };

//     let user_service = UserService::new(db);

//     match user_service.update_to_db(user_id, user.0) {
//         Ok(user_in_db) => Ok(user_in_db),
//         Err(error) => Err(error),
//     }
// }
