use cached::proc_macro::cached;
use reqwest;
use serde_json;

#[path = "../models/response.rs"]
mod response;

#[cached(size = 200, result = true, time = 45)]
pub fn api_hack_service(number: String) -> Result<String, reqwest::Error> {
    let json = get_resp(number.to_string());
    match json {
        Ok(js) => Ok(serde_json::to_string(&js).unwrap()),
        Err(e) => Err(e),
    }
}

fn get_resp(number: String) -> Result<response::Response, reqwest::Error> {
    let resp =
        reqwest::blocking::get(format!("http://api-3.hack.local/?number={}", number))?.json();

    match resp {
        Ok(js) => Ok(js),
        Err(e) => Err(e),
    }
}
