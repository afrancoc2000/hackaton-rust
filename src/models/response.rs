use serde::Deserialize;
use serde::Serialize;

pub type Hostname = String;
pub type Method = String;
pub type Url = String;
pub type Data = Option<String>;
pub type Date = String;
pub type ValiditySeconds = u32;
pub type Token = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    hostname: Hostname,
    method: Method,
    url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    data: Data,
    date: Date,
    #[serde(rename = "validitySeconds")]
    validity_seconds: ValiditySeconds,
    token: Token,
}
