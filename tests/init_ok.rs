use dlt_log::init;

#[test]
fn init_ok() {
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_ok());
}
