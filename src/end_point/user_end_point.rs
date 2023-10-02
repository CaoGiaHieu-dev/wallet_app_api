use crate::models::base_model::BaseModel;
use crate::models::user_model::UserModel;
use crate::repositories::mongo_repository::MongoRepo;
use crate::utils::helper;
use rocket::{http::Status, serde::json::Json, State};

#[post("/", data = "<new_user>")]
pub fn create(
    db: &State<MongoRepo>,
    new_user: Json<UserModel>,
) -> Result<Json<BaseModel<UserModel>>, Status> {
    let mut data = UserModel {
        id: None,
        user_name: new_user.user_name.to_owned(),
        password: new_user.password.to_owned(),
        display_name: new_user.display_name.to_owned(),
    };
    let user_detail = db.create_user(&data);

    match user_detail {
        Ok(user) => {
            data.id = user.inserted_id.as_object_id();
            let response: BaseModel<UserModel> = BaseModel {
                status: 200,
                time_stamp: helper::get_current_time(),
                data: Some(data),
                message: None,
            };
            Ok(Json(response))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<id>")]
pub fn get(db: &State<MongoRepo>, id: String) -> Result<Json<BaseModel<UserModel>>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let user_detail = db.get_user(&id);

    match user_detail {
        Ok(user) => match user {
            Some(result) => {
                let response: BaseModel<UserModel> = BaseModel {
                    status: 200,
                    time_stamp: helper::get_current_time(),
                    data: Some(result),
                    message: None,
                };
                Ok(Json(response))
            }
            None => {
                let response: BaseModel<UserModel> = BaseModel {
                    status: 404,
                    time_stamp: helper::get_current_time(),
                    data: None,
                    message: Some(String::from("Not Found")),
                };
                Ok(Json(response))
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }

    // }
}
