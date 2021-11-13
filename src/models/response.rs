use serde::Deserialize;
use serde::Serialize;

pub type Hostname = String;
pub type Method = String;
pub type Url = String;
pub type Data = String;
pub type Date = String;
pub type ValiditySeconds = String;
pub type Token = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    hostname: Hostname,
    method: Method,
    url: Url,
    data: Data,
    date: Date,
    validitySeconds: ValiditySeconds,
    token: Token,
}
