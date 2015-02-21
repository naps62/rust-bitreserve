#[derive(RustcDecodable, RustcEncodable)]
pub struct Transaction {
    pub id: String,
    pub status: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Denomination {
    pub currency: String,
    pub amount: f64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TransactionRequest {
    pub destination: String,
    pub denomination: Denomination
}

impl TransactionRequest {
   pub fn new(amount: f64, currency: String, destination: String) -> TransactionRequest {
        TransactionRequest {
            destination: destination,
            denomination: Denomination {
                currency: currency,
                amount: amount,
            }
        }
    }
}
