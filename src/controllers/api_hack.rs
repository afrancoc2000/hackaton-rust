use actix_web::{get, HttpRequest, HttpResponse};
use serde_json;

#[path = "../services/api_hack.rs"]
mod api_hack;

#[get("/")]
async fn api_hack_controller(request: HttpRequest) -> HttpResponse {
    let number = request.match_info().get("number");
    
    if number.is_none(){
        HttpResponse::InternalServerError().finish()
    }else{

        match api_hack::api_hack_service(number.unwrap()){
            Ok(js) => 
            HttpResponse::Ok().body(serde_json::to_string(&js).unwrap()),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
