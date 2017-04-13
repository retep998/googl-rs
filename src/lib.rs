// Copyright © 2015-2017 Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! A simple interface to the Google URL Shortener API.
extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate url;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use hyper::Error as HttpError;
use hyper::header::{ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::status::{StatusCode};
use rustc_serialize::json::{BuilderError, Json};
use std::borrow::{ToOwned};
use std::io::Read;
use std::io::Error as IoError;
use url::form_urlencoded::{Serializer};

const BASEURL: &'static str = "https://www.googleapis.com/urlshortener/v1/url";

/// Contains all possible errors you might get while shortening a URL
#[derive(Debug)]
pub enum Error {
    BadStatus(StatusCode, String),
    Http(HttpError),
    Io(IoError),
    Json(BuilderError),
    MissingId(Json),
}
impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error::Http(err)
    }
}
impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}
impl From<BuilderError> for Error {
    fn from(err: BuilderError) -> Error {
        Error::Json(err)
    }
}

/// Shortens a URL using the Google URL Shortener API
pub fn shorten(key: &str, longurl: &str) -> Result<String, Error> {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let args = &[("key", key)];
    let query = Serializer::new(String::new()).extend_pairs(args).finish();
    let url = format!("{}?{}", BASEURL, query);
    let body = vec![("longUrl".to_owned(), Json::String(longurl.to_owned()))];
    let body = Json::Object(body.into_iter().collect()).to_string();
    println!("{:?}", body);
    let mut response = try!(client.post(&url)
        .header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])))
        .body(&body)
        .send());
    let mut body = String::new();
    try!(response.read_to_string(&mut body));
    if response.status != StatusCode::Ok {
        return Err(Error::BadStatus(response.status, body))
    }
    let json = try!(Json::from_str(&*body));
    let id = json.find("id").and_then(|x| x.as_string());
    match id {
        Some(id) => Ok(id.to_owned()),
        None => Err(Error::MissingId(json.clone())), //FIXME - nonlexical borrows
    }
}
