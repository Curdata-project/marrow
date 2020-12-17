#![no_std]
#![feature(default_alloc_error_handler)]
#![feature(fmt_as_str)]

#[macro_use]
extern crate lazy_static;

extern crate alloc;
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub mod debug;
pub mod fs;
pub mod task;
pub mod queue;
mod macro_main;
pub use macro_main::main;

