mod controllers;
use actix_web::{App, HttpServer};
use controllers::api_hack::api_hack_controller;
use controllers::health::health_controller;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("Servidor Rust ejecutando");
    HttpServer::new(move || {
        App::new()
            .service(health_controller)
            .service(api_hack_controller)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
