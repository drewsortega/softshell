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
use regex::Regex;
use lazy_static;
use std::str;

type HmacSha1 = Hmac<Sha1>;

#[derive(Debug, Clone)]
pub struct Param {
    pub key: String,
    pub value: String,
}

percent_encode(input: String) -> String {
    for c in input {
        
    }
}

fn create_parameters_string(parameters: &mut Vec<Param>) -> String {
    parameters.sort_by(|a, b| a.key.cmp(&b.key));
    let mut parameter_string: String = "".to_string();
    for param in parameters {
        if parameter_string.len() != 0 {
            parameter_string.push_str("&");
        }
        parameter_string = format!(
            "{}{}={}",
            parameter_string,
            utf8_percent_encode(param.key.as_str(), USERINFO_ENCODE_SET).to_string(),
            utf8_percent_encode(param.value.as_str(), USERINFO_ENCODE_SET).to_string()
        );
        println!(
            "{}",
            utf8_percent_encode(param.key.as_str(), USERINFO_ENCODE_SET)
        );
    }
    parameter_string
}

fn create_base_string(url: &String, parameters: &String) -> String {
    format!(
        "GET&{}&{}",
        utf8_percent_encode(url.as_str(), USERINFO_ENCODE_SET).to_string(),
        utf8_percent_encode(parameters.as_str(), USERINFO_ENCODE_SET).to_string()
    )
}
fn create_signature(
    consumer_secret: &String,
    token_secret: &String,
    base_string: &String,
) -> String {
    let mut mac = HmacSha1::new_varkey(format!("{}&{}", consumer_secret, token_secret).as_bytes())
        .expect("HMAC can take key of any size");
    mac.input(format!("{}", base_string).as_bytes());

    str::from_utf8(mac.result().code().as_slice())
        .unwrap()
        .to_string()
}

fn get_timestamp() -> String {
    //TODO
    "".to_string()
}

fn get_nonce() -> String {
    //TODO
    "".to_string()
}

pub fn get_tweets(screen_name: &String) -> impl Future<Item = Vec<Tweet>, Error = FetchError> {
    let mut headers = HeaderMap::new();

    let mut params: Vec<Param> = vec![
        Param {
            key: "include_entities".to_string(),
            value: "true".to_string(),
        },
        Param {
            key: "oauth_consumer_key".to_string(),
            value: env::var("API_KEY").unwrap().to_string(),
        },
        Param {
            key: "oauth_nonce".to_string(),
            value: get_nonce(),
        },
        Param {
            key: "oauth_signature_method".to_string(),
            value: "HMAC-SHA1".to_string(),
        },
        Param {
            key: "oauth_timestamp".to_string(),
            value: get_timestamp(),
        },
        Param {
            key: "oauth_token".to_string(),
            value: env::var("ACCESS_TOKEN").unwrap().to_string(),
        },
        Param {
            key: "oauth_version".to_string(),
            value: "1.0".to_string(),
        },
        Param {
            key: "screen_name".to_string(),
            value: screen_name.to_string(),
        },
    ];
    let url = env::var("API_URL").unwrap().to_string();
    let oauth_consumer_secret = env::var("API_SECRET").unwrap().to_string();
    let oauth_token_secret = env::var("TOKEN_SECRET").unwrap().to_string();

    //todo: add twitter start index
    let parameter_string = create_parameters_string(&mut params);
    let base_string = create_base_string(&url, &parameter_string);
    let signature = create_signature(&oauth_consumer_secret, &oauth_token_secret, &base_string);

    headers.insert(AUTHORIZATION, format!("OAuth oauth_consumer_key=\"{}\",oauth_token=\"{}\",oauth_signature_method=\"HMAC-SHA1\",oauth_timestamp=\"{}\",oauth_nonce=\"{}\",oauth_version=\"1.0\",oauth_signature=\"{}\"",
        params[1].value, //consumer key
        params[5].value, //access token
        params[4].value, //timestamp based on time
        params[2].value, //nonce
        signature
        ).parse().unwrap());
    let uri = format!("{}?screen_name={}", url, screen_name)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_parameters_string_test() {
        let mut params = vec![
            Param {
                key: "param1k".to_string(),
                value: "param1v".to_string(),
            },
            Param {
                key: "2k".to_string(),
                value: "2v".to_string(),
            },
            Param {
                key: "1'2".to_string(),
                value: "a=c".to_string(),
            },
        ];
        assert_eq!(
            "1%272=a%3Dc&2k=2v&param1k=param1v",
            create_parameters_string(&mut params)
        );
    }
}
