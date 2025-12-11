#![allow(clippy::needless_doctest_main)]
/*!
This crate is a [`log`](https://docs.rs/log) adapter for integrating with the Diagnostic Log and Trace (DLT) system.

It provides functionality to register applications and contexts with the DLT daemon and implements a logger that sends log messages to the DLT system.
The library is designed to be used with the standard [`log`](https://docs.rs/log) crate, which provides a flexible logging API for Rust applications.

The DLT system is a flexible and scalable logging framework that is widely used in the automotive industry for logging and tracing in embedded systems.
It provides features such as log level filtering, message formatting, and log message buffering.
For more information about DLT, see the [DLT project on GitHub](https://github.com/COVESA/dlt-daemon).

# Dependencies
At least rust version 1.82 is required to build this crate.
This is caused by latest `bindgen` version which is not compatible with older rust versions, see [bindgen issue #3052](https://github.com/rust-lang/rust-bindgen/issues/3052).

You need to have `libclang-dev` installed on your system to run the binding generation and `libdlt-dev` to link against the DLT library.
On Ubuntu, you can install that with the following command:

```bash
sudo apt-get install libclang-dev libdlt-dev
```

The integration with the DLT system is achieved through the underlying C library for DLT.
[`bindgen`](https://docs.rs/bindgen) is used to generate the FFI (Foreign Function Interface) bindings.
The bindings are generated during the build process in order to ensure that the Rust code remains in sync with the system’s DLT library.

# Example usage

Add the `dlt_log` crate to your `Cargo.toml`:

```toml
[dependencies]
log = { version = "0.4", features = ["std"] }
dlt_log = "0.1"
```

To use this library, you need to initialize it with your application and context information.
This will register your application and context with the DLT daemon and set up the logger.

```rust
fn main() {
    let _ = dlt_log::init("TEST", "Rust tests", "EXPL", "Smoke test example");

    log::trace!("Tracing the untraceable!");
    log::debug!("Debugging the debugger!");
    log::info!("Information overload: {} + {} = {}", 2, 2, 4);
    log::warn!("Warning: Low on coffee!");
    log::error!("Error: Something went terribly right!");
}
```

Connect the DLT viewer to the DLT daemon to view the log messages.
The default log level is set to `info`, so only log messages with a level of `info` or higher will be displayed.
See the [Log levels](#log-levels) section for more information on how to configure the log levels.

![Example logs in DLT viewer](https://raw.githubusercontent.com/rusty-projects/dlt_log-rs/main/doc/dlt-viewer-example.png)

## Example for cross-compilation

As an example for cross-compilation, assume a yocto SDK is installed at `/opt/poky/5.1.2` for aarch64 architecture that includes the DLT library:
- `/opt/poky/5.1.2/environment-setup-cortexa57-poky-linux`
- `/opt/poky/5.1.2/sysroots/cortexa57-poky-linux/usr/lib/libdlt.so`
- `/opt/poky/5.1.2/sysroots/cortexa57-poky-linux/usr/include/dlt/dlt.h`

Create `.cargo/config.toml` in your project directory with the following content:

```toml
[target.aarch64-unknown-linux-gnu]
linker="/opt/poky/5.1.2/sysroots/x86_64-pokysdk-linux/usr/bin/aarch64-poky-linux/aarch64-poky-linux-gcc"
rustflags = [
"-C", "link-arg=--sysroot=/opt/poky/5.1.2/sysroots/cortexa57-poky-linux",
]
```

Bindgen needs to know the sysroot path to find the DLT headers, so set the `BINDGEN_EXTRA_CLANG_ARGS` environment variable before building the project:

```bash
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/opt/poky/5.1.2/sysroots/cortexa57-poky-linux"
cargo build --release --target aarch64-unknown-linux-gnu
```

# Log levels

The log levels and logging backend are configured by the DLT system.
For configuration, follow the [DLT documentation](https://github.com/COVESA/dlt-daemon/blob/master/doc/dlt_for_developers.md).

For example, to enable all log levels, set the `DLT_INITIAL_LOG_LEVEL` environment variable to `::6`.

```bash
DLT_INITIAL_LOG_LEVEL="::6" cargo run --example simple
```

# Logging to console

By default, the log messages are sent to the DLT daemon.
To view the logs, you can use the DLT viewer tool provided by the DLT daemon.
The logs can also be redirected to the console by setting the `DLT_LOCAL_PRINT_MODE` environment variable to `FORCE_ON`.

```bash
DLT_LOCAL_PRINT_MODE=FORCE_ON DLT_INITIAL_LOG_LEVEL="::6" cargo run --example simple
```

The output will look like this:

```text
2025/01/21 19:12:22.090974   28995094 000 ECU1 TEST EXPL log verbose V 1 [Tracing the untraceable!]
2025/01/21 19:12:22.091176   28995096 001 ECU1 TEST EXPL log debug V 1 [Debugging the debugger!]
2025/01/21 19:12:22.091248   28995097 002 ECU1 TEST EXPL log info V 1 [Information overload: 2 + 2 = 4]
2025/01/21 19:12:22.091284   28995097 003 ECU1 TEST EXPL log warn V 1 [Warning: Low on coffee!]
2025/01/21 19:12:22.091336   28995097 004 ECU1 TEST EXPL log error V 1 [Error: Something went terribly right!]
```

# Errors

The library defines an [`InitializeError`] enum to represent possible errors that can occur during initialization.
These include conversion errors, logger errors, and DLT library errors.
*/

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
    /// This variant encapsulates the [`DltReturnValue`],
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

        let text = format!(
            "[{}:{}] {}",
            record
                .file_static()
                .unwrap_or("?")
                .rsplit('/')
                .next()
                .unwrap(),
            record.line().unwrap_or(0),
            record.args()
        );

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
