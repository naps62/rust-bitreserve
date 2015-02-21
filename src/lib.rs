#![feature(core, io)]
#![allow(dead_code)]

extern crate "rustc-serialize" as rustc_serialize;
#[macro_use] extern crate hyper;
extern crate url;

pub mod client;
pub mod entities;
