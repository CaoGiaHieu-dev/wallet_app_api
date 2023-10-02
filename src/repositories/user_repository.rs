extern crate dotenv;
use super::mongo_repository::MongoRepo;
use crate::models::user_model::UserModel;
use crate::utils::helper;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
};
impl MongoRepo {
    pub fn create_user(&self, user: &UserModel) -> Result<InsertOneResult, Error> {
        let new_doc = UserModel {
            id: None,
            user_name: user.user_name.to_owned(),
            password: helper::encryption(user.password.to_owned()),
            display_name: user.display_name.to_owned(),
        };

        let insert_result = self.user_col.insert_one(new_doc, None);
        match insert_result {
            Ok(user) => Ok(user),
            Err(error) => {
                println!("{:?}", error);
                return Err(Error::DeserializationError {
                    message: error.to_string(),
                });
            }
        }
    }

    pub fn get_user(&self, id: &String) -> Result<Option<UserModel>, Error> {
        let obj_id = ObjectId::parse_str(id);

        match obj_id {
            Ok(_id) => {
                println!("{:?}", _id);
                let filter = doc! {"_id": _id};

                let user_detail = self.user_col.find_one(filter, None);

                match user_detail {
                    Ok(user) => Ok(user),
                    Err(error) => {
                        return Err(Error::DeserializationError {
                            message: error.to_string(),
                        })
                    }
                }
            }
            Err(n) => {
                println!("{:?}", n);
                return Err(Error::InvalidObjectId(n));
            }
        }
    }
}
