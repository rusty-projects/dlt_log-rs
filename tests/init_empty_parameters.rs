use dlt_log::init;

#[test]
fn init_empty_parameters() {
    assert!(init("", "Application").is_err());
    // empty description is ok
    assert!(init("APP", "").is_ok());
}
