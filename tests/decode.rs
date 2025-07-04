use assert_cmd::Command;

#[test]
fn decode_string() {
    let input = "4:pear";
    let expected = "\"pear\"\n";
    let mut cmd = Command::cargo_bin("codecrafters-bittorrent").expect("Binary not found");
    cmd.args(&["decode", input]);
    let output = cmd.output().expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    //let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    //let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(stdout, expected);
    //assert_eq!(stderr, expected_error);
    //assert_eq!(exit_code, expected_code);
}