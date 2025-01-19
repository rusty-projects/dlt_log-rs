use dlt_log::init;

#[test]
fn init_nulerror() {
    let x: [u8; 4] = [0, b'A', b'P', b'P'];
    let str_with_null: &str = std::str::from_utf8(&x).unwrap();
    assert!(init(str_with_null, "Application").is_err());
    assert!(init("APP3", str_with_null).is_err());
}
