use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::UserModel;
use crate::utils::string;

use mongodb::sync::{Client, Collection};

pub struct MongoRepo {
    pub user_col: Collection<UserModel>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var(string::ENV_URI) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(string::DB_NAME);

        let user_col: Collection<UserModel> = db.collection(string::USER_COL);
        MongoRepo { user_col }
    }
}
