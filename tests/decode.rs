use assert_cmd::Command;

pub const NO_ERROR: &str = "";
pub const SUCCESS: i32 = 0;

#[test]
fn decode_string() {
    run_decode("4:pear", "\"pear\"\n", NO_ERROR, SUCCESS);
    run_decode("9:raspberry", "\"raspberry\"\n", NO_ERROR, SUCCESS);
}
#[test]
fn decode_integer() {
    run_decode("i1052617151e", "1052617151\n", NO_ERROR, SUCCESS);
    run_decode("i-52e", "-52\n", NO_ERROR, SUCCESS);
}

pub fn run_decode(
    input: &str,
    expected: &str,
    expected_error: &str,
    expected_code: i32)
{
    let mut cmd = Command::cargo_bin("codecrafters-bittorrent").expect("Binary not found");
    cmd.args(&["decode", input]);
    let output = cmd.output().expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(stdout, expected);
    assert_eq!(stderr, expected_error);
    assert_eq!(exit_code, expected_code);
}