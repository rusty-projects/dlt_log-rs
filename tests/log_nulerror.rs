use dlt_log::init;
use regex::Regex;
use std::process::Command;

#[test]
fn log_test_nulerror() {
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_ok());

    // no panic when providing invalid c-string, NUL byte is escaped
    let x: [u8; 5] = [0, b'T', b'E', b'S', b'T'];
    let str_with_null: &str = std::str::from_utf8(&x).unwrap();
    // unique marker, so that messages of previous runs do not match
    let marker = std::process::id();
    log::info!("before {} after {}", str_with_null, marker);

    // get data
    let dlt_receive = Command::new("timeout")
        .arg("1")
        .arg("dlt-receive")
        .arg("-a")
        .arg("localhost")
        .output()
        .expect("failed to execute dlt-receive");

    // dlt running?
    if !String::from_utf8_lossy(&dlt_receive.stderr).contains("failed to connect") {
        let stdout = String::from_utf8_lossy(&dlt_receive.stdout);
        let pattern = format!(r"\] before \\0TEST after {}\]", marker);
        assert!(
            Regex::new(&pattern).unwrap().is_match(&stdout),
            "escaped message not found in stdout"
        );
    }
}
