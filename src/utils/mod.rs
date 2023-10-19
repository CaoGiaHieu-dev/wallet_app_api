pub mod constants;
pub mod helper;
pub mod routes;
pub mod status_code;

use crate::models::{base_response_model::BaseResponseModel, user_model::UserModel};
use actix_web::web::Json;

pub type UserResponseResult = Result<UserModel, BaseResponseModel<UserModel>>;
