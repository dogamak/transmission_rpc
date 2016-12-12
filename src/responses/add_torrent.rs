use super::Response;
use error::deserialize::*;
use serde_json::Value;
use serde_json;
use serde::{self, Error as SerdeError};

/// A response to the request `AddTorrent`.
#[derive(Debug)]
pub struct AddTorrent {
    /// The id of the torrent
    pub id: u64,
    /// The name of the torrent
    pub name: String,
    /// The hash string of the torrent
    pub hash_string: String,
    /// If true, the damon already had the torrent and did not create duplicate intance of
    /// it but instead sent us the old torrent's information.
    pub was_duplicate: bool
}

impl Response for AddTorrent {
    fn from_value(value: Value) -> Result<AddTorrent> {
        let obj = value.as_object().unwrap();

        let mut was_duplicate = false;
        let mut info: Value;
        if let Some(i) = obj.get("torrent-added") {
            info = i.clone();
        } else if let Some(i) = obj.get("torrent-duplicate") {
            info = i.clone();
            was_duplicate = true;
        } else {
            bail!(ErrorKind::MissingField("torrent-added".to_string()))
        }

        let field = match was_duplicate {
            true => "torrent-duplicate",
            false => "torrent-added"
        };
        
        let info = info.as_object()
            .ok_or(ErrorKind::InvalidType("object".to_string(), field.to_string()))?;

        Ok(AddTorrent {
            was_duplicate: was_duplicate,
            id: try!(info.get("id")
                .ok_or(ErrorKind::MissingField("id".to_string()))
                .and_then(|v| v.as_u64()
                          .ok_or(ErrorKind::InvalidType("number".to_string(), "id".to_string())))),
            name: try!(info.get("name")
                       .ok_or(ErrorKind::MissingField("name".to_string()))
                       .and_then(|v| v.as_str()
                                 .ok_or(ErrorKind::InvalidType("string".to_string(), "name".to_string())))).to_string(),
            hash_string: try!(info.get("hashString")
                              .ok_or(ErrorKind::MissingField("hashString".to_string()))
                              .and_then(|v| v.as_str()
                                        .ok_or(ErrorKind::InvalidType("string".to_string(), "hashString".to_string())))).to_string()
        })
    }
}
