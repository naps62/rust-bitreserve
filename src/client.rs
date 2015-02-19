#[derive(Copy, Clone, PartialEq, Debug)]
struct Bearer(pub String);

use hyper::header;
use hyper::header::HeaderFormat;
impl_header!(Bearer, "Bearer", String);
use hyper::header::common::authorization::{ Authorization, Basic };
use hyper;
use std::net;
use hyper::client::response::Response;


pub struct Client {
    api_root: str,
    bearer: Bearer,
    hyper: hyper::client::Client<net::HttpConnector>,
}

impl Client {
    fn get_token(username: String, password: String) -> String {
        let basic_auth_header = Authorization(Basic {
            username: username,
            password: Some(password)
        });

        let response = hyper::Client::new().get("https://api.bitreserve.org/v0/me/tokens").header(basic_auth_header).send().unwrap();

        println!("{}", response.read_to_string());
        "asd".to_string()
    }

    fn new(token: String) -> Client {
        Client {
            api_root: "https://api.bitreserve.org/v0/",
            bearer: Bearer(token),
            client: hyper::client::Client::new(),
        }
    }

    fn get(&self, path: String) -> Response {
        self.hyper.get(path).send().unwrap()
    }
}
