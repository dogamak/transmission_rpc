mod get_torrent;
mod add_torrent;
mod torrent_set;

pub use self::get_torrent::GetTorrent;
pub use self::add_torrent::AddTorrent;
pub use self::torrent_set::TorrentSet;

use serde_json::{self, Value};
use serde::Serialize;
use std::collections::BTreeMap;

/// A trait representing a request sent to the daemon.
pub trait Request: Serialize {
    type Response;
    const NAME: &'static str;

    fn arguments(&self) -> Value {
        serde_json::to_value(self)
    }
    
    fn to_value(&self) -> Value {
        Value::Object({
            let mut obj = BTreeMap::new();
            obj.insert("method".to_string(), Value::String(Self::NAME.to_string()));
            obj.insert("arguments".to_string(), self.arguments());
            obj
        })
    }
}
