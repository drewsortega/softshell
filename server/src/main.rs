#![feature(uniform_paths)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(type_ascription)]

extern crate dotenv;
extern crate dotenv_codegen;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate iron;
extern crate serde_json;
extern crate tokio;

use iron::headers;
use iron::prelude::*;
use iron::status::Status;

mod parser;
mod twitter;

use dotenv::dotenv;
use iron::mime;
use parser::parse_twitter;
use serde_json::json;
use std::env;
use std::io::{self, Write};
// #[get("/twitter?<screen_name>", format = "json")]
// fn twitter(screen_name: &RawStr) -> JsonValue {
//     let tweet_result = Reactor.rt::lazy(|| get_tweets(screen_name.as_str().to_owned())).unwrap();
//     match tweet_result {
//         Ok(tweets) => json!({
//             "validURL": true,
//             "data": tweets,
//         }),
//         Err(e) => json!({
//                 "validURL": false,
//                 "data": null,
//                 "error": format!("{:?}", e)
//         }),
//     }
//     //     Err(_) => json!({
//     //         "validURL": false,
//     //         "data": "undefined",
//     //     }),
//     // }
// }
fn handler(req: &mut Request) -> IronResult<Response> {
    let path = req.url.path();
    if path.len() == 2 && path[0] == "api" && path[1] == "twitter" {
        let content_type = "application/json".parse::<mime::Mime>().unwrap();
        if (req.url.query().is_none() == false) {
            let query = req.url.query().unwrap().to_owned();
            let mut query_iter = query.split("&");
            for q in query_iter {
                let (key, value) = q.split_at(4);
                if (key == "url=") {
                    parse_twitter(value.to_string());
                }
            }
            Ok(Response::with((
                content_type,
                Status::Ok,
                json!({
                "validURL": true,
                "data": null,
                }),
            )))
        } else {
            Ok(Response::with((
                content_type,
                Status::Ok,
                "{\"validURL\":false}",
            )))
        }
    } else {
        let content_type = "application/json".parse::<mime::Mime>().unwrap();
        Ok(Response::with((content_type, Status::NotFound)))
    }
}

fn main() {
    dotenv().ok();
    //get_tweets("realdonaldtrump".parse().unwrap());
    let args: Vec<String> = env::args().collect();
    Iron::new(handler).http("localhost:8000");
}
