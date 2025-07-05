use assert_cmd::Command;
use tempfile::NamedTempFile;
use std::io::Write;
use indoc::indoc;

pub const NO_ERROR: &str = "";
pub const SUCCESS: i32 = 0;

#[test] fn decode_string() {
    run_decode("4:pear", "\"pear\"\n");
    run_decode("9:raspberry", "\"raspberry\"\n");
    run_decode("55:http://bittorrent-test-tracker.codecrafters.io/announce",
               "\"http://bittorrent-test-tracker.codecrafters.io/announce\"\n");
}
#[test] fn decode_integer() {
    run_decode("i1052617151e", "1052617151\n");
    run_decode("i-52e", "-52\n");
}
#[test] fn decode_list() {
    run_decode("le", "[]\n");
    run_decode("l9:pineapplei317ee", "[\"pineapple\",317]\n");
    run_decode("lli317e9:pineappleee", "[[317,\"pineapple\"]]\n");
    run_decode("lli4eei5ee", "[[4],5]\n");
}
#[test] fn decode_dictionary() {
    run_decode("de", "{}\n");
    run_decode("d3:foo5:apple5:helloi52ee", "{\"foo\":\"apple\",\"hello\":52}\n");
    run_decode("d10:inner_dictd4:key16:value14:key2i42e8:list_keyl5:item15:item2i3eeee",
               "{\"inner_dict\":{\"key1\":\"value1\",\"key2\":42,\"list_key\":[\"item1\",\"item2\",3]}}\n");
}

#[test] fn decode_torrent_file() {
    let input = b"d8:announce11:http://x.y/4:infod6:lengthi5e4:name4:tiny12:piece lengthi16384e6:pieces2:\0\0ee";
    let expected2 = indoc! {"
        Tracker URL: http://x.y/
        Length: 5
    "};
    run_info(input, expected2);
}

pub fn run_decode(input: &str, expected: &str) {
    let mut cmd = Command::cargo_bin("codecrafters-bittorrent").expect("Binary not found");
    cmd.args(&["decode", input]);
    let output = cmd.output().expect("Failed to run binary");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    assert_eq!(stdout, expected);
}

pub fn run_info(input: &[u8], expected: &str) {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(input).expect("Failed to write to temp file");

    let mut cmd = Command::cargo_bin("codecrafters-bittorrent").expect("Binary not found");
    cmd.args(&["info", temp_file.path().to_str().unwrap()]);

    let output = cmd.output().expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let exit_code = output.status.code().unwrap_or(-1);

    eprintln!("stdout: {}", stdout);
    eprintln!("stderr: {}", stderr);
    eprintln!("exit code: {}", exit_code);
    assert_eq!(stdout, expected);
}