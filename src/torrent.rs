use std::fmt::Display;
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

impl Torrent {
    pub fn parse(content: &[u8]) -> Self {
        from_bytes(content).expect("Failed to decode the torrent file.")
    }

    /// Computes the SHA-1 hash of the bencoded `info` dictionary.
    /// Info hash is a unique identifier for a torrent file.
    /// It's used when talking to trackers or peers.
    pub fn get_info_hash(&self) -> [u8; 20] {
        let mut hasher = Sha1::new();
        let info_bytes = to_bytes(&self.info).expect("Failed to serialize file info");
        hasher.update(info_bytes);
        hasher.finalize().into()
    }

    /// Splits the concatenated SHA1 hashes into 20-byte chunks (as hex strings)
    pub fn get_piece_hashes(&self) -> Vec<String> {
        self.info
            .pieces
            .chunks(20)
            .map(hex::encode)
            .collect()
    }
}

impl Display for Torrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tracker URL: {}", self.announce)?;
        writeln!(f, "Length: {}", self.info.length)?;
        writeln!(f, "Info Hash: {}", hex::encode(self.get_info_hash()))?;
        writeln!(f, "Piece Length: {}", self.info.piece_length)?;
        for hash in self.get_piece_hashes() {
            writeln!(f, "{}", hash)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_torrent_file() {
        let input = serde_bencode::to_string(&get_test_torrent()).expect("Failed to convert torrent file to string.");
        println!("Torrent file content:\n{:?}", input);
        let torrent = Torrent::parse(input.as_bytes());

        assert_eq!("http://x.y/", torrent.announce);
        assert_eq!(5, torrent.info.length);
        assert_eq!("4bb37347b2131c296769c35615688a5ef7197935", hex::encode(torrent.get_info_hash()));
        assert_eq!(16384, torrent.info.piece_length);
        for piece_hash in torrent.get_piece_hashes() {
            println!("{}", hex::encode(piece_hash));
        }
    }

    fn get_test_torrent() -> Torrent {
        let info = TorrentInfo {
            length: 5,
            name: "tiny".to_string(),
            piece_length: 16384,
            pieces: serde_bytes::ByteBuf::from("abc".as_bytes().to_vec()),
        };
        Torrent {
            announce: "http://x.y/".to_string(),
            info: info,
        }
    }
}

