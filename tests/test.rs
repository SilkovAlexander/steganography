use assert_cmd::Command;
use predicates::prelude::*;
use file_diff::diff;

const BIN_NAME: &str = "stegoimage";

const DATA_PATH: &str = "tests/resources/data.txt";
const IMAGE_PATH: &str = "tests/resources/image.png";
const ENCODED_PATH: &str = "tests/resources/result.png";
const DECODED_PATH: &str = "tests/resources/decoded.txt";


#[test]
fn test_enc_dec() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("encode")
        .arg(IMAGE_PATH)
        .arg(DATA_PATH)
        .assert()
        .success()
        .stdout(predicate::str::contains("The program succeeded."));

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("decode")
        .arg(ENCODED_PATH)
        .arg(DECODED_PATH)
        .assert()
        .success()
        .stdout(predicate::str::contains("The program succeeded."));

    assert!(diff(DATA_PATH, DECODED_PATH));

    Ok(())
}