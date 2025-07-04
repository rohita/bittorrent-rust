use serde::Deserialize;
use serde_bencode::from_bytes;

#[derive(Deserialize, Clone)]
pub struct Torrent {
    /// URL to a "tracker", which is a central server that keeps
    /// track of peers participating in the sharing of a torrent.
    pub announce: String,
    /// A dictionary with keys
    pub info: TorrentInfo,
}

#[derive(Deserialize, Clone)]
pub struct TorrentInfo {
    /// Size of the file in bytes, for single-file torrents
    pub length: i64,
    /// Suggested name to save the file / directory as
    pub name: String,
    /// Number of bytes in each piece
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    /// Concatenated SHA-1 hashes of each piece
    pub pieces: serde_bytes::ByteBuf,
}

pub fn parse_torrent_file(file_path: &str) -> Torrent {
    let content = std::fs::read(file_path).expect("Cannot read torrent file.");
    from_bytes(content.as_slice()).expect("Failed to decode the torrent file.")
}

