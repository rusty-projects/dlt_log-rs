use dlt_log::init;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Counts how often it is formatted.
struct CountFormat<'a>(&'a AtomicUsize);

impl fmt::Display for CountFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fetch_add(1, Ordering::Relaxed);
        write!(f, "counted")
    }
}

#[test]
fn log_level_test() {
    // use the default log level `info` of DLT, with and without daemon
    std::env::remove_var("DLT_INITIAL_LOG_LEVEL");
    assert!(init("TEST", "Rust tests", "INT", "Integration tests").is_ok());

    assert!(log::log_enabled!(log::Level::Error));
    assert!(log::log_enabled!(log::Level::Warn));
    assert!(log::log_enabled!(log::Level::Info));
    assert!(!log::log_enabled!(log::Level::Debug));
    assert!(!log::log_enabled!(log::Level::Trace));

    // messages of disabled levels are not formatted
    let count = AtomicUsize::new(0);
    log::trace!("{}", CountFormat(&count));
    log::debug!("{}", CountFormat(&count));
    assert_eq!(count.load(Ordering::Relaxed), 0);

    // messages of enabled levels are formatted
    log::info!("{}", CountFormat(&count));
    assert_eq!(count.load(Ordering::Relaxed), 1);
}
