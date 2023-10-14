#[macro_use]
extern crate rocket;

#[macro_use]
extern crate magic_crypt;

use end_point::user_end_point;
use repositories::mongo_repository::MongoRepo;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

use utils::routes;

use rocket::{http::Method, routes};

mod end_point;
mod middleware;
mod models;
mod repositories;
mod service;
mod utils;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::All)
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .manage(db)
        .manage(cors.to_cors())
        .mount(
            routes::AUTH,
            routes![
                user_end_point::register,
                user_end_point::find_user,
                user_end_point::login,
                user_end_point::update_user,
                user_end_point::info,
            ],
        )
        .mount(routes::CHAT, routes![])
}
