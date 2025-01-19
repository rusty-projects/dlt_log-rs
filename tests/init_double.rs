use dlt_log::init;

#[test]
fn init_double() {
    assert!(init("APP4", "Application").is_ok());
    assert!(init("APP4", "Application").is_err());
}
