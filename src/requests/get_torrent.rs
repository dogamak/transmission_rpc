use torrent::Torrent;
use field::Field;
use responses::GetTorrent as GetTorrentResponse;
use super::Request;

/// A request to fetch information about the requested torrent(s).
#[derive(Serialize, Clone)]
pub struct GetTorrent {
    #[serde(rename="ids", skip_serializing_if="Vec::is_empty")]
    _ids: Vec<u32>,
    #[serde(rename="fields")]
    _fields: Vec<Field>
}

impl GetTorrent {
    /// Creates a request to fetch all available information of all the torrents.
    /// An empty list of torrent ids fetches all the torrents.
    /// An empty list of fields fetches all the fields.
    pub fn new() -> GetTorrent {
        GetTorrent {
            _ids: Vec::new(),
            _fields: Vec::new()
        }
    }

    /// Adds an torrent which's information is to fetched by it's id.
    pub fn id(mut self, id: u32) -> Self {
        self._ids.push(id);
        self
    }

    /// Sets the list of those torrent's ids which are to be fetched.
    pub fn ids(mut self, ids: Vec<u32>) -> Self {
        self._ids = ids;
        self
    }

    /// Add an field to the list of fields that are requested.
    pub fn field(mut self, field: Field) -> Self {
        self._fields.push(field);
        self
    }

    /// Sets the list of fields that are requested.
    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self._fields = fields;
        self
    }
}

impl Request for GetTorrent {
    type Response = GetTorrentResponse;
    const Name: &'static str = "torrent-get";
}
