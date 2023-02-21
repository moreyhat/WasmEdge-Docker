use serde::{Deserialize, Serialize};
use serde_json;
use wasmedge_http_req::request;

#[derive(Serialize, Deserialize)]
struct MyIpAddress {
    origin: String,
}

fn main() {
    let mut writer = Vec::new();
    request::get("http://httpbin.org/ip", &mut writer).unwrap();
    let body = String::from_utf8(writer).unwrap();
    let my_ip: MyIpAddress = serde_json::from_str(body.as_str()).unwrap();
    println!("Your global IP address: {}", my_ip.origin);
}
