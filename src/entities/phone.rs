#[derive(Decodable, Encodable)]
pub struct Phone {
    pub id: String,
    pub verified: bool,
    pub primary: bool,
    pub e164_masked: String,
    pub national_masked: String,
    pub international_masked: String,
}
