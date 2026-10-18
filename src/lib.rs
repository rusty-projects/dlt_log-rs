#![allow(clippy::needless_doctest_main)]
// Overrides the docs.rs fallback link in README.md with an intra-doc link.
//! [`InitializeError`]: crate::InitializeError
#![doc = include_str!("../README.md")]

use log::{Level, Metadata, Record, SetLoggerError};
use std::ffi::{CString, NulError};
use std::fmt;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicI8, Ordering};
use std::sync::{Mutex, PoisonError};

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
    /// This variant encapsulates the [`DltReturnValue`],
    /// which provides detailed information about the specific error encountered
    /// during DLT operations.
    DltLibraryError(DltReturnValue),
}

impl fmt::Display for InitializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitializeError::ConversionError(_) => {
                write!(f, "argument contains an interior null byte")
            }
            InitializeError::LoggerError(_) => write!(f, "failed to set the logger"),
            InitializeError::DltLibraryError(value) => {
                write!(f, "DLT library returned an error: {:?}", value)
            }
        }
    }
}

impl std::error::Error for InitializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InitializeError::ConversionError(err) => Some(err),
            InitializeError::LoggerError(err) => Some(err),
            InitializeError::DltLibraryError(_) => None,
        }
    }
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
/// * `LoggerError` - If there is an error setting the logger, e.g. because a logger is already set,
///   also by a previous successful call of this function. A failed call leaves no DLT state behind
///   and does not affect an already running logger.
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
    // convert all strings first, so that no DLT state is created for invalid input
    let c_app_id = CString::new(app_id)?;
    let c_app_description = CString::new(app_description)?;
    let c_context_id = CString::new(context_id)?;
    let c_context_description = CString::new(context_description)?;

    // serialize initialization, so that concurrent calls cannot interleave their DLT
    // registrations; a panic cannot leave the flag in an inconsistent state, so ignore poisoning
    let mut initialized = INITIALIZED.lock().unwrap_or_else(PoisonError::into_inner);
    if *initialized {
        // Registering again would replace the application of the running logger, so fail before
        // any DLT call. The global logger is set, so this returns the same error as setting it
        // again.
        return Err(log::set_logger(&UNUSED_LOGGER).unwrap_err().into());
    }

    // register application
    let dlt_return_value =
        unsafe { libdlt::dlt_register_app(c_app_id.as_ptr(), c_app_description.as_ptr()) };
    if dlt_return_value != DltReturnValue::DLT_RETURN_OK {
        return Err(InitializeError::DltLibraryError(dlt_return_value));
    }

    // create logger
    let ctx: Box<libdlt::DltContext> = Box::default();
    let ctx = Box::into_raw(ctx);

    // register context
    let dlt_return_value = unsafe {
        libdlt::dlt_register_context(ctx, c_context_id.as_ptr(), c_context_description.as_ptr())
    };
    if dlt_return_value != DltReturnValue::DLT_RETURN_OK {
        // context is not registered, so DLT does not use it and it can be freed
        drop(unsafe { Box::from_raw(ctx) });
        let _ = unsafe { libdlt::dlt_unregister_app() };
        return Err(InitializeError::DltLibraryError(dlt_return_value));
    }

    // set global logger; fails if another logger is already set
    if let Err(err) = log::set_boxed_logger(Box::new(DltLogger { ctx })) {
        // undo the registration, so that no DLT state is left behind
        let _ = unsafe { libdlt::dlt_unregister_context(ctx) };
        drop(unsafe { Box::from_raw(ctx) });
        let _ = unsafe { libdlt::dlt_unregister_app() };
        return Err(err.into());
    }
    *initialized = true;

    // set max level; the DLT log level can change at runtime, so the logger checks it on each call
    log::set_max_level(log::STATIC_MAX_LEVEL);

    Ok(())
}

/// Whether [`init`] has set the global logger. The mutex also serializes calls to [`init`].
static INITIALIZED: Mutex<bool> = Mutex::new(false);

/// Logger only used to obtain a [`SetLoggerError`] on repeated initialization, never set.
static UNUSED_LOGGER: DltLogger = DltLogger { ctx: null_mut() };

struct DltLogger {
    ctx: *mut libdlt::DltContext,
}

// The `DltLogger` struct is marked as `Send` and `Sync` because the underlying DLT library is
// thread-safe, see https://github.com/COVESA/dlt-daemon/blob/master/doc/dlt_design_specification.md.
unsafe impl Send for DltLogger {}
unsafe impl Sync for DltLogger {}

impl DltLogger {
    /// Checks whether DLT is going to log messages of `level` for the context of this logger.
    ///
    /// This is the logic of `dlt_user_is_logLevel_enabled`, which is a `static inline` function
    /// in the DLT header and thus not available through the bindings.
    fn is_level_enabled(&self, level: libdlt::DltLogLevelType) -> bool {
        if self.ctx.is_null() {
            return false;
        }
        // SAFETY: `ctx` points to a registered context, which is never unregistered
        let log_level_ptr = unsafe { (*self.ctx).log_level_ptr };
        if log_level_ptr.is_null() {
            return false;
        }
        // SAFETY: DLT allocates the log level of a context once at registration and updates it
        // from its receiver thread, so read it atomically
        let log_level = unsafe { AtomicI8::from_ptr(log_level_ptr) }.load(Ordering::Relaxed);
        // `level` is never `DLT_LOG_OFF`, as it is converted from a `log::Level`
        level as i8 <= log_level
    }
}

/// Converts a `log` level to the corresponding DLT log level.
fn dlt_level(level: Level) -> libdlt::DltLogLevelType {
    match level {
        Level::Error => libdlt::DltLogLevelType::DLT_LOG_ERROR,
        Level::Warn => libdlt::DltLogLevelType::DLT_LOG_WARN,
        Level::Info => libdlt::DltLogLevelType::DLT_LOG_INFO,
        Level::Debug => libdlt::DltLogLevelType::DLT_LOG_DEBUG,
        Level::Trace => libdlt::DltLogLevelType::DLT_LOG_VERBOSE,
    }
}

impl log::Log for DltLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.is_level_enabled(dlt_level(metadata.level()))
    }

    fn log(&self, record: &Record) {
        let level = dlt_level(record.level());
        // skip formatting of messages that DLT would drop
        if !self.is_level_enabled(level) {
            return;
        }

        let c_text = to_c_string(format_message(record));

        let _dlt_return_value = unsafe { libdlt::dlt_log_string(self.ctx, level, c_text.as_ptr()) };
        // not much we can do here in case of error
    }

    fn flush(&self) {}
}

/// Formats a log record as `[file:line] message`, with the file name stripped of its path.
fn format_message(record: &Record) -> String {
    // use `file()` instead of `file_static()`, as records created at runtime, e.g. by log
    // bridges, only provide a non-static file name
    format!(
        "[{}:{}] {}",
        record.file().unwrap_or("?").rsplit('/').next().unwrap(),
        record.line().unwrap_or(0),
        record.args()
    )
}

/// Converts a message to a C string, escaping interior NUL bytes as `\0`.
///
/// Escaping instead of dropping the message ensures that data containing NUL bytes cannot
/// suppress a log message. The buffer of `text` is reused, so the common case without NUL bytes
/// does not copy the message.
fn to_c_string(mut text: String) -> CString {
    if text.contains('\0') {
        text = text.replace('\0', "\\0");
    }
    CString::new(text).expect("NUL bytes have been escaped")
}

#[cfg(test)]
mod tests {
    use log::Log;

    use super::*;

    #[test]
    fn test_to_c_string_without_nul() {
        // spare capacity for the terminating NUL byte avoids a reallocation
        let mut text = String::with_capacity(13);
        text.push_str("no NUL bytes");
        let ptr = text.as_ptr();
        let c_text = to_c_string(text);
        assert_eq!(c_text.to_bytes(), b"no NUL bytes");
        // buffer is reused
        assert_eq!(c_text.as_ptr().cast::<u8>(), ptr);
    }

    #[test]
    fn test_to_c_string_with_nul() {
        let c_text = to_c_string(String::from("a\0b\0"));
        assert_eq!(c_text.to_bytes(), br"a\0b\0");
    }

    #[test]
    fn test_flush() {
        let logger = DltLogger { ctx: null_mut() };
        // does nothing
        logger.flush();
    }

    #[test]
    fn test_enabled_without_context() {
        let logger = DltLogger { ctx: null_mut() };

        let metadata = Metadata::builder().level(Level::Error).build();
        assert!(!logger.enabled(&metadata));
    }

    #[test]
    fn test_enabled_without_log_level() {
        // context that is not registered with DLT
        let mut ctx = libdlt::DltContext::default();
        let logger = DltLogger { ctx: &mut ctx };

        let metadata = Metadata::builder().level(Level::Error).build();
        assert!(!logger.enabled(&metadata));
    }

    #[test]
    fn test_enabled_levels() {
        let mut log_level = libdlt::DltLogLevelType::DLT_LOG_INFO as i8;
        let mut ctx = libdlt::DltContext {
            log_level_ptr: &mut log_level,
            ..Default::default()
        };
        let logger = DltLogger { ctx: &mut ctx };

        let enabled = |level| logger.enabled(&Metadata::builder().level(level).build());
        assert!(enabled(Level::Error));
        assert!(enabled(Level::Warn));
        assert!(enabled(Level::Info));
        assert!(!enabled(Level::Debug));
        assert!(!enabled(Level::Trace));
    }

    #[test]
    fn test_enabled_log_level_change() {
        let log_level = AtomicI8::new(libdlt::DltLogLevelType::DLT_LOG_OFF as i8);
        let mut ctx = libdlt::DltContext {
            log_level_ptr: log_level.as_ptr(),
            ..Default::default()
        };
        let logger = DltLogger { ctx: &mut ctx };
        let metadata = Metadata::builder().level(Level::Trace).build();
        assert!(!logger.enabled(&metadata));

        // DLT daemon changes the log level at runtime
        log_level.store(
            libdlt::DltLogLevelType::DLT_LOG_VERBOSE as i8,
            Ordering::Relaxed,
        );
        assert!(logger.enabled(&metadata));
    }

    #[test]
    fn test_log_without_context() {
        let logger = DltLogger { ctx: null_mut() };

        let record = Record::builder()
            .args(format_args!("Test log message"))
            .level(Level::Info)
            .file(Some("test_file.rs"))
            .line(Some(42))
            .build();

        // message is dropped without panic
        logger.log(&record);
    }

    #[test]
    fn test_format_message_static_file() {
        let record = Record::builder()
            .args(format_args!("message"))
            .file_static(Some("src/static.rs"))
            .line(Some(42))
            .build();

        assert_eq!(format_message(&record), "[static.rs:42] message");
    }

    #[test]
    fn test_format_message_non_static_file() {
        // file name only known at runtime, as provided by log bridges
        let file = String::from("src/dynamic.rs");
        let record = Record::builder()
            .args(format_args!("message"))
            .file(Some(&file))
            .line(Some(7))
            .build();

        assert_eq!(format_message(&record), "[dynamic.rs:7] message");
    }

    #[test]
    fn test_format_message_empty_values() {
        let record = Record::builder()
            .args(format_args!("message"))
            .file(None)
            .line(None)
            .build();

        assert_eq!(format_message(&record), "[?:0] message");
    }

    #[test]
    fn test_format_message_paths() {
        for (file, expected) in [("/my/test/path", "path"), ("path/", ""), ("/", "")] {
            let record = Record::builder()
                .args(format_args!("message"))
                .file(Some(file))
                .line(Some(1))
                .build();

            assert_eq!(format_message(&record), format!("[{}:1] message", expected));
        }
    }
}
