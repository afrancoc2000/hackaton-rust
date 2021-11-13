use actix_web::{get, HttpResponse};

#[get("/healthz")]
async fn health_controller() -> HttpResponse {
    println!("Solicitud al healthz");
    HttpResponse::Ok().body("Up")
}
