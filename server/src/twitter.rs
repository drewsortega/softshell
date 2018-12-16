use dotenv_codegen::dotenv;
use hyper::header::{HeaderValue, AUTHORIZATION};
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::io::{self, Write};
use std::iter;

pub fn get_tweets(screen_name: String) {
    let uri = format!(
        "{}?screen_name={}",
        env::var("API_URL").unwrap().to_string(),
        screen_name
    )
    .parse()
    .unwrap();
    println!("{}", uri);
    let fut = rt::lazy(|| {
        fetch_json(uri)
            .map(|tweets| {
                for tweet in &tweets {
                    println!("{}", tweet.text);
                }
            })
            .map_err(|err| match err {
                FetchError::Http(err) => eprintln!("http error: {}", err),
                FetchError::Json(err) => eprintln!("json parsing error: {}", err),
            })
    });

    rt::run(fut);
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
#[derive(Deserialize, Debug)]
struct Tweet {
    created_at: String,
    id_str: String,
    text: String,
}

// Define a type so we can return multiple types of errors
enum FetchError {
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
