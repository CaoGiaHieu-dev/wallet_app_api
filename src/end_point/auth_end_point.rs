use crate::models::user_model::UserModel;

use crate::service::user_service::UserService;
use crate::utils::constants;
use crate::utils::helper::{decode_jwt, validate_token};

use actix_web::{web::Json, HttpRequest};
use jsonwebtoken::{Algorithm, Validation};

use actix_web::{get, post, web, HttpResponse};

#[post("/register")]
pub async fn register(
    user_service: web::Data<UserService>,
    new_user: web::Json<UserModel>,
) -> HttpResponse {
    let validate_email = user_service.validate_email(&new_user.email);

    if validate_email.is_err() {
        let response = UserModel::bad_request(validate_email.err());
        return HttpResponse::Ok().json(response);
    }

    let validate_password = user_service.validate_password(&new_user.password);

    if validate_password.is_err() {
        let response = UserModel::bad_request(validate_password.err());
        return HttpResponse::Ok().json(response);
    };

    match user_service.register_user(&new_user.0) {
        Ok(user) => {
            return HttpResponse::Ok().json(Json(user));
        }
        Err(e) => return HttpResponse::Ok().json(Json(e)),
    }
}

#[post["/renew_token"]]
pub async fn renew_token(user_service: web::Data<UserService>, req: HttpRequest) -> HttpResponse {
    match req.headers().get(constants::AUTHORIZATION) {
        Some(token_raw) => {
            let token = token_raw.to_str().expect("Cannot parse token");
            let mut validate = Validation::new(Algorithm::HS512);
            validate.validate_exp = false;
            match decode_jwt(token.to_string(), Some(validate)) {
                Ok(claims) => match user_service.renew_token(claims.id.clone()) {
                    Ok(user_result) => {
                        let response = HttpResponse::Ok().json(user_result);
                        return response;
                    }
                    Err(_) => return HttpResponse::BadRequest().finish(),
                },
                Err(_) => HttpResponse::BadRequest().finish(),
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[post("/login")]
pub async fn login(
    user_service: web::Data<UserService>,
    user: web::Json<UserModel>,
) -> HttpResponse {
    let validate_email = user_service.validate_email(&user.email);

    if validate_email.is_err() {
        let response = UserModel::bad_request(validate_email.err());
        return HttpResponse::Ok().json(response);
    }

    let validate_password = user_service.validate_password(&user.password);

    if validate_password.is_err() {
        let response = UserModel::bad_request(validate_password.err());
        return HttpResponse::Ok().json(response);
    };

    match user_service.login(&user) {
        Ok(finder) => {
            let response = HttpResponse::Ok().json(finder);
            return response;
        }
        Err(e) => {
            let response = HttpResponse::Ok().json(e);
            return response;
        }
    }
}

#[get["/info"]]
pub async fn info(user_service: web::Data<UserService>, req: HttpRequest) -> HttpResponse {
    match req.headers().get(constants::AUTHORIZATION) {
        Some(token_raw) => {
            let token = token_raw.to_str().expect("Cannot parse token");

            match validate_token(token) {
                Ok(_) => match decode_jwt(token.to_string(), None) {
                    Ok(claims) => {
                        match user_service.get_info(UserModel {
                            id: Some(claims.id),
                            ..Default::default()
                        }) {
                            Ok(user_result) => {
                                let response = HttpResponse::Ok().json(user_result);
                                return response;
                            }
                            Err(e) => {
                                let response = HttpResponse::Ok().json(e);
                                return response;
                            }
                        }
                    }
                    Err(_) => HttpResponse::BadRequest().finish(),
                },
                Err(e) => {
                    let response = HttpResponse::Ok().json(e);
                    return response;
                }
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

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
