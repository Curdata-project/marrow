#![no_std]
#![feature(fmt_as_str)]

use core::fmt::Arguments;
use cstr_core::c_char;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct MWLogger;

fn println_stdout(s: Arguments) {
    extern "C" {
        fn p_stdout(s: *const c_char, length: usize);
    }
    let out_str = s.as_str().unwrap();
    let ptr = out_str.as_ptr();
    let length = out_str.len();
    unsafe {
        p_stdout(ptr as *const c_char, length);
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
