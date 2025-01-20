use dlt_log::init;

#[test]
fn log_test() {
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_ok());
    log::error!("Test error");
    log::warn!("Test warn");
    log::info!("Test info");
    log::debug!("Test debug");
    log::trace!("Test trace");

    // no panic when providing invalid c-string
    let x: [u8; 5] = [0, b'T', b'E', b'S', b'T'];
    let str_with_null: &str = std::str::from_utf8(&x).unwrap();
    log::info!("{}", str_with_null);
}
