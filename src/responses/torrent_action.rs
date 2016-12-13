use super::Response;
use serde_json::Value;
use error::deserialize::Result;

pub struct TorrentAction(());

impl Response for TorrentAction {
    fn from_value(v: Value) -> Result<TorrentAction> {
        return Ok(TorrentAction(()));
    }
}
