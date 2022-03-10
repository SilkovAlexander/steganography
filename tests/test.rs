use assert_cmd::Command;
use predicates::prelude::*;
use file_diff::diff;

const BIN_NAME: &str = "stegoimage";

const DATA_PATH: &str = "tests/resources/data.txt";
const TEST_IMAGES: &'static [&'static str] = &[
    "tests/resources/image.png",
    "tests/resources/image2.jpg",
    "tests/resources/image3_bw.jpg"
];
const ENCODED_PATH: &str = "tests/resources/result.png";
const DECODED_PATH: &str = "tests/resources/decoded.txt";


#[test]
fn test_enc_dec() -> Result<(), Box<dyn std::error::Error>> {
    for name in TEST_IMAGES {
        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.arg("encode")
            .arg(name)
            .arg(DATA_PATH)
            .arg(ENCODED_PATH)
            .assert()
            .success()
            .stdout(predicate::str::contains("Success."));

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.arg("decode")
            .arg(ENCODED_PATH)
            .arg(DECODED_PATH)
            .assert()
            .success()
            .stdout(predicate::str::contains("Success."));
    }
    assert!(diff(DATA_PATH, DECODED_PATH));
    std::fs::remove_file(ENCODED_PATH)?;
    Ok(())
}