#![feature(core, env, io)]
#![allow(dead_code)]

extern crate "rustc-serialize" as rustc_serialize;
#[macro_use] extern crate hyper;
extern crate url;

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
    let cards = bitreserve.my_cards().unwrap();

    for i in range(0, cards.len()) {
        println!("{} [{}] - {} {}", i, cards[i].label, cards[i].available, cards[i].currency);
    }

    let card_index: usize = prompt("\nChoose a card to transfer from: ").parse().unwrap();
    let amount: f64 = prompt("How much: ").parse().unwrap();
    let dest = prompt("Destination: ");

    // create transaction
    let card = &cards[card_index];

    let request = entities::TransactionRequest::new(amount, card.currency.clone(), dest);
    let transaction = bitreserve.create_transaction(card.id.clone(), request).unwrap();

    println!("{}", transaction.status);
    
    // commit transaction
    
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
