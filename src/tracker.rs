use std::net::{Ipv4Addr, SocketAddrV4};
use serde::{Deserialize, Deserializer};
use serde_bencode::from_bytes;
use serde::de::{Error as DeError};

#[derive(Debug, Clone, Deserialize)]
pub struct TrackerResponse {
    /// An integer, indicating how often client should make a request to the tracker in seconds.
    pub interval: i64,
    /// A ist of peers that client can connect to.
    #[serde(deserialize_with = "deserialize_peers")]
    pub peers: Vec<SocketAddrV4>,
}

impl TrackerResponse {
    pub fn parse(content: &[u8]) -> Self {
        from_bytes(content)
            .unwrap_or_else(|e| panic!("Failed to decode the response {:?}: {}", content, e))
    }
}

fn deserialize_peers<'de, D>(deserializer: D) -> Result<Vec<SocketAddrV4>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: serde_bytes::ByteBuf = Deserialize::deserialize(deserializer)?;
    if bytes.len() % 6 != 0 {
        return Err(DeError::custom(format!(
            "peers bytes length {} is not a multiple of 6",
            bytes.len()
        )));
    }

    let mut out = Vec::with_capacity(bytes.len() / 6);
    for chunk in bytes.chunks(6) {
        let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
        let port = u16::from_be_bytes([chunk[4], chunk[5]]);
        out.push(SocketAddrV4::new(ip, port));
    }
    Ok(out)
}