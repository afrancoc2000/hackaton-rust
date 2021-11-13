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
    let mut result = api_hack::api_hack_service(&number);
    if result.is_err() { 
        print!("ENTRO");
        // Try again in about 5 seconds ...
        std::thread::sleep(std::time::Duration::from_secs(97));
        // Reattempt
        result = api_hack::api_hack_service(&number);
    }
    match result {
        Ok(js) => HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&js).unwrap()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
