/// Enum that represents the torrent's current state
#[derive(Clone, Debug)]
pub enum Status {
    /// Torrent is stopped
    Stopped,
    /// Torrent is queued to check files
    CheckWait,
    /// Torrent's files are being checked
    Check,
    /// Torrent is queued to start downloading
    DownloadWait,
    /// Torrent is being downloaded
    Download,
    /// Torrent is queued to start seeding
    SeedWait,
    /// Torrent is being seeded
    Seed
}

impl_enum_serde! {
    Status {
        0 => Status::Stopped,
        1 => Status::CheckWait,
        2 => Status::Check,
        3 => Status::DownloadWait,
        4 => Status::Download,
        5 => Status::SeedWait,
        6 => Status::Seed
    }
}
