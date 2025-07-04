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
        let torrent = torrent::parse_torrent_file(file_name);
        println!("Tracker URL: {}", torrent.announce);
        println!("Length: {}", torrent.info.length);
    } else {
        println!("unknown command: {}", args[1])
    }
}

