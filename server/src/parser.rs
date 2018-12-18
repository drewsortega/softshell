use iron::mime;
use iron::prelude::*;
use iron::status::Status;
use serde_derive::Serialize;
use serde_json::json;
use std::collections::HashMap;

use super::twitter::get_tweets;
use super::twitter::FetchError;
use super::twitter::Tweet;
use std::vec::Vec;
use tokio_core::reactor::Core;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// fn get_empty_map() -> HashMap<String, u32> {
//     return new HashMap<String, u32>
// }
#[derive(Clone, Serialize)]
pub struct Word {
    pub text: String,
    pub count: u32,
}
pub fn get_empty_bad_hashmap() -> HashMap<String, u32> {
    let path = Path::new("bad_words.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut words = String::new();
    match file.read_to_string(&mut words) {
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(_) => {
            let mut map: HashMap<String, u32> = HashMap::new();
            for word in words.lines() {
                map.insert(word.to_string(), 0);
            }
            map
        }
    }
}
pub fn sort_insert(new_word: &Word, words: &mut Vec<Word>) {
    let mut contains = false;
    for i in 0..10 {
        if new_word.text == words[i].text {
            words[i].count = new_word.count;
            contains = true;
        }
    }
    if !contains {
        for i in 0..10 {
            if new_word.count >= words[i].count {
                words.insert(i, new_word.to_owned());
            }
        }
        words.resize(
            10,
            Word {
                text: "".to_string(),
                count: 0,
            },
        );
    }
}
pub fn top_10_bad(tweets: Vec<Tweet>) -> Vec<Word> {
    let mut map = get_empty_bad_hashmap();
    let mut top_bad: Vec<Word> = Vec::new();
    top_bad.resize(
        10,
        Word {
            text: "".to_string(),
            count: 0,
        },
    );
    for tweet in tweets {
        let words_iter = tweet.text.split(" ");
        for word in words_iter {
            if map.contains_key(word) {
                *map.get_mut(word).unwrap() += 1;
            }
            sort_insert(
                &(Word {
                    text: word.to_string(),
                    count: *map.get(word).unwrap(),
                }),
                &mut top_bad,
            );
        }
    }
    top_bad
}

pub fn parse_twitter(url: String) -> Result<Response, IronError> {
    let content_type = "application/json".parse::<mime::Mime>().unwrap();
    let mut reactor = Core::new().unwrap();
    let tweet_result: Result<Vec<Tweet>, FetchError> =
        reactor.run(get_tweets(url.as_str().to_owned()));
    match tweet_result {
        Ok(tweets) => Ok(Response::with((
            content_type,
            Status::Ok,
            json!({
            "validURL": true,
            "data": top_10_bad(tweets),
            })
            .to_string(),
        ))),
        Err(_) => Ok(Response::with((
            content_type,
            Status::InternalServerError,
            json!({
            "validURL": false,
            "data": null,
            })
            .to_string(),
        ))),
    }
}
