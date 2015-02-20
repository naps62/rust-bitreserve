#[derive(Clone, PartialEq, Debug)]
struct Bearer(pub String);
impl_header!(Bearer, "Bearer", String);

#[derive(Clone, PartialEq, Debug)]
struct XBitreserveOTP(pub String);
impl_header!(XBitreserveOTP, "X-Bitreserve-OTP", String);

use hyper;
use hyper::header;
use hyper::net;
use hyper::client::Response;

use hyper::mime;
use hyper::mime::Mime;

use std::collections::HashMap;

use rustc_serialize::{ json, Decodable };

use entities;

pub struct Client {
    api_root: String,
    auth_header: header::Authorization<String>,
    hyper: hyper::Client<net::HttpConnector<'static>>,
}

impl Client {
    pub fn new(token: String) -> Client {
        Client {
            api_root: "https://api.bitreserve.org/v0/".to_string(),
            auth_header: header::Authorization(format!("Bearer {}", token)),
            hyper: hyper::Client::new(),
        }
    }

    pub fn me(&mut self) -> Result<entities::User, json::DecoderError> {
        self.get("https://api.bitreserve.org/v0/me".to_string())
    }

    fn get<T: Decodable>(&mut self, path: String) -> Result<T, json::DecoderError> {
        let mut response = self.hyper
            .get(path.as_slice())
            .header(self.auth_header.clone())
            .send()
            .unwrap();

        let str_response = response.read_to_string().unwrap();
        println!("{}", str_response.clone());

        json::decode(str_response.as_slice())
    }
}


pub fn trigger_sms(username: String, password: String) {
    let basic_auth = header::Authorization(header::Basic {
        username: username,
        password: Some(password)
    });

    hyper::Client::new()
        .get("https://api.bitreserve.org/v0/me")
        .header(basic_auth)
        .send()
        .unwrap();
}

pub fn request_token(username: String, password: String, otp: String) -> String {
    let basic_auth = header::Authorization(header::Basic {
        username: username,
        password: Some(password)
    });

    let mime_json = Mime(
        mime::TopLevel::Application,
        mime::SubLevel::Json, vec![]
    );
    let content_type = header::ContentType(mime_json);

    let x_otp = XBitreserveOTP(otp);

    let mut payload = HashMap::new();
    payload.insert("description", "A Rust test");

    let mut response = hyper::Client::new()
        .post("https://api.bitreserve.org/v0/me/tokens")
        .header(basic_auth)
        .header(content_type)
        .header(x_otp)
        .body(json::encode(&payload).unwrap().as_slice())
        .send()
        .unwrap();

    let js = response.read_to_string().unwrap();

    json::decode::<entities::AuthToken>(js.as_slice()).unwrap().access_token
}
