use dlt_log::{init, InitializeError};
use regex::Regex;
use std::process::Command;

#[test]
fn init_double() {
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_ok());
    assert!(matches!(
        init("TEST", "Rust tests", "INT", "Integration tests"),
        Err(InitializeError::LoggerError(_))
    ));
    // a failed init with different IDs must not change the running logger
    assert!(matches!(
        init("TST2", "Rust tests", "INT2", "Integration tests"),
        Err(InitializeError::LoggerError(_))
    ));

    // unique marker, so that messages of previous runs do not match
    let marker = format!("double init marker {}", std::process::id());
    log::info!("{}", marker);

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
        let pattern = format!(r"TEST INT.*info.*{}", regex::escape(&marker));
        assert!(
            Regex::new(&pattern).unwrap().is_match(&stdout),
            "message not found with IDs of first init in stdout"
        );
    }
}
