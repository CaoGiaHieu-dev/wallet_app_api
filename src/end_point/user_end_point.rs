use crate::models::user_model::UserModel;
use crate::repositories::mongo_repository::MongoRepo;
use crate::utils::helper;
use crate::{models::base_model::BaseModel, utils::string};
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

#[post("/register", format = "application/json", data = "<new_user>")]
pub fn register(
    db: &State<MongoRepo>,
    new_user: Json<UserModel>,
) -> Result<Json<BaseModel<UserModel>>, Status> {
    if new_user.email.is_none() || new_user.password.is_none() {
        return Err(Status::BadRequest);
    } else if new_user.email.to_owned().unwrap().is_empty()
        || new_user.password.to_owned().unwrap().is_empty()
    {
        return Err(Status::BadRequest);
    }

    let mut data = UserModel {
        id: None,
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        display_name: new_user.display_name.to_owned(),
    };
    if helper::validate_email(data.email.to_owned().unwrap().to_owned()) {
        let user_detail = db.register_user(&data);

        match user_detail {
            Ok(user) => {
                data.id = user.inserted_id.as_object_id();
                let response: BaseModel<UserModel> = BaseModel {
                    status: 200,
                    time_stamp: helper::get_current_time(),
                    data: Some(data),
                    message: None,
                };
                Ok(Json(response))
            }
            Err(e) => {
                let mut response = BaseModel {
                    status: 500,
                    time_stamp: helper::get_current_time(),
                    data: None,
                    message: Some(e.to_string()),
                };
                match e {
                    mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                        println!("{:?}", message);
                        if message == string::EMAIL_EXITS {
                            response.status = 400;
                            Ok(Json(response))
                        } else {
                            return Err(Status::BadRequest);
                        }
                    }
                    _ => return Ok(Json(response)),
                }
            }
        }
    } else {
        let response: BaseModel<UserModel> = BaseModel {
            status: 400,
            time_stamp: helper::get_current_time(),
            data: None,
            message: Some(string::INVALID_EMAIL.to_string()),
        };
        return Ok(Json(response));
    }
}

#[get("/login", format = "application/json", data = "<user>")]
pub fn login(
    db: &State<MongoRepo>,
    user: Json<UserModel>,
) -> Result<Json<BaseModel<UserModel>>, Status> {
    let user_info = user.into_inner();
    if user_info.email.is_none() || user_info.password.is_none() {
        return Err(Status::BadRequest);
    } else if user_info.email.to_owned().unwrap().is_empty()
        || user_info.password.to_owned().unwrap().is_empty()
    {
        return Err(Status::BadRequest);
    }

    return find_in_db(db, user_info);
}

#[get("/<id>", format = "application/json")]
pub fn find_user(db: &State<MongoRepo>, id: String) -> Result<Json<BaseModel<UserModel>>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let obj_id = ObjectId::parse_str(id);
    match obj_id {
        Ok(user_id) => {
            let user_info = UserModel {
                id: Some(user_id),
                email: None,
                password: None,
                display_name: None,
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
            status: 200,
            time_stamp: helper::get_current_time(),
            data: Some(user),
            message: None,
        })),
        Err(e) => match e {
            mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                let mut response: BaseModel<UserModel> = BaseModel {
                    status: 500,
                    time_stamp: helper::get_current_time(),
                    data: None,
                    message: Some(message.clone()),
                };
                if message == string::NOT_FOUND {
                    response.status = 400;
                    Ok(Json(response))
                } else {
                    Ok(Json(response))
                }
            }
            _ => Err(Status::InternalServerError),
        },
    }
}
