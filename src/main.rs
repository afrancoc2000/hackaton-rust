mod controllers;
use actix_web::{App, HttpServer};
use controllers::api_hack::api_hack_controller;
use controllers::health::health_controller;
use std::env;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("Servidor Rust ejecutando en 5000");
    let hostname = env::var("SVC_API_HOSTNAME")
        .expect("No puede leer la variable de entorno SVC_API_HOSTNAME");
    let port = env::var("SVC_API_PORT").expect("No puede leer la variable de entorno SVC_API_PORT");
    println!("Consumo a la API por {}:{}", hostname, port);
    HttpServer::new(move || {
        App::new()
            .service(health_controller)
            .service(api_hack_controller)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
