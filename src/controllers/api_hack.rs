use actix_web::{get, HttpRequest, HttpResponse};

#[path = "../services/api_hack.rs"]
mod api_hack;

#[get("/")]
async fn api_hack_controller(request: HttpRequest) -> HttpResponse {
    request.match_info().get("number").map_or(
        HttpResponse::BadRequest().body("Missing parameter"),
        |number| match api_hack::api_hack_service(number) {
            Ok(json) => HttpResponse::Ok().body(json),
            _ => HttpResponse::InternalServerError().finish(),
        },
    )

    //TODO
}
