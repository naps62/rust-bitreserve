#[derive(RustcDecodable, RustcEncodable)]
pub struct Card {
    pub id: String,
    pub label: String,
    pub currency: String,
    pub balance: f64,
    pub available: f64,
}
