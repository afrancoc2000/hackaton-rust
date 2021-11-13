use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

#[path = "../services/api_hack.rs"]
mod api_hack;

#[derive(Deserialize)]
struct Info {
    number: String,
}

#[get("/")]
async fn api_hack_controller(info: web::Query<Info>) -> HttpResponse {
    let number = &info.number;
    let mut result = api_hack::api_hack_service(number.to_string());
    while result.is_err() {
        result = api_hack::api_hack_service(number.to_string());
    }
    match result {
        Ok(js) => HttpResponse::Ok()
            .content_type("application/json")
            .body(js.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
