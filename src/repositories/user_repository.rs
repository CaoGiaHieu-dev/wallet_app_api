extern crate dotenv;
use super::mongo_repository::MongoRepo;
use crate::models::user_model::UserModel;
use crate::utils::constants;
use crate::utils::helper;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::bson::to_document;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult};

impl MongoRepo {
    pub fn register_user(&self, user: &UserModel) -> Result<InsertOneResult, Error> {
        let mut new_doc = user.clone();
        new_doc.password = Some(helper::encryption(user.password.to_owned().unwrap()));

        let exits_user = self.user_col.find_one(doc! {"email": &new_doc.email}, None);

        match exits_user {
            Ok(user) => match user {
                Some(_) => {
                    return Err(Error::DeserializationError {
                        message: constants::EMAIL_EXITS.to_string(),
                    });
                }
                None => {
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
            },
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
        let mut user_doc = user.clone();
        if user_doc.password.is_some() {
            user_doc.password = Some(helper::encryption(user_doc.password.unwrap().to_owned()));
        }

        match to_document(&user_doc) {
            Ok(doc) => {
                let find_result = self.user_col.find_one(doc, None);
                match find_result {
                    Ok(user) => match user {
                        Some(result) => {
                            let mut user_response = result.clone();
                            user_response.password = Some(helper::decryption(
                                user_response.password.unwrap().to_owned(),
                            ));
                            user_response.token = None;

                            Ok(user_response)
                        }
                        None => {
                            return Err(Error::DeserializationError {
                                message: constants::NOT_FOUND.to_string(),
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

    pub fn update_user(&self, query: &UserModel, update: &UserModel) -> Result<UserModel, Error> {
        let mut user_query = query.clone();
        if user_query.password.is_some() {
            user_query.password = Some(helper::encryption(user_query.password.unwrap().to_owned()));
        }

        let mut user_update = update.clone();
        if user_update.password.is_some() {
            user_update.password =
                Some(helper::encryption(user_update.password.unwrap().to_owned()));
        }

        let query_doc = to_document(&user_query).expect("Cannot convert query to doc");
        let update_doc = to_document(&user_update).expect("Cannot convert update to doc");

        let find_result = self
            .user_col
            .update_one(query_doc, doc! { "$set": update_doc }, None);

        match find_result {
            Ok(user) => {
                if user.matched_count == 0 {
                    return Err(Error::DeserializationError {
                        message: constants::NOT_FOUND.to_string(),
                    });
                }

                match user.upserted_id {
                    Some(bson_result) => {
                        let mut user_from_bson: UserModel = bson::from_bson(bson_result).unwrap();

                        user_from_bson.password = Some(helper::decryption(
                            user_from_bson.password.unwrap().to_owned(),
                        ));

                        return Ok(user_from_bson);
                    }
                    None => return Ok(update.clone()),
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
