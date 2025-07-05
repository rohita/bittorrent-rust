use serde::{Deserialize, Serialize};
use serde_bencode::{from_bytes, to_bytes};
use sha1::{Digest, Sha1};

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

pub fn parse_torrent(content: &[u8]) -> Torrent {
    from_bytes(content).expect("Failed to decode the torrent file.")
}

pub fn get_info_hash(torrent: &Torrent) -> String {
    let mut hasher = Sha1::new();
    let info_bytes = to_bytes(&torrent.info).expect("Failed to serialize file info");
    hasher.update(info_bytes);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_torrent_file() {
        let input = serde_bencode::to_string(&get_test_torrent()).expect("Failed to convert torrent file to string.");
        println!("Torrent file content:\n{:?}", input);
        let torrent = parse_torrent(input.as_bytes());

        assert_eq!(torrent.announce, "http://x.y/");
        assert_eq!(torrent.info.length, 5);
    }

    #[test]
    fn test_info_hash() {
        assert_eq!("13f82ce4d395db43d8a33821ab8af306b13f9446", get_info_hash(&get_test_torrent()));
    }

    fn get_test_torrent() -> Torrent {
        let info = TorrentInfo {
            length: 5,
            name: "tiny".to_string(),
            piece_length: 16384,
            pieces: serde_bytes::ByteBuf::from(vec![0u8; 2]),
        };
        Torrent {
            announce: "http://x.y/".to_string(),
            info: info,
        }
    }
}

