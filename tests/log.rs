use dlt_log::init;
use regex::Regex;
use std::process::Command;

#[test]
fn log_test() {
    let app_id = "TEST";
    let context_id = "INT";
    assert!(init(app_id, "Rust tests", context_id, "Integration tests").is_ok());

    log::error!("Test error");
    log::warn!("Test warn");
    let info_test_msg = "Test info";
    let info_log_line_number = line!() + 1;
    log::info!("{}", info_test_msg);
    log::debug!("Test debug");
    log::trace!("Test trace");

    // no panic when providing invalid c-string
    // let x: [u8; 5] = [0, b'T', b'E', b'S', b'T'];
    //let str_with_null: &str = std::str::from_utf8(&x).unwrap();
    //log::info!("{}", str_with_null);
    //    2025/12/11 17:50:20.681635   43991829 003 ECU1 TEST INT- log info V 1 [ERROR: NulError when converting log message from Rust to C.]

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

        // 2025/12/11 17:50:20.681606   43991828 000 ECU1 TEST INT- log info V 1 [[log.rs:13] Test info]
        let pattern = format!(
            r"{} {}.*info.*\[log\.rs:{}\] {}",
            app_id,
            context_id,
            info_log_line_number,
            regex::escape(info_test_msg)
        );
        assert!(
            Regex::new(&pattern).unwrap().is_match(&stdout),
            "info message not found in stdout"
        );
    }
}
