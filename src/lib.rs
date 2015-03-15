// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! A simple interface to the Google URL Shortener API.
extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate url;

use hyper::{HttpError};
use hyper::client::{Request};
use hyper::header::{ContentType};
use hyper::method::{Method};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::status::{StatusCode};
use rustc_serialize::json::{BuilderError, Json};
use std::borrow::{ToOwned};
use std::error::{FromError};
use std::io::{Read, Write};
use std::io::Error as IoError;
use url::{Url};
use url::ParseError as UrlError;
use url::form_urlencoded::{serialize};

const BASEURL: &'static str = "https://www.googleapis.com/urlshortener/v1/url";

/// Contains all possible errors you might get while shortening a URL
#[derive(Debug)]
pub enum Error {
    BadStatus(StatusCode, String),
    Http(HttpError),
    Io(IoError),
    Json(BuilderError),
    MissingId(Json),
    Url(UrlError),
}
impl FromError<HttpError> for Error {
    fn from_error(err: HttpError) -> Error {
        Error::Http(err)
    }
}
impl FromError<IoError> for Error {
    fn from_error(err: IoError) -> Error {
        Error::Io(err)
    }
}
impl FromError<BuilderError> for Error {
    fn from_error(err: BuilderError) -> Error {
        Error::Json(err)
    }
}
impl FromError<UrlError> for Error {
    fn from_error(err: UrlError) -> Error {
        Error::Url(err)
    }
}

/// Shortens a URL using the Google URL Shortener API
pub fn shorten(key: &str, longurl: &str) -> Result<String, Error> {
    let query = [("key", key)];
    let query = serialize(query.iter().map(|&x| x));
    let url = format!("{}?{}", BASEURL, query);
    let url = try!(Url::parse(&url));
    let mut request = try!(Request::new(Method::Post, url));
    request.headers_mut().set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
    let mut request = try!(request.start());
    let body = vec![("longUrl".to_owned(), Json::String(longurl.to_owned()))];
    let body = Json::Object(body.into_iter().collect());
    try!(request.write_all(body.to_string().as_bytes()));
    let mut response = try!(request.send());
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
