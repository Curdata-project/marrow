#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

pub mod debug;
pub mod fs;

pub mod task;
pub mod runtime;
mod utils;

