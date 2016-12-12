mod get_torrent;
mod add_torrent;

pub use self::get_torrent::GetTorrent;
pub use self::add_torrent::AddTorrent;

use serde_json::{self, Value};
use serde::Deserialize;
use error::deserialize::*;

/// A trait that is used in deserializing th e daemons response.
pub trait Response: Sized {
    fn from_value(v: Value) -> Result<Self>;
}

impl<T> Response for T where T: Deserialize {
    fn from_value(v: Value) -> Result<T> {
        Ok(serde_json::from_value::<T>(v)?)
    }
}
