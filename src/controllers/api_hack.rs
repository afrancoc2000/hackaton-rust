use actix_web::{get, HttpResponse, web};
use serde_json;
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
    match api_hack::api_hack_service(&number) {
        Ok(js) => HttpResponse::Ok().body(serde_json::to_string(&js).unwrap()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
