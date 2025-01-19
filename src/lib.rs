use std::ffi::{CString, NulError};

mod libdlt;

use libdlt::{dlt_log_string, dlt_user_is_logLevel_enabled};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

#[derive(Debug)]
pub enum InitializeError {
    ConversionError(NulError),
    LoggerError(SetLoggerError),
    DltLibraryError(libdlt::DltReturnValue),
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
    if dlt_return_value != libdlt::DLT_RETURN_OK {
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
    if dlt_return_value != libdlt::DLT_RETURN_OK {
        return Err(InitializeError::DltLibraryError(dlt_return_value));
    }

    // set global logger
    log::set_boxed_logger(Box::new(dlt_logger))?;
    log::set_max_level(LevelFilter::Trace);

    Ok(())
}

struct DltLogger {
    ctx: *mut libdlt::DltContext,
}
// The dlt library is thread-safe, see https://github.com/COVESA/dlt-daemon/blob/master/doc/dlt_design_specification.md
unsafe impl Send for DltLogger {}
unsafe impl Sync for DltLogger {}

impl log::Log for DltLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let level = match metadata.level() {
            Level::Error => libdlt::DLT_LOG_ERROR,
            Level::Warn => libdlt::DLT_LOG_WARN,
            Level::Info => libdlt::DLT_LOG_INFO,
            Level::Debug => libdlt::DLT_LOG_DEBUG,
            Level::Trace => libdlt::DLT_LOG_VERBOSE,
        };
        //let dlt_return_value = unsafe { dlt_user_is_logLevel_enabled(self.ctx, level) };
        let dlt_return_value = libdlt::DLT_RETURN_TRUE;
        dlt_return_value == libdlt::DLT_RETURN_TRUE
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                Level::Error => libdlt::DLT_LOG_ERROR,
                Level::Warn => libdlt::DLT_LOG_WARN,
                Level::Info => libdlt::DLT_LOG_INFO,
                Level::Debug => libdlt::DLT_LOG_DEBUG,
                Level::Trace => libdlt::DLT_LOG_VERBOSE,
            };

            let text = format!("{}", record.args());
            let c_text = CString::new(text).unwrap();

            let _dlt_return_value = unsafe { dlt_log_string(self.ctx, level, c_text.as_ptr()) };
            // not much we can do here in case of error
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    //use super::*;
}
