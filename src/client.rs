#[derive(Clone, PartialEq, Debug)]
struct Bearer(pub String);

impl_header!(Bearer, "Bearer", String);

use hyper;
use hyper::header;
use hyper::net;
use hyper::client::Response;


pub struct Client {
    api_root: String,
    bearer: Bearer,
    hyper: hyper::Client<net::HttpConnector<'static>>,
}

impl Client {
    pub fn get_token(username: String, password: String) -> String {
        let basic_auth_header = header::Authorization(header::Basic {
            username: username,
            password: Some(password)
        });

        let mut response = hyper::Client::new().get("https://api.bitreserve.org/v0/me/tokens").header(basic_auth_header).send().unwrap();

        println!("{}", response.read_to_string().unwrap());
        "asd".to_string()
    }

    fn new(token: String) -> Client {
        Client {
            api_root: "https://api.bitreserve.org/v0/".to_string(),
            bearer: Bearer(token),
            hyper: hyper::Client::new(),
        }
    }

    fn get(&mut self, path: String) -> Response {
        self.hyper.get(path.as_slice()).send().unwrap()
    }
}
