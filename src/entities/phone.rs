#[derive(RustcDecodable, RustcEncodable)]
pub struct Phone {
    pub id: String,
    pub verified: bool,
    pub primary: bool,
    pub e164Masked: String,
    pub nationalMasked: String,
    pub internationalMasked: String,
}
