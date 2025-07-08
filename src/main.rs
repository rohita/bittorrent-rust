mod decode;
mod torrent;
mod tracker;
mod client;

use std::env;
use torrent::Torrent;
use crate::client::Client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            let encoded_value = &args[2];
            let decoded_value = decode::decode_bencoded_value(encoded_value).0;
            println!("{}", decoded_value);
        }
        "info" => {
            let file_name = &args[2];
            let content = std::fs::read(file_name).expect("Cannot read torrent file.");
            let torrent = Torrent::parse(&content);
            println!("{}", torrent);
        }
        "peers" => {
            let file_name = &args[2];
            let content = std::fs::read(file_name).expect("Cannot read torrent file.");
            let torrent = Torrent::parse(&content);
            let client = Client::default();
            let peers = client.get_peers(&torrent);
            for peer in peers {
                println!("{}:{}", peer.ip(), peer.port());
            }
        }
        _ => {
            println!("unknown command: {}", args[1]);
        }
    }
}

