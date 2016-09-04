use torrent::Torrent;
use std::iter::IntoIterator;
use std::vec;

/// A response to the request `GetTorrent`
#[derive(Deserialize, Debug)]
pub struct GetTorrent {
    torrents: Vec<Torrent>
}

impl IntoIterator for GetTorrent {
    type Item = Torrent;
    type IntoIter = vec::IntoIter<Torrent>;

    /// Returns an iterator over received torrents.
    fn into_iter(self) -> vec::IntoIter<Torrent> {
        self.torrents.into_iter()
    }
}
