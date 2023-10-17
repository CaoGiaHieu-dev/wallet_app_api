use crate::models::base_response_model::BaseResponseModel;
use crate::models::request_header_model::RequestHeaders;
use crate::models::token_model::JWT;
use crate::models::user_model::UserModel;
use crate::repositories::mongo_repository::MongoRepo;
use crate::service::user_service::UserService;
use crate::utils::helper::{create_jwt, decode_jwt, validate_token};
use crate::utils::ErrorResponse;
use jsonwebtoken::{Algorithm, Validation};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

#[post("/register", format = "application/json", data = "<new_user>")]
pub fn register(
    db: &State<MongoRepo>,
    new_user: Json<UserModel>,
) -> Result<Json<BaseResponseModel<UserModel>>, Status> {
    let validate_email = UserService::<'_>::validate_email(&new_user.email);

    if validate_email.is_err() {
        return Ok(BaseResponseModel::bad_request(validate_email.err()));
    }

    let validate_password = UserService::<'_>::validate_password(&new_user.password);

    if validate_password.is_err() {
        return Ok(BaseResponseModel::bad_request(validate_password.err()));
    }

    let mut data = UserModel {
        id: None,
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        display_name: new_user.display_name.to_owned(),
        ..Default::default()
    };

    match db.register_user(&data) {
        Ok(user) => {
            let user_id = user.inserted_id.as_object_id().expect("Cannot get user id");
            data.id = Some(user_id.clone());

            Ok(BaseResponseModel::success(Some(data)))
        }
        Err(e) => match e {
            mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                Ok(BaseResponseModel::bad_request(Some(message)))
            }
            _ => Ok(BaseResponseModel::internal_error(Some(e.to_string()))),
        },
    }
}

#[post["/renew_token", format = "application/json"]]

pub fn renew_token(
    db: &State<MongoRepo>,
    header: RequestHeaders<'_>,
) -> Result<Json<BaseResponseModel<String>>, Status> {
    let token = header.get_one("authorization").expect("Cannot found token");

    let mut validate = Validation::new(Algorithm::HS512);
    validate.validate_exp = false;

    match decode_jwt(token.to_string(), Some(validate)) {
        Ok(user_claims) => {
            let query = &UserModel {
                id: Some(user_claims.id),
                ..Default::default()
            };
            println!("{:?}", query);

            match db.find_user(query) {
                Ok(mut result) => match create_jwt(result.id.unwrap()) {
                    Ok(new_token) => {
                        println!("{:?}", new_token);

                        result.token = Some(new_token.clone());

                        match db.update_user(query, &result) {
                            Ok(success) => {
                                return Ok(BaseResponseModel::success(success.token));
                            }
                            Err(_) => return Ok(BaseResponseModel::internal_error(None)),
                        }
                    }
                    Err(_) => return Ok(BaseResponseModel::internal_error(None)),
                },
                Err(_) => return Ok(BaseResponseModel::not_found(None)),
            };
        }
        Err(error) => match error {
            _ => return Err(Status::BadRequest),
        },
    }
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login(
    db: &State<MongoRepo>,
    user: Json<UserModel>,
) -> Result<Json<BaseResponseModel<UserModel>>, Status> {
    let validate_email = UserService::<'_>::validate_email(&user.email);

    if validate_email.is_err() {
        return Ok(BaseResponseModel::bad_request(validate_email.err()));
    }

    let validate_password = UserService::<'_>::validate_password(&user.password);

    if validate_password.is_err() {
        return Ok(BaseResponseModel::bad_request(validate_password.err()));
    }

    match db.find_user(&user) {
        Ok(finder) => {
            let mut update = finder.clone();
            update.token = Some(create_jwt(update.id.unwrap()).expect("Cannot generate token"));

            match db.update_user(&finder, &update) {
                Ok(response) => {
                    println!("response {:?}", response);

                    Ok(BaseResponseModel::success(Some(response)))
                }
                Err(e) => Ok(BaseResponseModel::internal_error(Some(e.to_string()))),
            }
        }
        Err(e) => match e {
            mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                Ok(BaseResponseModel::not_found(Some(message)))
            }
            _ => Ok(BaseResponseModel::internal_error(Some(e.to_string()))),
        },
    }
}

#[get("/<id>", format = "application/json")]
pub fn find_user(
    db: &State<MongoRepo>,
    id: String,
) -> Result<Json<BaseResponseModel<UserModel>>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let user_service = UserService::new(db);

    match ObjectId::parse_str(id) {
        Ok(user_id) => {
            let user_info = UserModel {
                id: Some(user_id),
                ..Default::default()
            };

            return user_service.find_in_db(user_info);
        }
        Err(_) => {
            return Err(Status::BadRequest);
        }
    }
}

#[get["/info", format = "application/json"]]
pub fn info(
    db: &State<MongoRepo>,
    token: Result<JWT, ErrorResponse>,
) -> Result<Json<BaseResponseModel<UserModel>>, Status> {
    let user_id = match validate_token(token.clone()) {
        Ok(id) => id,
        Err(e) => return Ok(e.clone()),
    };

    println!("{:?}", user_id);

    let user_service = UserService::new(db);

    let user_info = UserModel {
        id: Some(user_id),
        ..Default::default()
    };

    match user_service.find_in_db(user_info) {
        Ok(user_in_db) => Ok(user_in_db),
        Err(error) => Err(error),
    }
}

#[post["/update_user", data ="<user>"]]
pub fn update_user(
    db: &State<MongoRepo>,
    user: Json<UserModel>,
    token: Result<JWT, ErrorResponse>,
) -> Result<Json<BaseResponseModel<UserModel>>, Status> {
    let user_id = match validate_token(token.clone()) {
        Ok(id) => id,
        Err(e) => return Ok(e.clone()),
    };

    let user_service = UserService::new(db);

    match user_service.update_to_db(user_id, user.0) {
        Ok(user_in_db) => Ok(user_in_db),
        Err(error) => Err(error),
    }
}
