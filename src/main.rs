mod decode;
mod torrent;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode::decode_bencoded_value(encoded_value).0;
        println!("{}", decoded_value);
    } else if command == "info" {
        let file_name = &args[2];
        let content = std::fs::read(file_name).expect("Cannot read torrent file.");
        let torrent = torrent::parse_torrent(content.as_slice());

        println!("Tracker URL: {}", torrent.announce);
        println!("Length: {}", torrent.info.length);
        println!("Info Hash: {}", torrent.get_info_hash());
        println!("Piece Length: {}", torrent.info.piece_length);
        for piece_hash in torrent.get_piece_hashes() {
            println!("{piece_hash}");
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}

