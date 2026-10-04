use dlt_log::init;
use regex::Regex;
use std::process::Command;

#[test]
fn log_test_nulerror() {
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_ok());

    // no panic when providing invalid c-string, NUL byte is escaped
    let x: [u8; 5] = [0, b'T', b'E', b'S', b'T'];
    let str_with_null: &str = std::str::from_utf8(&x).unwrap();
    log::info!("before {} after", str_with_null);

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
        assert!(
            Regex::new(r"\] before \\0TEST after\]")
                .unwrap()
                .is_match(&stdout),
            "escaped message not found in stdout"
        );
    }
}
