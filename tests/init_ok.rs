use dlt_log::init;

#[test]
fn init_ok() {
    assert!(init("APP", "Application").is_ok());
}
