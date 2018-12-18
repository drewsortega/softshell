#![feature(uniform_paths)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(type_ascription)]

extern crate dotenv;
extern crate dotenv_codegen;
extern crate hyper;
extern crate hyper_tls;
extern crate iron;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

extern crate futures;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_io;

use iron::prelude::*;
use iron::status::Status;

mod parser;
mod twitter;

use dotenv::dotenv;
use iron::mime;
use parser::parse_twitter;
use serde_json::json;
use std::env;
fn handler(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<mime::Mime>().unwrap();
    let path = req.url.path();
    if path.len() == 2 && path[0] == "api" && path[1] == "twitter" {
        if req.url.query().is_none() == false {
            let query = req.url.query().unwrap().to_owned();
            let query_iter = query.split("&");
            for q in query_iter {
                let (key, value) = q.split_at(4);
                if key == "url=" {
                    return parse_twitter(value.to_string());
                }
            }
        }
        Ok(Response::with((
            content_type,
            Status::Ok,
            json!({
            "validURL": false,
            "data": null
            })
            .to_string(),
        )))
    } else {
        Ok(Response::with((content_type, Status::NotFound)))
    }
}

fn main() {
    dotenv().ok();
    //get_tweets("realdonaldtrump".parse().unwrap());
    let args: Vec<String> = env::args().collect();
    Iron::new(handler).http("localhost:8000");
}
