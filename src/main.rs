#[macro_use]
extern crate rocket;

#[macro_use]
extern crate magic_crypt;

use end_point::auth_end_point;
use repositories::mongo_repository::MongoRepo;

use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

use utils::routes;

use rocket::{fs::FileServer, http::Method, routes};

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
        .mount(routes::USER_PATH, FileServer::from("assets/user/"))
        .mount(
            routes::AUTH,
            routes![
                auth_end_point::register,
                auth_end_point::find_user,
                auth_end_point::login,
                auth_end_point::update_user,
                auth_end_point::info,
                auth_end_point::renew_token,
            ],
        )
        .mount(routes::CHAT, routes![])
}
