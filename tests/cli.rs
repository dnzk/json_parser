use assert_cmd::Command;

#[test]
fn step1_valid() {
    let mut cmd = Command::cargo_bin("json_parser").unwrap();
    cmd.arg("./test_files/step1/valid.json").assert().code(0);
}

#[test]
fn step1_invalid() {
    let mut cmd = Command::cargo_bin("json_parser").unwrap();
    cmd.arg("./test_files/step1/invalid.json").assert().code(1);
}

#[test]
fn step2_valid() {
    let mut cmd = Command::cargo_bin("json_parser").unwrap();
    cmd.arg("./test_files/step2/valid.json").assert().code(0);
}

#[test]
fn step2_valid2() {
    let mut cmd = Command::cargo_bin("json_parser").unwrap();
    cmd.arg("./test_files/step2/valid2.json").assert().code(0);
}

#[test]
fn step2_invalid() {
    let mut cmd = Command::cargo_bin("json_parser").unwrap();
    cmd.arg("./test_files/step2/invalid.json").assert().code(1);
}

#[test]
fn step2_invalid2() {
    let mut cmd = Command::cargo_bin("json_parser").unwrap();
    cmd.arg("./test_files/step2/invalid2.json").assert().code(1);
}