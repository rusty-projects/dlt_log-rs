use std::process::Command;

/// Environment variable that makes tests fail if the DLT daemon cannot be reached.
const REQUIRE_DAEMON_VAR: &str = "DLT_LOG_REQUIRE_DAEMON";

/// Receives messages from the DLT daemon for one second and returns the output of `dlt-receive`.
///
/// Returns `None` if the daemon cannot be reached, so that tests can skip checking the messages.
/// Panics instead if `DLT_LOG_REQUIRE_DAEMON=1` is set, so that a missing daemon does not go unnoticed in CI.
pub fn dlt_receive() -> Option<String> {
    let dlt_receive = Command::new("timeout")
        .arg("1")
        .arg("dlt-receive")
        .arg("-a")
        .arg("localhost")
        .output()
        .expect("failed to execute dlt-receive");

    if String::from_utf8_lossy(&dlt_receive.stderr).contains("failed to connect") {
        assert!(
            std::env::var(REQUIRE_DAEMON_VAR).as_deref() != Ok("1"),
            "dlt-receive failed to connect to the DLT daemon, but {REQUIRE_DAEMON_VAR}=1 is set"
        );
        return None;
    }
    Some(String::from_utf8_lossy(&dlt_receive.stdout).into_owned())
}
