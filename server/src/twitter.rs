use futures::prelude::*;
use hmac::{Hmac, Mac};
use hyper::header::AUTHORIZATION;
use hyper::rt::Stream;
use hyper::Client;
use hyper::HeaderMap;
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use sha1::Sha1;
use std::env;
use std::str;

type HmacSha1 = Hmac<Sha1>;
fn create_base_string(
    consumer_key: String,
    oauth_nonce: String,
    timestamp: String,
    oauth_token: String,
    screen_name: String,
) -> String {
    format!("POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fuser_timeline.json&include_entities%3Dtrue%26oauth_consumer_key%3D{}%26oauth_nonce%3D{}%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D{}%26oauth_token%3D{}%26oauth_version%3D1.0%26screen_name%3D{}",
        consumer_key, oauth_nonce, timestamp, oauth_token, screen_name)
}
fn create_signature(consumer_secret: String, token_secret: String, base_string: String) -> String {
    let mut mac = HmacSha1::new_varkey(format!("{}&{}", consumer_secret, token_secret).as_bytes())
        .expect("HMAC can take key of any size");
    mac.input(format!("{}", base_string).as_bytes());

    str::from_utf8(mac.result().code().as_slice())
        .unwrap()
        .to_string()
}

fn get_timestamp() -> String {}

fn get_nonce() -> String {}

pub fn get_tweets(screen_name: String) -> impl Future<Item = Vec<Tweet>, Error = FetchError> {
    let mut headers = HeaderMap::new();

    let oauth_timestamp = get_timestamp();
    let oauth_nonce = get_nonce();

    let oauth_consumer_key = env::var("API_KEY").unwrap().to_string();
    let oauth_consumer_secret = env::var("API_SECRET").unwrap().to_string();

    let oauth_token = env::var("ACCESS_TOKEN").unwrap().to_string();
    let oauth_token_secret = env::var("TOKEN_SECRET").unwrap().to_string();

    headers.insert(AUTHORIZATION, format!("OAuth oauth_consumer_key=\"{}\",oauth_token=\"{}\",oauth_signature_method=\"HMAC-SHA1\",oauth_timestamp=\"{}\",oauth_nonce=\"{}\",oauth_version=\"1.0\",oauth_signature=\"{}\"",
        oauth_consumer_key, //consumer key
        oauth_token, //access token
        oauth_timestamp, //based on time
        oauth_nonce, //generate nonce
        create_signature(oauth_consumer_secret, oauth_token_secret, create_base_string(oauth_consumer_key, oauth_nonce, oauth_timestamp, oauth_token, screen_name))
        ).parse().unwrap());
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
