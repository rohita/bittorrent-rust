use assert_cmd::Command;
use tempfile::NamedTempFile;
use std::io::Write;
use indoc::indoc;

pub const NO_ERROR: &str = "";
pub const SUCCESS: i32 = 0;


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