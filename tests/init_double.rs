use dlt_log::init;

#[test]
fn init_double() {
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_ok());
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_err());
}
