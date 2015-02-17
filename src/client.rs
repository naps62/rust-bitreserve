#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bearer(pub String);

impl_header!(Bearer, "Bearer", String)


pub struct Client {
    bearer: Bearer,
    hyper: hyper::Client,
}

impl Client {
    fn new(&self, token: String) -> Client {
        Client { Bearer(token), client: hyper::Client::new() }
    }

    fn get(&self, path: String) -> hyper::Response {
        self.hyper.get(path).send().unwrap()
    }
}
