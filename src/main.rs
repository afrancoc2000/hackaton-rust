mod controllers;
use actix_web::{App, HttpServer};
use controllers::api_hack::api_hack_controller;
use controllers::health::health_controller;
use std::env;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let hostname = env::var("SVC_API_HOSTNAME").unwrap_or("127.0.0.1".to_owned());
    let port = env::var("SVC_API_PORT").unwrap_or("5000".to_owned());
    println!("Server up in {}", port);
    HttpServer::new(move || {
        App::new()
            .service(health_controller)
            .service(api_hack_controller)
    })
    .bind(hostname.to_owned() + ":" + &*port)?
    .run()
    .await
}
