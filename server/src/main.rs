#![feature(uniform_paths)]

extern crate hyper;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate hyper_tls;

mod twitter;
use twitter::init;

fn main() {
    init("realdonaldtrump".parse().unwrap());
}
