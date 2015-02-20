#![feature(core, io)]
#![allow(dead_code)]

extern crate "rustc-serialize" as rustc_serialize;
#[macro_use] extern crate hyper;

pub mod client;
pub mod entities;
