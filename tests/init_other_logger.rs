use dlt_log::{init, InitializeError};

struct OtherLogger;

impl log::Log for OtherLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, _record: &log::Record) {}

    fn flush(&self) {}
}

static OTHER_LOGGER: OtherLogger = OtherLogger;

#[test]
fn init_other_logger() {
    log::set_logger(&OTHER_LOGGER).unwrap();

    // DLT registration succeeds, but setting the logger fails and the registration is undone
    assert!(matches!(
        init("TEST", "Rust tests", "INT", "Integration tests"),
        Err(InitializeError::LoggerError(_))
    ));
    // no DLT state is left behind, so a retry fails the same way
    assert!(matches!(
        init("TEST", "Rust tests", "INT", "Integration tests"),
        Err(InitializeError::LoggerError(_))
    ));
}
