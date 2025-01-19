use dlt_log::init;

#[test]
fn init_empty_parameters() {
    assert!(init("", "Rust tests", "INT", "Integration tests").is_err());
    assert!(init("TEST", "Rust tests", "", "Integration tests").is_err());
    // empty description is ok
    assert!(init("TEST", "", "INT", "Integration tests").is_ok());
}
