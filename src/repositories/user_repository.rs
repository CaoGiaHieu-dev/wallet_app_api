extern crate dotenv;
use super::mongo_repository::MongoRepo;
use crate::models::user_model::UserModel;
use crate::utils::helper;
use crate::utils::string;
use mongodb::bson::doc;
use mongodb::bson::to_document;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult};

impl MongoRepo {
    pub fn register_user(&self, user: &UserModel) -> Result<InsertOneResult, Error> {
        let new_doc = UserModel {
            id: None,
            email: user.email.to_owned(),
            password: Some(helper::encryption(user.password.to_owned().unwrap())),
            display_name: user.display_name.to_owned(),
        };

        let exits_user = self.user_col.find(doc! {"email": &new_doc.email}, None);
        match exits_user {
            Ok(_) => {
                return Err(Error::DeserializationError {
                    message: string::EMAIL_EXITS.to_string(),
                });
            }
            Err(_) => {
                let insert_result = self.user_col.insert_one(new_doc, None);
                match insert_result {
                    Ok(user) => Ok(user),
                    Err(error) => {
                        return Err(Error::DeserializationError {
                            message: error.to_string(),
                        });
                    }
                }
            }
        }
    }

    pub fn find_user(&self, user: &UserModel) -> Result<UserModel, Error> {
        let new_doc = UserModel {
            id: if user.id.is_none() {
                None
            } else {
                user.id.to_owned()
            },
            email: if user.email.is_none() {
                None
            } else {
                user.email.to_owned()
            },
            password: if user.password.is_none() {
                None
            } else {
                Some(helper::encryption(user.password.to_owned().unwrap()))
            },

            display_name: if user.display_name.is_none() {
                None
            } else {
                user.display_name.to_owned()
            },
        };
        let filter = to_document(&new_doc);
        match filter {
            Ok(doc) => {
                let find_result = self.user_col.find_one(doc, None);
                match find_result {
                    Ok(user) => match user {
                        Some(result) => {
                            let mut user_sponse = result.clone();
                            user_sponse.password =
                                Some(helper::decryption(user_sponse.password.unwrap().to_owned()));

                            Ok(user_sponse)
                        }
                        None => {
                            return Err(Error::DeserializationError {
                                message: string::NOT_FOUND.to_string(),
                            });
                        }
                    },
                    Err(error) => {
                        return Err(Error::DeserializationError {
                            message: error.to_string(),
                        });
                    }
                }
            }
            Err(error) => {
                return Err(Error::DeserializationError {
                    message: error.to_string(),
                });
            }
        }
    }
}
