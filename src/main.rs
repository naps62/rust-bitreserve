extern crate "rustc-serialize" as rustc_serialize;
#[macro_use] extern crate hyper;
pub mod entities;
mod client;

use std::old_io;


fn main() {
    let username = prompt(&"Bitreserve email: ");
    let password = read_env(&"BITRESERVE_PASSWORD");

    println!("Hello {}", username);
    println!("Password: <secret>");

    println!("Getting a token");
    client::trigger_sms(username.clone(), password.clone());

    let otp = prompt("You should have received an SMS. What's the Token? ");

    client::request_token(username, password, otp);
    // let mut client = Client::new();

    // let mut res = client.get("https://api.bitreserve.org/v0/ticker").send().unwrap();

    // let decoded: Vec<Ticker> = json::decode(res.read_to_string().unwrap().as_slice()).unwrap();

    // println!("{}", decoded[1].bid)
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    let mut stdin = old_io::stdin();

    let io_line: old_io::IoResult<String> = stdin.read_line();
    let line: String = io_line.unwrap();

    let trimmed_line: &str = line.as_slice().trim();
    trimmed_line.to_string()
}

fn read_env(key: &str) -> String {
    match std::os::getenv(key) {
        Some(val) => val,
        None      => panic!("Please define a {} env variable with your Bitreserve password", key),
    }
}
