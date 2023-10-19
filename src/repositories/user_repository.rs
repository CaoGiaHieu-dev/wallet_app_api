extern crate dotenv;
use super::mongo_repository::MongoRepo;
use crate::models::base_response_model::BaseResponseModel;
use crate::models::user_model::UserModel;
use crate::utils::constants;
use crate::utils::helper;
use crate::utils::UserResponseResult;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::bson::to_document;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult};

impl MongoRepo {
    pub fn find_user(&self, user: &UserModel) -> UserResponseResult {
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
                            return Err(UserModel::not_found(None));
                        }
                    },
                    Err(error) => {
                        return Err(UserModel::internal_error(Some(error.to_string())));
                    }
                }
            }
            Err(error) => {
                return Err(UserModel::internal_error(Some(error.to_string())));
            }
        }
    }

    pub fn update_user(&self, query: &UserModel, update: &UserModel) -> UserResponseResult {
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
                    return Err(UserModel::not_found(None));
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
            Err(error) => return Err(UserModel::internal_error(Some(error.to_string()))),
        }
    }
}
