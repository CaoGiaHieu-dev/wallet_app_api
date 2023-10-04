use crate::models::token_model::JWT;
use crate::models::user_model::UserModel;
use crate::repositories::mongo_repository::MongoRepo;
use crate::utils::helper::{self, create_jwt, encryption, get_current_time};
use crate::utils::{status_code, ErrorResponse};
use crate::{models::base_model::BaseModel, utils::constants};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

#[post("/register", format = "application/json", data = "<new_user>")]
pub fn register(
    db: &State<MongoRepo>,
    new_user: Json<UserModel>,
) -> Result<Json<BaseModel<UserModel>>, Status> {
    if new_user.email.is_none() || new_user.password.is_none() {
        return Ok(BaseModel::bad_request(None));
    } else if new_user.email.to_owned().unwrap().is_empty()
        || new_user.password.to_owned().unwrap().is_empty()
    {
        return Ok(BaseModel::bad_request(None));
    }

    let mut data = UserModel {
        id: None,
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        display_name: new_user.display_name.to_owned(),
        ..Default::default()
    };
    if helper::validate_email(data.email.to_owned().unwrap().to_owned()) {
        let user_detail = db.register_user(&data);

        match user_detail {
            Ok(user) => {
                let user_id = user.inserted_id.as_object_id().expect("Cannot get user id");
                data.id = Some(user_id.clone());

                let response = BaseModel {
                    status: status_code::SUCCESS,
                    time_stamp: helper::get_current_time(),
                    data: Some(data),
                    ..Default::default()
                };

                Ok(response.success())
            }
            Err(e) => match e {
                mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                    Ok(BaseModel::bad_request(Some(message)))
                }
                _ => Ok(BaseModel::internal_error(Some(e.to_string()))),
            },
        }
    } else {
        return Ok(BaseModel::bad_request(Some(
            constants::INVALID_EMAIL.to_string(),
        )));
    }
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login(
    db: &State<MongoRepo>,
    user: Json<UserModel>,
) -> Result<Json<BaseModel<UserModel>>, Status> {
    let mut user_info = user.clone().into_inner();
    if user_info.email.is_none() || user_info.password.is_none() {
        return Ok(BaseModel::bad_request(None));
    } else if user_info.email.to_owned().unwrap().is_empty()
        || user_info.password.to_owned().unwrap().is_empty()
    {
        return Ok(BaseModel::bad_request(None));
    }

    if helper::validate_email(user_info.email.to_owned().unwrap().to_owned()) {
        user_info.password = Some(encryption(user_info.password.to_owned().unwrap()));
        let user_detail = db.find_user(&user_info);

        match user_detail {
            Ok(user) => {
                let id = user.id.expect("Cannot find user id");
                let token = create_jwt(id).expect("Cannot generate token");

                let mut user_update = user.clone();
                user_update.token = Some(token);
                user_update.password = Some(encryption(user.password.to_owned().unwrap()));

                let mut user_query = user.clone();
                user_query.password = Some(encryption(user.password.to_owned().unwrap()));

                match db.update_user(&user_query, &user_update) {
                    Ok(update) => {
                        let response: BaseModel<UserModel> = BaseModel {
                            status: status_code::SUCCESS,
                            time_stamp: helper::get_current_time(),
                            data: Some(update),
                            ..Default::default()
                        };
                        Ok(response.success())
                    }
                    Err(e) => Ok(BaseModel::internal_error(Some(e.to_string()))),
                }
            }
            Err(e) => match e {
                mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                    Ok(BaseModel::not_found(Some(message)))
                }
                _ => Ok(BaseModel::internal_error(Some(e.to_string()))),
            },
        }
    } else {
        return Ok(BaseModel::bad_request(Some(
            constants::INVALID_EMAIL.to_string(),
        )));
    }
}

#[get("/<id>", format = "application/json")]
pub fn find_user(
    db: &State<MongoRepo>,
    id: String,
    key: Result<JWT, ErrorResponse>,
) -> Result<Json<BaseModel<UserModel>>, Status> {
    if key.is_err() {
        let error = key.err().unwrap().into_inner();
        let response_model: BaseModel<UserModel> = BaseModel {
            status: error.status,
            time_stamp: get_current_time(),
            message: error.message,
            ..Default::default()
        };
        return Ok(response_model.error());
    }

    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let obj_id = ObjectId::parse_str(id);
    match obj_id {
        Ok(user_id) => {
            let user_info = UserModel {
                id: Some(user_id),
                ..Default::default()
            };

            return find_in_db(db, user_info);
        }
        Err(_) => {
            return Err(Status::BadRequest);
        }
    }
}

fn find_in_db(
    db: &State<MongoRepo>,
    user_info: UserModel,
) -> Result<Json<BaseModel<UserModel>>, Status> {
    let user_detail = db.find_user(&user_info);
    match user_detail {
        Ok(user) => Ok(Json(BaseModel {
            status: status_code::SUCCESS,
            time_stamp: helper::get_current_time(),
            data: Some(user),
            message: None,
        })),
        Err(e) => match e {
            mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                let mut response: BaseModel<UserModel> = BaseModel {
                    status: status_code::INTERNAL_SERVER_ERROR,
                    time_stamp: helper::get_current_time(),
                    data: None,
                    message: Some(message.clone()),
                };
                if message == constants::NOT_FOUND {
                    response.status = status_code::NOT_FOUND;
                    Ok(response.error())
                } else {
                    Ok(response.error())
                }
            }
            _ => Err(Status::InternalServerError),
        },
    }
}
