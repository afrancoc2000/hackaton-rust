use actix_web::web::Path;
use actix_web::{post, HttpResponse};
use actix_web::{App, HttpServer};
use controllers::health::health_controller;
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(health_controller()))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
