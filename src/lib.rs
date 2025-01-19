#![allow(non_snake_case)]
#![allow(unused_attributes)]
include!(concat!(env!("OUT_DIR"), "/libdlt_bindings.rs"));

use std::ffi::{CString, NulError};

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

#[derive(Debug)]
pub enum InitializeError {
    ConversionError(NulError),
    LoggerError(SetLoggerError),
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

pub fn init(appId: &str, description: &str) -> Result<(), InitializeError> {
    let dlt_logger = DltLogger {};
    let c_appId = CString::new(appId)?;
    let c_descriptionId = CString::new(description)?;
    let dltReturnValue = unsafe { dlt_register_app(c_appId.as_ptr(), c_descriptionId.as_ptr()) };
    if dltReturnValue != DLT_RETURN_OK {
        return Err(InitializeError::DltLibraryError(dltReturnValue));
    }
    log::set_boxed_logger(Box::new(dlt_logger))?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}

struct DltLogger {}

impl log::Log for DltLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    //use super::*;
}
