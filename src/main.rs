#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use] extern crate hyper;
extern crate ease;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use ease::{Url, Request};
use ease::header::Authorization;
use hyper::header::{Headers, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
header! { (ApiKey, "apiKey") => [String] }

#[derive(Deserialize, Debug)]
struct PostResponse {
    args: HashMap<String, String>,
    data: Option<String>,
    files: Option<HashMap<String, String>>,
    form: Option<HashMap<String, String>>,
    headers: HashMap<String, String>,
    json: Option<String>,
    origin: String,
    url: String,
}

fn main() {
    let url = Url::parse("http://thegame.nerderylabs.com/points").unwrap();
    let key = "2d13ac86-6ae9-4e87-82da-cc9190a6b1f8".to_owned();
    let mut request = Request::new(url);
    println!("{:?}", request
                        .header(ApiKey(key))
                             .post().unwrap().body);
}
