use actix_web::{get, HttpResponse};

#[get("/healthz")]
async fn health_controller() -> HttpResponse {
    println!("Petición healthz");
    HttpResponse::Ok().body("Up")
}
