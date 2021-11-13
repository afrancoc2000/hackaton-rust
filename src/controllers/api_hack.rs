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
    let result = api_hack::api_hack_service(number.to_string());
    match result {
        Ok(js) => HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&js).unwrap()),
        Err(e) => match api_hack::api_hack_service(number.to_string()) { 
            Ok(js) => HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&js).unwrap()),
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}
