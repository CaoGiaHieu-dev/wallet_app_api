#[macro_use]
extern crate rocket;

#[macro_use]
extern crate magic_crypt;

//add imports below
use end_point::user_end_point;
use repositories::mongo_repository::MongoRepo;
use utils::routes;

mod end_point;
mod models;
mod repositories;
mod utils;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db).mount(
        routes::USER,
        routes![user_end_point::create, user_end_point::get],
    )
}
