use super::phone;

#[derive(RustcDecodable, RustcEncodable)]
pub struct User {
    pub username: String,
    pub email: String,
    pub firstName: String,
    pub lastName: String,
    pub country: String,
    pub state: String,
    pub currencies: Vec<String>,
    pub phones: Vec<phone::Phone>,
}
