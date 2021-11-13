mod controllers;
use actix_web::{App, HttpServer};
use controllers::health::health_controller;
use controllers::api_hack::api_hack_controller;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(health_controller)
        .service(api_hack_controller
        ))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
