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

impl TransactionRequest {
    pub new(card_id: String, amount: f64, currency: String, destination: String) -> Result<entities::Transaction, json::DecoderError> {
        TransactionRequest {
            card_id: card_id,
            denomination: Denomination {
                currency: currency,
                amount: amount,
            }
        }
}
