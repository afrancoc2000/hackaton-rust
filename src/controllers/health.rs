use actix_web::HttpResponse;

pub async fn health_controller() -> HttpResponse {
    HttpResponse::Ok().body("Up")
}
