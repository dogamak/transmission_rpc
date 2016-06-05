use std::error::{self, Error as StdError};
use hyper;
use hyper::status::StatusCode;
use serde_json;
use std::fmt;
use std::io;

pub type Result<T> = ::std::result::Result<T, Error>;


/// A general error type used troughout the library.
#[derive(Debug)]
pub enum Error {
    /// Contacting the daemon failed
    Http(hyper::Error),
    /// Deserializing the response from the daemon failed
    Deserialize(DeserializeError),
    /// Daemon returned an error
    Daemon(DaemonError),
    /// General IO error
    Io(io::Error)
}

/// An error type returned when deserializing the deamon's response fails.
#[derive(Debug)]
pub enum DeserializeError {
    /// JSON related error
    Json(serde_json::Error),
    /// The response is missing an expected field. The payload is the fields name.
    MissingField(String),
    /// A field on the response has an invalid type. The first field is
    /// the name of the expected type and the second is the field's name. 
    InvalidType(String, String),
}

/// An error type returned when daemon returns an error
#[derive(Debug)]
pub enum DaemonError {
    /// The response from the daemon had a HTTP status code different from 200
    StatusCode(StatusCode),
    /// The daemon responded with an error message
    Result(String)
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Http(_) => "HTTP error",
            &Error::Deserialize(_) => "deserializing error",
            &Error::Daemon(_) => "daemon returned an error",
            &Error::Io(_) => "IO error"
        }
    }
    
    fn cause(&self) -> Option<&StdError> {
        match self {
            &Error::Http(ref e) => Some(e),
            &Error::Deserialize(ref e) => Some(e),
            &Error::Daemon(ref e) => Some(e),
            &Error::Io(ref e) => Some(e)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Http(ref e) =>
                write!(f, "HTTP error: {}", e),
            &Error::Deserialize(ref e) =>
                write!(f, "deserializing error: {}", e),
            &Error::Daemon(ref e) =>
                write!(f, "daemon returned an error: {}", e),
            &Error::Io(ref e) =>
                write!(f, "IO error: {}", e)
        }
    }
}

impl StdError for DeserializeError {
    fn description(&self) -> &str {
        match self {
            &DeserializeError::Json(ref e) => "JSON error",
            &DeserializeError::MissingField(_) => "Missing field",
            &DeserializeError::InvalidType(_, _) => "Invalid type"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            &DeserializeError::Json(ref e) => Some(e),
            _ => None
        }
    }
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DeserializeError::Json(ref e) =>
                write!(f, "JSON Error: {}", e),
            &DeserializeError::MissingField(ref name) =>
                write!(f, "Missing field '{}'", name),
            &DeserializeError::InvalidType(ref typ, ref name) =>
                write!(f, "Field type '{}' for field '{}'", typ, name)
        }
    }
}

impl StdError for DaemonError {
    fn description(&self) -> &str {
        match self {
            &DaemonError::StatusCode(_) => "non-success HTTP status code",
            &DaemonError::Result(_) => "daemon returned an error"
        }
    }
}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DaemonError::StatusCode(ref c) =>
                write!(f, "non-success HTTP status code: {}", c),
            &DaemonError::Result(ref e) =>
                write!(f, "daemon returned an error: {}", e)
        }
    }
}

impl From<DeserializeError> for Error {
    fn from(e: DeserializeError) -> Error {
        Error::Deserialize(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Deserialize(DeserializeError::Json(e))
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Error {
        Error::Http(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for DeserializeError {
    fn from(e: serde_json::Error) -> DeserializeError {
        DeserializeError::Json(e)
    }
}
