use actix_web::{get, HttpResponse};

#[get("/healthz")]
async fn health_controller() -> HttpResponse {
    HttpResponse::Ok().body("Up")
}
