use image::{io::Reader as ImageReader, ImageFormat};
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};
use std::{env, io::Cursor};

use crate::{
    models::{base_response_model::BaseResponseModel, user_model::UserModel},
    repositories::mongo_repository::MongoRepo,
    utils::{
        constants::{self},
        helper, status_code,
    },
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

        if !helper::validate_email(email.to_owned().unwrap().to_owned()) {
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
            Ok(user) => Ok(BaseResponseModel::success(Some(user))),
            Err(e) => match e {
                mongodb::bson::extjson::de::Error::DeserializationError { message } => {
                    let mut response: Json<BaseResponseModel<UserModel>> =
                        BaseResponseModel::internal_error(Some(message.clone()));

                    if message == constants::NOT_FOUND {
                        response.status = status_code::NOT_FOUND;
                    }

                    return Ok(response.0.self_response());
                }
                _ => Err(Status::InternalServerError),
            },
        }
    }

    pub fn update_to_db(
        &self,
        id: ObjectId,
        user_info: UserModel,
    ) -> Result<Json<BaseResponseModel<UserModel>>, Status> {
        if user_info.email.is_some() {
            let email_validated = UserService::<'a>::validate_email(&user_info.email);
            if email_validated.is_err() {
                return Ok(BaseResponseModel::bad_request(email_validated.err()));
            }
        }

        if user_info.email.is_some() {
            let password_validated = UserService::<'a>::validate_password(&user_info.password);
            if password_validated.is_err() {
                return Ok(BaseResponseModel::bad_request(password_validated.err()));
            }
        }

        let user_update = self.0.update_user(
            &UserModel {
                id: Some(id),
                ..Default::default()
            },
            &user_info,
        );

        if user_update.is_err() {
            return Ok(BaseResponseModel::internal_error(Some(
                "Cannot Update".to_string(),
            )));
        }

        if user_info.image.is_none() {
            return Ok(BaseResponseModel::success(Some(user_update.unwrap())));
        }

        let dynamic_image = match match ImageReader::new(Cursor::new(user_info.image.unwrap()))
            .with_guessed_format()
        {
            Ok(it) => it,
            Err(err) => {
                println!("{:?}", err);
                return Err(Status::InternalServerError);
            }
        }
        .decode()
        {
            Ok(it) => it,
            Err(err) => {
                println!("{:?}", err);
                return Err(Status::InternalServerError);
            }
        };

        match env::var(constants::ASSET_USER_FOLDER) {
            Ok(path) => match dynamic_image.save_with_format(path, ImageFormat::Png) {
                Ok(_) => Ok(BaseResponseModel::success(Some(user_update.unwrap()))),
                Err(save_error) => {
                    println!("{:?}", save_error);
                    return Err(Status::InternalServerError);
                }
            },
            Err(err) => {
                println!("{:?}", err);
                return Err(Status::InternalServerError);
            }
        }
    }
}
