#![no_std]
#![feature(fmt_as_str)]

use log::{Level, Metadata, Record, SetLoggerError};

struct MWLogger {}

#[link(wasm_import_module = "prints")]
extern "C" {
    fn print(s: *const u8, length: usize);
}

pub fn println(s: &str) {
    let ptr = s.as_ptr();
    let length = s.len();

    unsafe {
        print(ptr, length);
    };
}

impl log::Log for MWLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let s = format_args!("{} - {}", record.level(), record.args())
            .as_str()
            .unwrap();

        println(s);
    }

    fn flush(&self) {}
}

static LOGGER: MWLogger = MWLogger {};

pub fn init() -> Result<(), SetLoggerError> {
    // println("asassd");
    log::set_logger(&LOGGER)?;
    log::set_max_level(Level::Info.to_level_filter());
    Ok(())
}
