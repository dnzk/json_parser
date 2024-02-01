use assert_cmd::Command;

#[test]
fn step1_valid() {
    let mut cmd = Command::cargo_bin("json_parser").unwrap();
    cmd.arg("./test_files/step1/valid.json")
        .assert()
        .stdout("JSON is valid\n");
}

#[test]
fn step1_invalid() {
    let mut cmd = Command::cargo_bin("json_parser").unwrap();
    cmd.arg("./test_files/step1/invalid.json")
        .assert()
        .stderr("JSON is invalid\n");
}
