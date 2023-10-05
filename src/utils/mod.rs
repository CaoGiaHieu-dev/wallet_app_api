pub mod constants;
pub mod helper;
pub mod routes;
pub mod status_code;

use crate::models::base_response_model::BaseResponseModel;
use rocket::serde::json::Json;

pub type ErrorResponse = Json<BaseResponseModel<()>>;
