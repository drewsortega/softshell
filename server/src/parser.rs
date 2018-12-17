use iron::headers;
use iron::mime;
use iron::prelude::*;
use iron::status::Status;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

use super::twitter::get_tweets;
use super::twitter::FetchError;
use dotenv::dotenv;
use std::env;
use std::io::{self, Write};
use tokio::reactor::Reactor;
// fn get_empty_map() -> HashMap<String, u32> {
//     return new HashMap<String, u32>
// }

pub fn parse_twitter(url: String) -> Result<Response, IronError> {
    let tweet_result = Reactor::rt::lazy(|| get_tweets(url.as_str().to_owned())).unwrap();
    match tweet_result {
        Ok(tweets) => Response::with(Response::with((
            content_type,
            Status::Ok,
            json!({
            "validURL": true,
            "data": tweets,
            }),
        ))),
        Err(e) => json!({
                "validURL": false,
                "data": null,
                "error": format!("{:?}", e)
        }),
    }
}
