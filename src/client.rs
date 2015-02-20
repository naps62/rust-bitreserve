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


pub struct Client {
    api_root: String,
    bearer: Bearer,
    hyper: hyper::Client<net::HttpConnector<'static>>,
}

impl Client {

    fn new(token: String) -> Client {
        Client {
            api_root: "https://api.bitreserve.org/v0/".to_string(),
            bearer: Bearer(token),
            hyper: hyper::Client::new(),
        }
    }

    // fn get(&mut self, path: String) -> Response {
    //     self.hyper.get(path.as_slice()).send().unwrap()
    // }
}


pub fn trigger_sms(username: String, password: String) {
    let basic_auth = header::Authorization(header::Basic {
        username: username,
        password: Some(password)
    });

    let mut response = hyper::Client::new().get("https://api.bitreserve.org/v0/me").header(basic_auth).send().unwrap();
}

pub fn request_token(username: String, password: String, otp: String) -> String {
    let basic_auth = header::Authorization(header::Basic {
        username: username,
        password: Some(password)
    });

    let mime_json = Mime(mime::TopLevel::Application, mime::SubLevel::Json, vec![]);
    let content_type = header::ContentType(mime_json);

    println!("otp: {}", otp.clone());
    let x_otp = XBitreserveOTP(otp);

    // let params = vec![("description", "A rust test")]
    let mut response = hyper::Client::new().post("https://api.bitreserve.org/v0/me/tokens").header(basic_auth).header(content_type).header(x_otp).send().unwrap();

    println!("Your token: {}", response.read_to_string().unwrap());

    "asd".to_string()
}
