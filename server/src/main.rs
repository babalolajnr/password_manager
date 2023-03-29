use std::env;

use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use database::init::init;
use dotenvy::dotenv;
use listenfd::ListenFd;
use log::info;
use sea_orm::DatabaseConnection;

use crate::routes::{auth, password};

mod api_error;
mod database;
mod dto;
mod entities;
pub mod middlewares;
mod routes;
mod services;
mod validators;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

// Start server and file watcher with
// systemfd --no-pid -s http::5000 -- cargo watch -x run
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Connect to database
    let conn = init().await.unwrap();

    let mut listenfd = ListenFd::from_env();
    let state = AppState { conn };

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .service(web::scope("/auth").configure(auth::init_routes))
            .service(web::scope("/password").configure(password::init_routes))
            .service(echo)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST not set");
            let port = env::var("PORT").expect("PORT not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!(
        "Starting server at http://{}:{}",
        server.addrs()[0].ip(),
        server.addrs()[0].port()
    );

    server.run().await
}

#[actix_web::get("/echo")]
async fn echo() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello")
}
