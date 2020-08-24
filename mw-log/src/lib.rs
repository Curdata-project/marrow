#![no_std]
#![feature(fmt_as_str)]

use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

struct MWLogger;

impl log::Log for MWLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        extern {
            fn println_stdout(s: *const u8);
        }
        if self.enabled(record.metadata()) {
            let out_str = record.args().as_str().unwrap();
            unsafe { println_stdout(out_str.as_ptr()) };
            // println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: MWLogger = MWLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}
