use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::utils::{constants, helper::get_current_time, status_code};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct BaseResponseModel<T> {
    pub status: i32,
    pub time_stamp: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> BaseResponseModel<T> {
    pub fn self_response(self) -> Json<BaseResponseModel<T>> {
        return Json(BaseResponseModel {
            status: self.status,
            time_stamp: get_current_time(),
            message: self.message,
            data: None,
        });
    }

    pub fn success(data: Option<T>) -> Json<BaseResponseModel<T>> {
        return Json(BaseResponseModel {
            status: status_code::SUCCESS,
            time_stamp: get_current_time(),
            data: data,
            message: None,
        });
    }

    pub fn expired_token(custom_message: Option<String>) -> Json<BaseResponseModel<T>> {
        return Json(BaseResponseModel {
            status: status_code::EXPIRED_TOKEN,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::EXPIRED_TOKEN.to_string())
            } else {
                custom_message
            },
            data: None,
        });
    }

    pub fn invalid_token(custom_message: Option<String>) -> Json<BaseResponseModel<T>> {
        return Json(BaseResponseModel {
            status: status_code::INVALID_TOKEN,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::INVALID_TOKEN.to_string())
            } else {
                custom_message
            },
            data: None,
        });
    }

    pub fn internal_error(custom_message: Option<String>) -> Json<BaseResponseModel<T>> {
        return Json(BaseResponseModel {
            status: status_code::INTERNAL_SERVER_ERROR,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::SOME_THING_WENT_WRONG.to_string())
            } else {
                custom_message
            },
            data: None,
        });
    }

    pub fn not_found(custom_message: Option<String>) -> Json<BaseResponseModel<T>> {
        return Json(BaseResponseModel {
            status: status_code::NOT_FOUND,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::NOT_FOUND.to_string())
            } else {
                custom_message
            },
            data: None,
        });
    }

    pub fn bad_request(custom_message: Option<String>) -> Json<BaseResponseModel<T>> {
        return Json(BaseResponseModel {
            status: status_code::BAD_REQUEST,
            time_stamp: get_current_time(),
            message: if custom_message.is_none() {
                Some(constants::BAD_REQUEST.to_string())
            } else {
                custom_message
            },
            data: None,
        });
    }
}
