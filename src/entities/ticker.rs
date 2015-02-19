#[derive(RustcDecodable, RustcEncodable)]
pub struct Ticker {
    pub ask: f64,
    pub bid: f64,
    pub currency: String,
    pub pair: String,
}
