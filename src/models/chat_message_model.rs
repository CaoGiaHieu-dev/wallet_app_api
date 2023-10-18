use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ChatMessageModel {
    pub room: i64,
    pub sender_id: String,
    pub message: String,
}
