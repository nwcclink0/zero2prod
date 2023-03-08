//! lib.rs
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;
pub mod routes;
pub mod configuration;
pub mod startup;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    }).listen(listener)?.run();
    Ok(server)
}
