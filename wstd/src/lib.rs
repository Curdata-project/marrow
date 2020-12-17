#![no_std]
#![feature(default_alloc_error_handler)]
#![feature(fmt_as_str)]

#[macro_use]
extern crate lazy_static;

extern crate alloc;
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub mod debug;
pub mod fs;

// mod utils;
pub mod task;
pub mod queue;

pub fn spawn_local<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
{
    task::Task::spawn(Box::pin(future));
}
