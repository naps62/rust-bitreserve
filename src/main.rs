#![feature(core, env, io)]
#![allow(dead_code)]

extern crate "rustc-serialize" as rustc_serialize;
#[macro_use] extern crate hyper;
mod entities;
mod client;

use std::old_io;
use std::env;
use client::Client;

fn main() {
    let auth_token = match env::var("BITRESERVE_AUTH_TOKEN") {
        Ok(token) => token,
        Err(_)    => request_new_token(),
    };

    println!("Your token is: {}", auth_token);

    let mut bitreserve = Client::new(auth_token);
    let user = bitreserve.me().unwrap();
    println!("{}", user.email);

    // let mut res = client.get("https://api.bitreserve.org/v0/ticker").send().unwrap();

    // let nata: Nata = json::decode("{ \"x\": \"xxx\", \"y\": \"yyy\" }").unwrap();

    // println!("{}", nata.x);
}

fn request_new_token() -> String {
    let username = read_env_or_prompt(&"BITRESERVE_EMAIL", "Bitreserve email: ");
    let password = read_env_or_prompt(&"BITRESERVE_PASSWORD", "Bitreserve password: ");

    println!("Hello, {}", username);
    client::trigger_sms(username.clone(), password.clone());

    let otp = prompt("You should have received an SMS. What's the Token? ");

    client::request_token(username, password, otp)
}

fn read_env_or_prompt(key: &str, fallback_prompt: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_)  => prompt(fallback_prompt),
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    let mut stdin = old_io::stdin();

    let io_line: old_io::IoResult<String> = stdin.read_line();
    let line: String = io_line.unwrap();

    let trimmed_line: &str = line.as_slice().trim();
    trimmed_line.to_string()
}
