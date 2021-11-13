use reqwest;

#[path = "../models/response.rs"]
mod response;

pub fn api_hack_service(number: &str) -> Result<response::Response, reqwest::Error> {
    let resp = reqwest::blocking::get(format!("http://api-3.hack.local/?number={}", number))?
        .json();

    match resp {
        Ok(js) => Ok(js),
        Err(e) => Err(e)
    }
}
