use actix_web::web::{self, Json};
use anyhow::Error;
use hyper::{Client, Uri};
use reqwest;
use std::collections::HashMap;

#[path = "../models/response.rs"]
mod response;

pub async fn api_hack_service(number: &str) -> Result<HashMap<String, String>, Error> {
    let resp = reqwest::blocking::get(format!("http://api-3.hack.local/?number={}", number))?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok((resp))
}
