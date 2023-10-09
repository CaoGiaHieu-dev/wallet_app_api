use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::{models::user_model::UserModel, utils::constants};

use mongodb::sync::{Client, Collection};

#[derive(Clone)]
pub struct MongoRepo {
    pub user_col: Collection<UserModel>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var(constants::MONGO_DB_URI) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(constants::DB_NAME);

        let user_col: Collection<UserModel> = db.collection(constants::USER_COL);
        MongoRepo { user_col }
    }
}
