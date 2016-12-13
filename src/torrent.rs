use types::{Status, Priority};
use chrono::NaiveDateTime;
use ::types::time_t::deserialize_time_t_option;

/// A struct to hold the torrent information returned from the daemon.
/// Not all fields are usually populated.
#[derive(Deserialize, Clone, Debug)]
pub struct Torrent {
    #[serde(rename="activityDate", deserialize_with="deserialize_time_t_option", default)]
    pub activity_date: Option<NaiveDateTime>, //time_t
    #[serde(rename="addedDate", deserialize_with="deserialize_time_t_option", default)]
    pub added_date: Option<NaiveDateTime>, //time_t
    #[serde(rename="announceResponse")]
    pub announce_response: Option<String>,
    #[serde(rename="announceUrl")]
    pub announce_url: Option<String>,
    #[serde(rename="bandwidthPriority")]
    pub bandwidth_priority: Option<Priority>,
    pub comment: Option<String>,
    #[serde(rename="corruptEver")]
    pub corrupt_ever: Option<u64>,
    pub creator: Option<String>,
    #[serde(rename="dateCreated", deserialize_with="deserialize_time_t_option", default)]
    pub date_created: Option<NaiveDateTime>, //time_t
    #[serde(rename="desiredAvailable")]
    pub desired_available: Option<u64>,
    #[serde(rename="doneDate", deserialize_with="deserialize_time_t_option", default)]
    pub done_date: Option<NaiveDateTime>, //time_t
    #[serde(rename="downloadDir")]
    pub download_dir: Option<String>,
    #[serde(rename="downloadedEver")]
    pub downloaded_ever: Option<u64>,
    pub downloaders: Option<u64>,
    #[serde(rename="downloadLimit")]
    pub download_limit: Option<u64>,
    #[serde(rename="downloadLimited")]
    pub download_limited: Option<bool>,
    pub error: Option<u64>,
    #[serde(rename="errorString")]
    pub error_string: Option<String>,
    pub eta: Option<NaiveDateTime>, //time_t
    pub files: Option<Vec<File>>,
    pub file_stats: Option<Vec<FileStat>>,
    #[serde(rename="hashString")]
    pub hash_string: Option<String>,
    #[serde(rename="haveUnchecked")]
    pub have_unchecked: Option<u64>,
    #[serde(rename="haveValid")]
    pub have_valid: Option<u64>,
    #[serde(rename="honorsSessionLimits")]
    pub honors_session_limits: Option<bool>,
    pub id: Option<u64>,
    #[serde(rename="isPrivate")]
    pub is_private: Option<bool>,
    #[serde(rename="lastAnnounceTime", deserialize_with="deserialize_time_t_option", default)]
    pub last_announce_time: Option<NaiveDateTime>, //time_t
    #[serde(rename="lastScrapeTime")]
    pub last_scrape_time: Option<u64>,
    pub leechers: Option<u64>,
    #[serde(rename="leftUntilDone")]
    pub left_until_done: Option<u64>,
    #[serde(rename="manualAnnounceTime", deserialize_with="deserialize_time_t_option", default)]
    pub manual_announce_time: Option<NaiveDateTime>, //time_t
    #[serde(rename="maxConnectedPeers")]
    pub max_connected_peers: Option<u64>,
    pub name: Option<String>,
    #[serde(rename="nextAnnounceTime", deserialize_with="deserialize_time_t_option", default)]
    pub next_announce_time: Option<NaiveDateTime>, //time_t
    #[serde(rename="nextScrapeTime", deserialize_with="deserialize_time_t_option", default)]
    pub next_scrape_time: Option<NaiveDateTime>, //time_t
    #[serde(rename="peerLimit")]
    pub peer_limit: Option<u64>,
    pub peers: Option<Vec<Peer>>,
    #[serde(rename="peersConnected")]
    pub peers_connected: Option<u64>,
    #[serde(rename="peersFrom")]
    pub peers_from: Option<PeersFrom>,
    #[serde(rename="peersGettingFromUs")]
    pub peers_getting_from_us: Option<u64>,
    #[serde(rename="peersKnown")]
    pub peers_known: Option<u64>,
    #[serde(rename="peersSendingToUs")]
    pub peers_sending_to_us: Option<u64>,
    #[serde(rename="percentDone")]
    pub percent_done: Option<f64>,
    pub pieces: Option<String>,
    #[serde(rename="pieceCount")]
    pub piece_count: Option<u64>,
    #[serde(rename="pieceSize")]
    pub piece_size: Option<u64>,
    pub priorities: Option<Vec<Priority>>,
    #[serde(rename="rateDownload")]
    pub rate_download: Option<u64>,
    #[serde(rename="rateUpload")]
    pub rate_upload: Option<u64>,
    #[serde(rename="recheckProgress")]
    pub recheck_progress: Option<f64>,
    #[serde(rename="scrapeResponse")]
    pub scrape_response: Option<String>,
    #[serde(rename="scrapeUrl")]
    pub scrape_url: Option<String>,
    pub seeders: Option<u64>,
    #[serde(rename="seedRatioLimit")]
    pub seed_ratio_limit: Option<f64>,
    #[serde(rename="seedRatioMode")]
    pub seed_ratio_mode: Option<u64>,
    #[serde(rename="sizeWhenDone")]
    pub size_when_done: Option<u64>,
    #[serde(rename="startDate", deserialize_with="deserialize_time_t_option", default)]
    pub start_date: Option<NaiveDateTime>, //time_t
    pub status: Option<Status>,
    #[serde(rename="swarmSpeed")]
    pub swarm_speed: Option<u64>,
    #[serde(rename="timesCompleted")]
    pub times_completed: Option<u64>,
    pub trackers: Option<Vec<Tracker>>,
    #[serde(rename="totalSize")]
    pub total_size: Option<u64>,
    #[serde(rename="torrentFile")]
    pub torrent_file: Option<String>,
    #[serde(rename="uploadedEver")]
    pub uploaded_ever: Option<u64>,
    #[serde(rename="uploadLimit")]
    pub upload_limit: Option<u64>,
    #[serde(rename="uploadLimited")]
    pub upload_limited: Option<bool>,
    #[serde(rename="uploadRatio")]
    pub upload_ratio: Option<f64>,
    pub wanted: Option<Vec<bool>>,
    pub webseeds: Option<Vec<String>>,
    #[serde(rename="webseedsSendingToUs")]
    pub webseeds_sending_to_us: Option<u64>,
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

impl Torrent {
    /// Creates an instace of `Torrent` where all fields are set to `None`
    pub fn new() -> Torrent {
        Torrent {        
            activity_date: None,
            added_date: None,
            announce_response: None,
            announce_url: None,
            bandwidth_priority: None,
            comment: None,
            corrupt_ever: None,
            creator: None,
            date_created: None,
            desired_available: None,
            done_date: None,
            download_dir: None,
            downloaded_ever: None,
            downloaders: None,
            download_limit: None,
            download_limited: None,
            error: None,
            error_string: None,
            eta: None,
            files: None,
            file_stats: None,
            hash_string: None,
            have_unchecked: None,
            have_valid: None,
            honors_session_limits: None,
            id: None,
            is_private: None,
            last_announce_time: None,
            last_scrape_time: None,
            leechers: None,
            left_until_done: None,
            manual_announce_time: None,
            max_connected_peers: None,
            name: None,
            next_announce_time: None,
            next_scrape_time: None,
            peer_limit: None,
            peers: None,
            peers_connected: None,
            peers_from: None,
            peers_getting_from_us: None,
            peers_known: None,
            peers_sending_to_us: None,
            percent_done: None,
            pieces: None,
            piece_count: None,
            piece_size: None,
            priorities: None,
            rate_download: None,
            rate_upload: None,
            recheck_progress: None,
            scrape_response: None,
            scrape_url: None,
            seeders: None,
            seed_ratio_limit: None,
            seed_ratio_mode: None,
            size_when_done: None,
            start_date: None,
            status: None,
            swarm_speed: None,
            times_completed: None,
            trackers: None,
            total_size: None,
            torrent_file: None,
            uploaded_ever: None,
            upload_limit: None,
            upload_limited: None,
            upload_ratio: None,
            wanted: None,
            webseeds: None,
            webseeds_sending_to_us: None,
        }
    }
}
