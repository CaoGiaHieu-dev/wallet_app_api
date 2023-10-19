#[macro_use]
extern crate magic_crypt;



use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use end_point::auth_end_point;
use env_logger;

use repositories::mongo_repository::MongoRepo;
use utils::routes;

use crate::service::user_service::UserService;

mod end_point;
mod models;
mod repositories;
mod service;
mod utils;
pub mod web_socket;

#[actix_web::main]

async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mongodb_repo = MongoRepo::init();
    // let database = web::Data::new(mongodb_repo.clone());

    let json_config = web::JsonConfig::default()
        .limit(1024 * 10)
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
        });

    let user_service = web::Data::new(UserService::new(mongodb_repo));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(actix_web_lab::middleware::CatchPanic::default())
            // .app_data(database.clone())
            .app_data(json_config.clone())
            .service(
                web::scope(routes::AUTH)
                    .app_data(user_service.clone())
                    .service(auth_end_point::register)
                    .service(auth_end_point::renew_token)
                    .service(auth_end_point::info)
                    .service(auth_end_point::login),
            )
    })
    .workers(4)
    .bind(("127.0.0.1", 1999))
    .expect("Failed to bind to address")
    .run()
    .await
    .expect("Failed to start")
}

// #[launch]
// fn rocket() -> _ {

//     let cors = CorsOptions::default()
//         .allowed_origins(AllowedOrigins::all())
//         .allowed_headers(AllowedHeaders::All)
//         .allow_credentials(true);

//     rocket::build()
//         .manage(db)
//         .manage(channel::<SocketEmitEvent>(1024).0)
//         .manage(cors.to_cors())
//         .mount(routes::USER_PATH, FileServer::from("assets/user/"))
//         .mount(
//             routes::AUTH,
//             routes![
//                 auth_end_point::register,
//                 auth_end_point::find_user,
//                 auth_end_point::login,
//                 auth_end_point::update_user,
//                 auth_end_point::info,
//                 auth_end_point::renew_token,
//             ],
//         )
//         .mount("/", routes![web_socket::echo_channel, web_socket::events,])
// }
