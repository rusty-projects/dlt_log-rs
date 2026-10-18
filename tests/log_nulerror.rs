mod common;

use dlt_log::init;
use regex::Regex;

#[test]
fn log_test_nulerror() {
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_ok());

    // no panic when providing invalid c-string, NUL byte is escaped
    let x: [u8; 5] = [0, b'T', b'E', b'S', b'T'];
    let str_with_null: &str = std::str::from_utf8(&x).unwrap();
    // unique marker, so that messages of previous runs do not match
    let marker = std::process::id();
    log::info!("before {} after {}", str_with_null, marker);

    // get data, skip the check if the daemon is not running
    if let Some(stdout) = common::dlt_receive() {
        let pattern = format!(r"\] before \\0TEST after {}\]", marker);
        assert!(
            Regex::new(&pattern).unwrap().is_match(&stdout),
            "escaped message not found in stdout"
        );
    }
}
