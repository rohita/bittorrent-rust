use std::net::SocketAddrV4;
use rand::{distr::Alphanumeric, Rng};
use url::Url;
use crate::torrent::Torrent;
use crate::tracker::TrackerResponse;

pub struct Client {
    id: String,
    port: u16,
}

impl Client {
    pub fn default() -> Client {
        Self::new(generate_peer_id(), 6881)
    }

    pub fn new(id: String, port: u16) -> Self {
        if id.len() > 20 {
            panic!("ID must be less than 20 characters");
        }
        Client { id, port }
    }

    pub fn get_peers(&self, torrent: &Torrent) -> Vec<SocketAddrV4> {
        let url = build_tracker_url(
            &torrent.announce,
            &torrent.get_info_hash(),
            &self.id,
            self.port,
            0,
            0,
            torrent.info.length,
        );

        let body = reqwest::blocking::get(url).unwrap().bytes().unwrap();
        let tracker_response = TrackerResponse::parse(&body);
        tracker_response.peers
    }
}

fn generate_peer_id() -> String {
    let rng = rand::rng();
    let peer_id: String = rng
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();

    peer_id
}

fn build_tracker_url(
    announce: &str,
    info_hash: &[u8],
    peer_id: &str,
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: i64,
) -> String {
    let mut url = Url::parse(announce)
        .unwrap_or_else(|e| panic!("Failed to parse tracker url {:?}: {}", announce, e));

    // Manually URL encode the info_hash bytes to avoid double encoding
    let info_hash_encoded = info_hash
        .iter()
        .map(|&b| format!("%{:02x}", b))
        .collect::<String>();

    // Build other query parameters normally
    url.query_pairs_mut()
        .append_pair("peer_id", peer_id)
        .append_pair("port", &port.to_string())
        .append_pair("uploaded", &uploaded.to_string())
        .append_pair("downloaded", &downloaded.to_string())
        .append_pair("left", &left.to_string())
        .append_pair("compact", "1");

    // Manually add the info_hash to avoid double encoding
    let mut final_url = url.to_string();
    let separator = if final_url.contains('?') { '&' } else { '?' };
    final_url.push_str(&format!("{}info_hash={}", separator, info_hash_encoded));

    final_url
}