use dlt_log::init;

#[test]
fn init_nulerror() {
    let x: [u8; 4] = [0, b'A', b'P', b'P'];
    let str_with_null: &str = std::str::from_utf8(&x).unwrap();
    assert!(init(str_with_null, "Rust tests", "INT", "Integration tests").is_err());
    assert!(init("TEST", str_with_null, "INT", "Integration tests").is_err());
    assert!(init("TEST", "Rust tests", str_with_null, "Integration tests").is_err());
    assert!(init("TEST", "Rust tests", "INT", str_with_null).is_err());
}
