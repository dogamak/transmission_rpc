mod get_torrent;
mod add_torrent;

pub use self::get_torrent::GetTorrent;
pub use self::add_torrent::AddTorrent;

use std::collections::HashMap;
use serde_json::{self, Value};
use serde::Deserialize;
use error::DeserializeError;

/// A trait that is used in deserializing th e daemons response.
pub trait Response: Sized {
    fn from_value(v: Value) -> Result<Self, DeserializeError>;
}

impl<T> Response for T where T: Deserialize {
    fn from_value(v: Value) -> Result<T, DeserializeError> {
        serde_json::from_value::<T>(v)
            .map_err(|e| DeserializeError::from(e))
    }
}
