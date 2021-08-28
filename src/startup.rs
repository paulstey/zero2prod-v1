//! src/startup.rs
use crate::routes::{health_check::health_check, subscriptions::subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net::TcpListener;



pub fn run(
    listener: TcpListener,
    connection: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the `wrap` methond on `App`
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(db_pool.clone())

    })
    .listen(listener)?
    .run();

    Ok(server)
}