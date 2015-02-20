#[derive(RustcDecodable, RustcEncodable)]
pub struct AuthToken {
    pub access_token: String,
    pub description: String,
}
