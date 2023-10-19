use crate::{
    models::{base_response_model::BaseResponseModel, user_model::UserModel},
    repositories::mongo_repository::MongoRepo,
    utils::{
        constants::{self},
        helper::{self, create_jwt},
        status_code, UserResponseResult,
    },
};
use actix_web::error::ParseError::Status;
use actix_web::web::Json;
use image::io::Reader as ImageReader;
use mongodb::bson::oid::ObjectId;
use std::{
    env, fs,
    io::Cursor,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone)]
pub struct UserService {
    db: MongoRepo,
}

impl UserService {
    pub fn new(db: MongoRepo) -> Self {
        Self { db }
    }

    pub fn validate_email(&self, email: &Option<String>) -> Result<(), String> {
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

    pub fn validate_password(&self, password: &Option<String>) -> Result<(), String> {
        if password.is_none() {
            return Err(constants::PASSWORD_EMPTY.to_string());
        }

        if password.to_owned().unwrap().is_empty() {
            return Err(constants::PASSWORD_EMPTY.to_string());
        }

        return Ok(());
    }

    pub fn register_user(&self, new_user: &UserModel) -> UserResponseResult {
        println!("{:?}", new_user);

        match self.db.find_user(new_user) {
            Ok(user_exit) => {
                println!("Exits : {:?}", user_exit);

                let response = UserModel::internal_error(Some(constants::EMAIL_EXITS.to_string()));
                return Err(response);
            }
            Err(_) => {
                let mut new_doc = new_user.clone();
                new_doc.password = Some(helper::encryption(new_user.password.to_owned().unwrap()));

                let insert_result = self.db.user_col.insert_one(&new_doc, None);
                match insert_result {
                    Ok(user_inserted) => {
                        println!("Create : {:?}", user_inserted);
                        let mut created_user: UserModel = new_user.clone();
                        created_user.id = user_inserted.inserted_id.as_object_id();

                        return Ok(created_user);
                    }
                    Err(error) => {
                        let response = UserModel::internal_error(Some(error.to_string()));
                        return Err(response);
                    }
                }
            }
        }
    }

    pub fn login(&self, user: &UserModel) -> UserResponseResult {
        return self.db.find_user(user);
    }

    pub fn renew_token(&self, user_id: ObjectId) -> UserResponseResult {
        let query_user = &UserModel {
            id: Some(user_id),
            ..Default::default()
        };
        match self.db.find_user(query_user) {
            Ok(user_match) => {
                let mut update_user = query_user.clone();
                update_user.token = Some(
                    create_jwt(user_match.id.expect("Not found user_id"))
                        .expect("Cannot gen new token"),
                );

                return self.db.update_user(query_user, &update_user);
            }
            Err(e) => return Err(e),
        }
    }

    // pub fn find_in_db(&self, user_info: UserModel) -> BaseResponseModel<UserModel> {
    //     let user_detail = self.db.find_user(&user_info);

    //     match user_detail {
    //         Ok(user) => BaseResponseModel::success(Some(user)),
    //         Err(e) => e,
    //     }
    // }

    // pub fn update_to_db(
    //     &self,
    //     id: ObjectId,
    //     user_info: UserModel,
    // ) -> Result<BaseResponseModel<UserModel>, ()> {
    //     if user_info.email.is_some() {
    //         let email_validated = self.validate_email(&user_info.email);
    //         if email_validated.is_err() {
    //             return Ok(BaseResponseModel::bad_request(email_validated.err()));
    //         }
    //     }

    //     if user_info.password.is_some() {
    //         let password_validated = self.validate_password(&user_info.password);
    //         if password_validated.is_err() {
    //             return Ok(BaseResponseModel::bad_request(password_validated.err()));
    //         }
    //     }

    //     let query_user = &UserModel {
    //         id: Some(id),
    //         ..Default::default()
    //     };

    //     let mut update_user = user_info.clone();

    //     if user_info.image.is_none() {
    //         let user_update = self.db.update_user(query_user, &update_user);
    //         if user_update.is_err() {
    //             return Ok(BaseResponseModel::internal_error(Some(
    //                 "Cannot Update".to_string(),
    //             )));
    //         }
    //         return Ok(BaseResponseModel::success(Some(user_update.unwrap())));
    //     }

    //     let dynamic_image =
    //         match match ImageReader::new(Cursor::new(user_info.to_owned().image.unwrap()))
    //             .with_guessed_format()
    //         {
    //             Ok(it) => it,
    //             Err(_) => {
    //                 return Err(());
    //             }
    //         }
    //         .decode()
    //         {
    //             Ok(it) => it,
    //             Err(_) => {
    //                 return Err(());
    //             }
    //         };

    //     let start = SystemTime::now();
    //     let since_the_epoch: u128 = start.duration_since(UNIX_EPOCH).ok().unwrap().as_millis();
    //     if user_info.image_path.is_some() {
    //         if fs::remove_file(user_info.image_path.clone().unwrap()).is_err() {
    //             println!("cannot remove file")
    //         }
    //     }

    //     let user_avatar_path = env::var(constants::ASSET_USER_FOLDER).unwrap()
    //         + "/"
    //         + &id.to_string()
    //         + &since_the_epoch.to_string()
    //         + ".png";

    //     if let Err(_) = dynamic_image.save(user_avatar_path.clone()) {
    //         return Err(());
    //     } else {
    //         update_user.image = None;
    //         update_user.image_path = Some(user_avatar_path.clone());

    //         let update = self.db.update_user(query_user, &update_user);

    //         if update.is_err() {
    //             return Ok(BaseResponseModel::internal_error(Some(
    //                 "Cannot Update".to_string(),
    //             )));
    //         }

    //         let mut user_update_with_image = update.clone().unwrap().clone();
    //         user_update_with_image.image = None;
    //         user_update_with_image.image_path = Some(user_avatar_path.clone());

    //         Ok(BaseResponseModel::success(Some(user_update_with_image)))
    //     }
    // }
}
