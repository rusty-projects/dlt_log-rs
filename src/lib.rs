#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

use log::{Level, Metadata, Record, SetLoggerError};
use std::ffi::{CString, NulError};

/// this module includes the generated bindings for the DLT library
mod libdlt;

// re-export the DLT return value from the `libdlt` module for use in the `InitializeError` enum
pub use libdlt::DltReturnValue;

#[derive(Debug)]
/// Represents possible errors that can occur during initialization.
pub enum InitializeError {
    /// Represents an error that occurs when a null byte is found in a string
    /// where it is not allowed.
    ///
    /// This error is typically encountered when converting a Rust string to a C string.
    ConversionError(NulError),
    /// Represents an error that occurs when setting the logger.
    ///
    /// This error is typically encountered when there is an attempt to set a logger
    /// after one has already been set, or if there is an issue with the logger configuration.
    LoggerError(SetLoggerError),
    /// Represents an error returned by the DLT library.
    ///
    /// This variant encapsulates the `DltReturnValue` from the `libdlt` crate,
    /// which provides detailed information about the specific error encountered
    /// during DLT operations.
    DltLibraryError(DltReturnValue),
}

impl From<NulError> for InitializeError {
    fn from(err: NulError) -> InitializeError {
        InitializeError::ConversionError(err)
    }
}

impl From<SetLoggerError> for InitializeError {
    fn from(err: SetLoggerError) -> InitializeError {
        InitializeError::LoggerError(err)
    }
}

/// Initializes the DLT logging system with the provided application and context information.
///
/// This function registers the application and context with the DLT daemon and sets up the logger
/// to send log messages to the DLT system.
///
/// # Arguments
///
/// * `app_id` - A string slice that holds the application ID.
/// * `app_description` - A string slice that holds the application description.
/// * `context_id` - A string slice that holds the context ID.
/// * `context_description` - A string slice that holds the context description.
///
/// # Errors
///
/// This function returns an `InitializeError` if there is an error during initialization. Possible
/// errors include:
///
/// * `ConversionError` - If there is a null byte in the provided strings.
/// * `LoggerError` - If there is an error setting the logger.
/// * `DltLibraryError` - If there is an error returned by the DLT library.
///
/// # Example
///
/// ```rust
/// fn main() {
///     let _ = dlt_log::init("APID", "Application Description", "CTID", "Context Description");
///     log::info!("This is an info message");
/// }
/// ```
pub fn init(
    app_id: &str,
    app_description: &str,
    context_id: &str,
    context_description: &str,
) -> Result<(), InitializeError> {
    // register application
    let c_app_id = CString::new(app_id)?;
    let c_app_description = CString::new(app_description)?;
    let dlt_return_value =
        unsafe { libdlt::dlt_register_app(c_app_id.as_ptr(), c_app_description.as_ptr()) };
    if dlt_return_value != DltReturnValue::DLT_RETURN_OK {
        return Err(InitializeError::DltLibraryError(dlt_return_value));
    }

    // create logger
    let ctx: Box<libdlt::DltContext> = Box::default();
    let dlt_logger = DltLogger {
        ctx: Box::into_raw(ctx),
    };

    // register context
    let c_context_id = CString::new(context_id)?;
    let c_context_description = CString::new(context_description)?;
    let dlt_return_value = unsafe {
        libdlt::dlt_register_context(
            dlt_logger.ctx,
            c_context_id.as_ptr(),
            c_context_description.as_ptr(),
        )
    };
    if dlt_return_value != DltReturnValue::DLT_RETURN_OK {
        return Err(InitializeError::DltLibraryError(dlt_return_value));
    }

    // set global logger
    log::set_boxed_logger(Box::new(dlt_logger))?;

    // set max level; DLT system takes care on filtering
    log::set_max_level(log::STATIC_MAX_LEVEL);

    Ok(())
}

struct DltLogger {
    ctx: *mut libdlt::DltContext,
}

// The `DltLogger` struct is marked as `Send` and `Sync` because the underlying DLT library is
// thread-safe, see https://github.com/COVESA/dlt-daemon/blob/master/doc/dlt_design_specification.md.
unsafe impl Send for DltLogger {}
unsafe impl Sync for DltLogger {}

impl log::Log for DltLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // will be handled inside DLT
        true
    }

    fn log(&self, record: &Record) {
        let level = match record.level() {
            Level::Error => libdlt::DltLogLevelType::DLT_LOG_ERROR,
            Level::Warn => libdlt::DltLogLevelType::DLT_LOG_WARN,
            Level::Info => libdlt::DltLogLevelType::DLT_LOG_INFO,
            Level::Debug => libdlt::DltLogLevelType::DLT_LOG_DEBUG,
            Level::Trace => libdlt::DltLogLevelType::DLT_LOG_VERBOSE,
        };

        let text = format!("{}", record.args());

        let c_text = match CString::new(text) {
            Ok(result) => result,
            Err(_error) => {
                CString::from(c"ERROR: NulError when converting log message from Rust to C.")
            }
        };

        let _dlt_return_value = unsafe { libdlt::dlt_log_string(self.ctx, level, c_text.as_ptr()) };
        // not much we can do here in case of error
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;

    use log::Log;

    use super::*;

    #[test]
    fn test_flush() {
        let logger = DltLogger { ctx: null_mut() };
        // does nothing
        logger.flush();
    }

    #[test]
    fn test_enabled() {
        let logger = DltLogger { ctx: null_mut() };

        // always needs be enabled as this is handled by DLT
        let metadata = Metadata::builder().level(Level::max()).build();
        assert!(logger.enabled(&metadata));
    }
}
