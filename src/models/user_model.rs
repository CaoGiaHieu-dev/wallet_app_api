use actix_web::{body::MessageBody, http::Error};
use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

use crate::utils::{constants, helper::get_current_time, status_code};

use super::base_response_model::BaseResponseModel;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl UserModel {
    pub fn success(&self) -> BaseResponseModel<UserModel> {
        return BaseResponseModel {
            status: status_code::SUCCESS,
            time_stamp: get_current_time(),
            data: Some(self.clone()),
            message: None,
        };
    }

    pub fn expired_token(custom_message: Option<String>) -> BaseResponseModel<UserModel> {
        return BaseResponseModel {
            status: status_code::EXPIRED_TOKEN,
            time_stamp: get_current_time(),
            data: None,
            message: if custom_message.is_none() {
                Some(constants::EXPIRED_TOKEN.to_string())
            } else {
                custom_message
            },
        };
    }

    pub fn invalid_token(custom_message: Option<String>) -> BaseResponseModel<UserModel> {
        return BaseResponseModel {
            status: status_code::INVALID_TOKEN,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::INVALID_TOKEN.to_string())
            } else {
                custom_message
            },
            data: None,
        };
    }

    pub fn internal_error(custom_message: Option<String>) -> BaseResponseModel<UserModel> {
        return BaseResponseModel {
            status: status_code::INTERNAL_SERVER_ERROR,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::SOME_THING_WENT_WRONG.to_string())
            } else {
                custom_message
            },
            data: None,
        };
    }

    pub fn not_found(custom_message: Option<String>) -> BaseResponseModel<UserModel> {
        return BaseResponseModel {
            status: status_code::NOT_FOUND,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::NOT_FOUND.to_string())
            } else {
                custom_message
            },
            data: None,
        };
    }

    pub fn bad_request(custom_message: Option<String>) -> BaseResponseModel<UserModel> {
        return BaseResponseModel {
            status: status_code::BAD_REQUEST,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::BAD_REQUEST.to_string())
            } else {
                custom_message
            },
            data: None,
        };
    }
}
