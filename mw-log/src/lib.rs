#![no_std]
#![feature(fmt_as_str)]

use cstr_core::c_char;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use core::fmt::Arguments;

struct MWLogger;

fn println_stdout(s: Arguments) {
    extern "C" {
        fn p_stdout(s: *const c_char);
    }
    let out_str = s.as_str().unwrap();
    let ptr = out_str.as_ptr();
    unsafe {
        p_stdout(ptr as *const c_char);
    }
}

impl log::Log for MWLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {

            println_stdout(format_args!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}

static LOGGER: MWLogger = MWLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
