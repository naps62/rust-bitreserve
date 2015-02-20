#[derive(Clone, PartialEq, Debug)]
struct Bearer(pub String);
impl_header!(Bearer, "Bearer", String);

#[derive(Clone, PartialEq, Debug)]
struct XBitreserveOTP(pub String);
impl_header!(XBitreserveOTP, "X-Bitreserve-OTP", String);

use hyper;
use hyper::header;
use hyper::net;

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
        self.get("me")
    }

    pub fn my_cards(&mut self) -> Result<Vec<entities::Card>, json::DecoderError> {
        self.get("me/cards")
    }

    pub fn create_transaction(&mut self, card_id: String, amount: f64, currency: String, destination: String) -> Result<entities::Transaction, json::DecoderError> {
        let mut payload = HashMap::new();
        payload.insert("demonination", "A Rust test");
        paload
        let mut response = self.hyper
            .post(format!("https://api.bitreserve.org/v0/me/cards/{}/transactions", card_id).as_slice())
            .header(self.auth_header.clone())
            // .body(format!("denomination[currency]={}&denomination[amount]={}&destination={}", amount, currency, destination).as_slice())
            .body(json::encode(&payload).unwrap().as_slice())
            .send()
            .unwrap();

        let str_response = response.read_to_string().unwrap();
        println!("{}", str_response);
        json::decode(str_response.as_slice())
    }

    fn get<T: Decodable>(&mut self, path: &str) -> Result<T, json::DecoderError> {
        let mut response = self.hyper
            .get(format!("{}{}", self.api_root, path).as_slice())
            .header(self.auth_header.clone())
            .send()
            .unwrap();

        let str_response = response.read_to_string().unwrap();

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
