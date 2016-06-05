use super::Request;
use responses::AddTorrent as AddTorrentResponse;
use std::io::Read;
use rustc_serialize::base64::{self, ToBase64};
use std::path::{PathBuf, Path};
use error::Result;
use std::fs::File;

/// Request the daemon to add an torrent. Torrent can be added from the
/// metainfo held in an .torrent file or by providing the daemon an URL
/// or a file path where it can fetch the metainfo.
#[derive(Serialize, Clone)]
pub struct AddTorrent {
    #[serde(skip_serializing_if="Option::is_none")]
    filename: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    metainfo: Option<String>,
    #[serde(rename="download_dir", skip_serializing_if="Option::is_none")]
    _download_dir: Option<PathBuf>,
    #[serde(rename="paused", skip_serializing_if="Option::is_none")]
    _paused: Option<bool>,
    #[serde(rename="peer_limit", skip_serializing_if="Option::is_none")]
    _peer_limit: Option<u32>,
}

impl AddTorrent {
    /// Reads the torrent's metainfo from a reader and creates a request to create
    /// a new torrent for the daemon using that metainfo. Returns an error in case
    /// reading from the reader fails.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<AddTorrent> {
        let mut bytes = Vec::new();
        try!(reader.read_to_end(&mut bytes));
        let bytes: &[u8] = bytes.as_slice();
        let metainfo = bytes.to_base64(base64::STANDARD);

        Ok(AddTorrent {
            filename: None,
            metainfo: Some(metainfo),
            _download_dir: None,
            _paused: None,
            _peer_limit: None
        })
    }

    /// A convinience function that opens a file and gives it's reader to the `from_reader`
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<AddTorrent> {
        let mut file = try!(File::open(&path));
        AddTorrent::from_reader(&mut file)
    }

    /// Passes the URL or file path provided to the daemon so that the daemon can fetch the metainfo from there.
    /// Please note that the path is for a file on the machine running the daemon and the user that
    /// runs the daemon should have the permissions to read that file.
    pub fn from_source<S: Into<String>>(source: S) -> AddTorrent {
        AddTorrent {
            filename: Some(source.into()),
            metainfo: None,
            _download_dir: None,
            _paused: None,
            _peer_limit: None
        }
    }

    /// Sets the directory to where the daemon will download the torrent's content.
    /// Please note that the path is for a directory on the machine running the daemon and the user
    /// running the daemon should have the right's to create that directory and write to it.
    pub fn download_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self._download_dir = Some(path.into());
        self
    }

    /// Sets the initial state of the torrent.
    pub fn paused(mut self, paused: bool) -> Self {
        self._paused = Some(paused);
        self
    }

    /// Sets the peer limit on the torrent.
    pub fn peer_limit(mut self, limit: u32) -> Self {
        self._peer_limit = Some(limit);
        self
    }
}

impl Request for AddTorrent {
    type Response = AddTorrentResponse;
    const Name: &'static str = "torrent-add";
}
