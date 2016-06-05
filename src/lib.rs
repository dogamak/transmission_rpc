#![feature(custom_derive, custom_attribute, plugin, associated_consts)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_macros;
extern crate rustc_serialize;

pub mod error;
pub mod requests;
pub mod responses;
mod torrent;
mod field;

pub use self::torrent::Torrent;
pub use self::field::Field;

use error::{Error, Result, DaemonError};
use hyper::{Url, Client};
use hyper::client::Body;
use hyper::status::StatusCode;
use hyper::header::{ContentType, Headers, Authorization, Basic};
use hyper::mime::{Mime, TopLevel, SubLevel, self};
use requests::Request;
use responses::Response;
use serde::Deserialize;
use serde_json::Value;

/// A struct that represents the connection to the Transmission daemon.
struct Transmission {
    client:  Client,
    auth:    Option<Basic>,
    session: Option<String>,
    url:     Url,
    tag:     usize
}

impl Transmission {
    /// Create a new instance of `Transmission` using the default url
    /// `http://127.0.0.1:9091/transmission/rpc` and no authentication
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
    pub fn set_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

    /// Sets the credinteals to be used in future requests
    pub fn set_auth(mut self, username: String, password: String) -> Self {
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

    /// Sends the given request to the daemon and returns either the
    /// response from the daemon or an error
    pub fn send<R>(&mut self, mut request: &R) -> Result<R::Response>
        where R: Request, R::Response: Response
    {    
        let req_str = try!(serde_json::to_string(&request.to_value()));

        let mut response = try!(self.client.post(self.url.clone())
            .headers(self.headers())
            .body(&*req_str)
            .send());

        // X-Transmission-Session-Id HTTP header must be set to correct value for the
        // daemon to accept the request. The daemon returns the correct value for the
        // header on every response and changes that value from time to time.
        // HTTP status code 901 or `Conflict` means that we passed no or an invalid
        // session id.
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
        // Return also if the second try with the new session id failed too.
        if response.status != StatusCode::Ok {
            return Err(Error::Daemon(DaemonError::StatusCode(response.status)));
        }
        
        let value: Value = try!(serde_json::from_reader(&mut response));
        let obj = value.as_object().unwrap();

        if let Some(&Value::String(ref result)) = obj.get("result") {
            if result != &"success".to_string() {
                return Err(Error::Daemon(DaemonError::Result(result.clone())));
            }
        }
        
        let args = obj.get("arguments").unwrap();
        Ok(try!(R::Response::from_value(args.clone())))
    }
}

#[cfg(test)]
mod tests {
    use super::Transmission;
    use requests::{GetTorrent, AddTorrent};
    use field::Field;
    
    #[test]
    fn test() {
        let mut tr = Transmission::new()
            .set_auth("flcllcl".to_string(), "OpenRoad".to_string());

        let res = tr.send(&GetTorrent::new()
                          .field(Field::Name)
                          .field(Field::PercentDone)
                          .field(Field::Files));

        for torrent in res.unwrap() {
            println!("{}: {}", torrent.name.unwrap(), torrent.percent_done.unwrap());
        }

        let res = tr.send(&AddTorrent::from_file("/home/flcllcl/Lataukset/[HorribleSubs] Kabaneri of the Iron Fortress - 07 [480p].mkv.torrent").unwrap());

        println!("{:?}", res);
    }
}
