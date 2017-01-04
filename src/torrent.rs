use types::{Status, Priority};
use chrono::NaiveDateTime;
use ::types::time_t::deserialize_time_t_option;

torrent_proc! {
    /// Struct containing information about a torrent. All fields are optional and wrapped in Option.
    ///
    /// This is generated using procedural macro in the `torrent_macro` crate.
    #[derive(Deserialize,Debug)]
    pub struct Torrent {
        #[time_t] pub activity_date: NaiveDateTime, 
        #[time_t] pub added_date: NaiveDateTime, 
        pub announce_response: String,
        pub announce_url: String,
        pub bandwidth_priority: Priority,
        pub comment: String,
        pub corrupt_ever: u64,
        pub creator: String,
        #[time_t] pub date_created: NaiveDateTime, 
        pub desired_available: u64,
        #[time_t] pub done_date: NaiveDateTime, 
        pub download_dir: String,
        pub downloaded_ever: u64,
        pub downloaders: u64,
        pub download_limit: u64,
        pub download_limited: bool,
        pub error: u64,
        pub error_string: String,
        pub eta: NaiveDateTime, 
        pub files: Vec<File>,
        pub file_stats: Vec<FileStat>,
        pub hash_string: String,
        pub have_unchecked: u64,
        pub have_valid: u64,
        pub honors_session_limits: bool,
        pub id: u64,
        pub is_private: bool,
        #[time_t] pub last_announce_time: NaiveDateTime, 
        pub last_scrape_time: u64,
        pub leechers: u64,
        pub left_until_done: u64,
        #[time_t] pub manual_announce_time: NaiveDateTime, 
        pub max_connected_peers: u64,
        pub name: String,
        #[time_t] pub next_announce_time: NaiveDateTime, 
        #[time_t] pub next_scrape_time: NaiveDateTime, 
        pub peer_limit: u64,
        pub peers: Vec<Peer>,
        pub peers_connected: u64,
        pub peers_from: PeersFrom,
        pub peers_getting_from_us: u64,
        pub peers_known: u64,
        pub peers_sending_to_us: u64,
        pub percent_done: f64,
        pub pieces: String,
        pub piece_count: u64,
        pub piece_size: u64,
        pub priorities: Vec<Priority>,
        pub rate_download: u64,
        pub rate_upload: u64,
        pub recheck_progress: f64,
        pub scrape_response: String,
        pub scrape_url: String,
        pub seeders: u64,
        pub seed_ratio_limit: f64,
        pub seed_ratio_mode: u64,
        pub size_when_done: u64,
        #[time_t] pub start_date: NaiveDateTime, 
        pub status: Status,
        pub swarm_speed: u64,
        pub times_completed: u64,
        pub trackers: Vec<Tracker>,
        pub total_size: u64,
        pub torrent_file: String,
        pub uploaded_ever: u64,
        pub upload_limit: u64,
        pub upload_limited: bool,
        pub upload_ratio: f64,
        pub wanted: Vec<bool>,
        pub webseeds: Vec<String>,
        pub webseeds_sending_to_us: u64,
    }

    /// Enum with each variant representing a field in the `Torrent` struct.
    ///
    /// Generated using procedural macro in the `torrent_macro` crate.
    #[derive(Debug,Clone)]
    pub enum TorrentField;
}

/// Contains information about a file on the disk.
#[derive(Deserialize, Clone, Debug)]
pub struct File {
    #[serde(rename="bytesCompleted")]
    pub bytes_completed: usize,
    pub length: usize,
    pub name: String
}

/// Contains information about transmission's state regarding a file.
#[derive(Deserialize, Clone, Debug)]
pub struct FileStat {
    #[serde(rename="bytesCompleted")]
    pub bytes_completed: usize,
    pub wanted: bool,
    pub priority: Priority,
}

/// Contains information about BitTorrent peer.
#[derive(Deserialize, Clone, Debug)]
pub struct Peer {
    pub address: String,
    #[serde(rename="clientName")]
    pub client_name: String,
    #[serde(rename="clientIsChocked")]
    pub client_is_chocked: bool,
    #[serde(rename="clientIsIntrested")]
    pub client_is_intrested: bool,
    #[serde(rename="isDownloadingFrom")]
    pub is_downloading_from: bool,
    #[serde(rename="isEncrypted")]
    pub is_encrypted: bool,
    #[serde(rename="isIncoming")]
    pub is_incoming: bool,
    #[serde(rename="isUploadingTo")]
    pub is_uploading_to: bool,
    #[serde(rename="peerIsChoked")]
    pub peer_is_choked: bool,
    #[serde(rename="peerIsIntrested")]
    pub peer_is_intrested: bool,
    pub port: usize,
    pub progress: f64,
    #[serde(rename="rateToClient")]
    pub rate_to_client: usize,
    #[serde(rename="rateToPeer")]
    pub rate_to_peer: usize
}

/// Contains statistics about the sources of peers.
#[derive(Deserialize, Clone, Debug)]
pub struct PeersFrom {
    #[serde(rename="fromCache")]
    pub from_cache: usize,
    #[serde(rename="fromIncoming")]
    pub from_incoming: usize,
    #[serde(rename="fromPex")]
    pub from_pex: usize,
    #[serde(rename="fromTracker")]
    pub from_tracker: usize
}

/// Contains information about a tracker.
#[derive(Deserialize, Clone, Debug)]
pub struct Tracker {
    pub announce: String,
    pub scrape: String,
    pub tier: usize
}
