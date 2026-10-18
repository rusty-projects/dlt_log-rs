mod common;

use dlt_log::init;
use regex::Regex;

#[test]
fn log_test() {
    let app_id = "TEST";
    let context_id = "INT";
    assert!(init(app_id, "Rust tests", context_id, "Integration tests").is_ok());

    log::error!("Test error");
    log::warn!("Test warn");
    // unique marker, so that messages of previous runs do not match
    let info_test_msg = format!("Test info {}", std::process::id());
    let info_log_line_number = line!() + 1;
    log::info!("{}", info_test_msg);
    log::debug!("Test debug");
    log::trace!("Test trace");

    // get data, skip the check if the daemon is not running
    if let Some(stdout) = common::dlt_receive() {
        // 2025/12/11 17:50:20.681606   43991828 000 ECU1 TEST INT- log info V 1 [[log.rs:16] Test info 12345]
        let pattern = format!(
            r"{} {}.*info.*\[log\.rs:{}\] {}",
            app_id,
            context_id,
            info_log_line_number,
            regex::escape(&info_test_msg)
        );
        assert!(
            Regex::new(&pattern).unwrap().is_match(&stdout),
            "info message not found in stdout"
        );
    }
}
