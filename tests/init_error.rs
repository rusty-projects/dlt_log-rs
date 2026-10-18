use dlt_log::{init, DltReturnValue, InitializeError};
use std::error::Error;

struct OtherLogger;

impl log::Log for OtherLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, _record: &log::Record) {}

    fn flush(&self) {}
}

static OTHER_LOGGER: OtherLogger = OtherLogger;

/// Initializes and returns the error as boxed trait object, as an application would use it.
fn init_boxed(app_id: &str) -> Result<(), Box<dyn Error>> {
    init(app_id, "Rust tests", "INT", "Integration tests")?;
    Ok(())
}

#[test]
fn init_error() {
    // conversion error
    let err = init_boxed("A\0PP").unwrap_err();
    assert_eq!(err.to_string(), "argument contains an interior null byte");
    assert!(err.source().unwrap().is::<std::ffi::NulError>());

    // logger error
    log::set_logger(&OTHER_LOGGER).unwrap();
    let err = init_boxed("TEST").unwrap_err();
    assert_eq!(err.to_string(), "failed to set the logger");
    assert!(err.source().unwrap().is::<log::SetLoggerError>());

    // DLT library error, which cannot be provoked reliably
    let err = InitializeError::DltLibraryError(DltReturnValue::DLT_RETURN_ERROR);
    assert_eq!(
        err.to_string(),
        "DLT library returned an error: DLT_RETURN_ERROR"
    );
    assert!(err.source().is_none());
}
