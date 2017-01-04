#![feature(proc_macro, custom_derive, custom_attribute, plugin, associated_consts)]
#![plugin(torrent_macro)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate chrono;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate rustc_serialize;

pub mod error;
pub mod requests;
pub mod responses;
pub mod types;
mod torrent;

#[cfg(test)]
mod tests;

pub use self::torrent::*;

use error::*;
use hyper::{Url, Client};
use hyper::status::StatusCode;
use hyper::header::{ContentType, Headers, Authorization, Basic};
use hyper::mime::{Mime, TopLevel, SubLevel, self};
use requests::Request;
use responses::Response;
use serde_json::Value;

/// A struct that represents the connection to the Transmission daemon.
pub struct Transmission {
    client:  Client,
    auth:    Option<Basic>,
    session: Option<String>,
    url:     Url,
    tag:     usize
}

impl Transmission {
    /// Create a new instance of `Transmission` using the default url
    /// `http://127.0.0.1:9091/transmission/rpc` and no authentication.
    pub fn new() -> Transmission {
        Transmission {
            client:  Client::new(),
            auth:    None,
            session: None,
            url:    Url::parse("http://127.0.0.1:9091/transmission/rpc").unwrap(),
            tag:     0
        }
    }

    /// Sets the url used to connect the daemon for future requests
    pub fn set_url<U>(&mut self, url: U) -> &mut Self where Url: From<U> {
        self.url = Url::from(url) ;
        self
    }

    /// Sets the credinteals to be used in future requests
    pub fn set_auth(&mut self, username: String, password: String) -> &mut Self {
        self.auth = Some(Basic{
            username: username,
            password: Some(password)
        });
        self
    }

    fn headers(&self) -> Headers {
        let mut headers = Headers::new();

        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                     vec![(mime::Attr::Charset, mime::Value::Utf8)])));

        if let Some(ref key) = self.session {
            headers.set_raw("X-Transmission-Session-Id", vec![key.clone().into_bytes()]);
        }
        
        if let Some(ref auth) = self.auth {
            headers.set(Authorization(auth.clone()));
        }

        headers
    }

    /// Sends given request to the daemon and returns the received response.
    pub fn send<R>(&mut self, request: &R) -> Result<R::Response>
        where R: Request, R::Response: Response
    {    
        let req_str = serde_json::to_string(&request.to_value())
            .map_err(|e| ::error::deserialize::Error::from(e))?;

        let mut response = try!(self.client.post(self.url.clone())
            .headers(self.headers())
            .body(&*req_str)
            .send());

        // X-Transmission-Session-Id HTTP header must be set to correct value for the
        // daemon to accept the request. The daemon returns the wanted value for the
        // header on every response and changes that value from time to time.
        // HTTP status code 901 or `Conflict` means that the daemon received an invalid
        // or no session id.
        if response.status == StatusCode::Conflict {
            // Get the correct session id from the response. 
            self.session = Some(String::from_utf8(response.headers.get_raw("X-Transmission-Session-Id").unwrap()[0].clone()).unwrap());

            // Try again using the correct id.
            response = try!(self.client.post(self.url.clone())
                .headers(self.headers())
                .body(&*req_str)
                .send());
        }

        // If the daemon responded with status other than 200 return an error.
        // Return also if the second try with the new session id failed.
        if response.status != StatusCode::Ok {
            bail!(::error::ErrorKind::Daemon(::error::daemon::ErrorKind::StatusCode(response.status)));
        }
        
        let value: Value = serde_json::from_reader(&mut response)
            .map_err(|e| ::error::ErrorKind::Deserialize(::error::deserialize::ErrorKind::Json(e)))?;
        let obj = value.as_object().unwrap();

        if let Some(&Value::String(ref result)) = obj.get("result") {
            if result != &"success".to_string() {
                bail!(ErrorKind::Daemon(::error::daemon::ErrorKind::Message(result.clone())));
            }
        }
        
        let args = obj.get("arguments").unwrap();
        Ok(R::Response::from_value(args.clone())?)
    }
}
