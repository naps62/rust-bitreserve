#[derive(RustcDecodable, RustcEncodable)]
pub struct Transaction {
    pub id: String,
    pub status: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Denomination {
    pub currency: String,
    pub amoun: f64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TransactionRequest {
    pub card_id: String,
    pub denomination: Denomination
}
