#[macro_use]
extern crate rocket;

#[macro_use]
extern crate magic_crypt;

use std::{net::TcpStream, sync::Arc};

use end_point::auth_end_point;

use repositories::mongo_repository::MongoRepo;

use rocket::{
    fs::FileServer, futures::stream::SplitSink, http::Method, routes,
    tokio::sync::broadcast::channel,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use utils::routes;
use web_socket::SocketEmitEvent;

mod end_point;
mod middleware;
mod models;
mod repositories;
mod service;
mod utils;
pub mod web_socket;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::All)
        .allow_credentials(true);

    rocket::build()
        .manage(db)
        .manage(channel::<SocketEmitEvent>(1024).0)
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
        .mount("/", routes![web_socket::echo_channel,])
}
