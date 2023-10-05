use rocket::{http::Status, serde::json::Json, State};

use crate::{
    models::{base_response_model::BaseResponseModel, user_model::UserModel},
    repositories::mongo_repository::MongoRepo,
    utils::{constants, helper, status_code},
};

#[derive()]
pub struct UserService<'a>(&'a State<MongoRepo>);

impl<'a> UserService<'a> {
    pub fn new(db: &'a State<MongoRepo>) -> Self {
        Self(db)
    }

    pub fn validate_email(email: &Option<String>) -> Result<(), String> {
        if email.is_none() {
            return Err(constants::EMAIL_EMPTY.to_string());
        }

        if email.to_owned().unwrap().is_empty() {
            return Err(constants::EMAIL_EMPTY.to_string());
        }

        if helper::validate_email(email.to_owned().unwrap().to_owned()) {
            return Err(constants::INVALID_EMAIL.to_string());
        }

        return Ok(());
    }

    pub fn validate_password(password: &Option<String>) -> Result<(), String> {
        if password.is_none() {
            return Err(constants::PASSWORD_EMPTY.to_string());
        }

        if password.to_owned().unwrap().is_empty() {
            return Err(constants::PASSWORD_EMPTY.to_string());
        }

        return Ok(());
    }

    pub fn find_in_db(
        &self,
        user_info: UserModel,
    ) -> Result<Json<BaseResponseModel<UserModel>>, Status> {
        let user_detail = self.0.find_user(&user_info);
        match user_detail {
            Ok(user) => Ok(Json(BaseResponseModel {
                status: status_code::SUCCESS,
                time_stamp: helper::get_current_time(),
                data: Some(user),
                message: None,
            })),
            Err(e) => match e {
                mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                    let mut response: BaseResponseModel<UserModel> = BaseResponseModel {
                        status: status_code::INTERNAL_SERVER_ERROR,
                        time_stamp: helper::get_current_time(),
                        message: Some(message.clone()),
                        ..Default::default()
                    };

                    if message == constants::NOT_FOUND {
                        response.status = status_code::NOT_FOUND;
                    }

                    Ok(response.self_response())
                }
                _ => Err(Status::InternalServerError),
            },
        }
    }
}
