use serde::{Deserialize, Serialize};
use serde_bencode::from_bytes;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Torrent {
    /// URL to a "tracker", which is a central server that keeps
    /// track of peers participating in the sharing of a torrent.
    pub announce: String,
    /// A dictionary with keys
    pub info: TorrentInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

fn print_test() {
    let info = TorrentInfo {
        length: 5,
        name: "tiny".to_string(),
        piece_length: 16384,
        pieces: serde_bytes::ByteBuf::from(vec![0u8; 2]),
    };
    let torrent = Torrent {
        announce: "http://x.y/".to_string(),
        info: info,
    };
    let printer = serde_bencode::to_string(&torrent).expect("Failed to convert torrent file to string.");
    eprintln!("Torrent file content:\n{:?}", printer);
}

