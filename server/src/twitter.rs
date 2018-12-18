use dotenv_codegen::dotenv;
use futures::prelude::*;
use hyper::header::{HeaderValue, AUTHORIZATION};
use hyper::rt::{self, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::io::{self, Write};
use std::iter;
pub fn get_tweets(screen_name: String) -> impl Future<Item = Vec<Tweet>, Error = FetchError> {
    let uri = format!(
        "{}?screen_name={}",
        env::var("API_URL").unwrap().to_string(),
        screen_name
    )
    .parse()
    .unwrap();

    fetch_json(uri).and_then(|tweets| Ok(tweets))
}

fn fetch_json(url: hyper::Uri) -> impl Future<Item = Vec<Tweet>, Error = FetchError> {
    let https = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);

    client
        .get(url)
        .and_then(|res| res.into_body().concat2())
        .from_err::<FetchError>()
        .and_then(|body| {
            // println!(
            //     "body: {}",
            //     String::from_utf8_lossy(&(body.to_vec())).to_string()
            // );
            let tweets: Vec<Tweet> = serde_json::from_slice(&body)?;

            Ok(tweets)
        })
        .from_err()
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    pub created_at: String,
    pub id_str: String,
    pub text: String,
}

// Define a type so we can return multiple types of errors
#[derive(Debug)]
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}
