#![feature(uniform_paths)]

extern crate dotenv;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate dotenv_codegen;
extern crate hyper_tls;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod twitter;

use dotenv::dotenv;
use std::env;
use std::path::Path;
use twitter::get_tweets;

fn main() {
    dotenv().ok();
    get_tweets("realdonaldtrump".parse().unwrap());
}
