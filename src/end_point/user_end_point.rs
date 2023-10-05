use crate::models::base_response_model::BaseResponseModel;
use crate::models::user_model::UserModel;
use crate::repositories::mongo_repository::MongoRepo;
use crate::service::user_service::UserService;
use crate::utils::helper::{create_jwt, encryption};
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

    let mut user_info = user.clone().into_inner();
    user_info.password = Some(encryption(user_info.password.unwrap()));

    match db.find_user(&user_info) {
        Ok(user) => {
            let id = user.id.expect("Cannot find user id");
            let token = create_jwt(id).expect("Cannot generate token");

            let mut user_update = user.clone();
            user_update.token = Some(token);
            user_update.password = Some(encryption(user.password.to_owned().unwrap()));

            let mut user_query = user.clone();
            user_query.password = Some(encryption(user.password.to_owned().unwrap()));

            match db.update_user(&user_query, &user_update) {
                Ok(update) => Ok(BaseResponseModel::success(Some(update))),
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

    let obj_id = ObjectId::parse_str(id);
    match obj_id {
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
