use actix_web::{get, HttpRequest, HttpResponse, web};
use serde_json;

#[path = "../services/api_hack.rs"]
mod api_hack;

#[get("/{number}")]
async fn api_hack_controller(web::Path(number): web::Path<String>) -> HttpResponse {
    //let number = request.match_info().query("number");

    println!("number:{}", number);

    match api_hack::api_hack_service(&number) {
        Ok(js) => HttpResponse::Ok().body(serde_json::to_string(&js).unwrap()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
