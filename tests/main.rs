use assert_cmd::Command;

#[test]
fn debug_flag_single_url() {
    // Setup
    let mut command: Command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Exercise
    let assert = command.args(&["-d", "https://google.com"]);

    // Verify
    assert
        .assert()
        .stdout(predicates::str::contains("url: https://google.com"));
}
