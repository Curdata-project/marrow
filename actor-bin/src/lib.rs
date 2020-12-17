#![no_std]

extern crate alloc;
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wstd::fs;
use wstd::debug;
use wstd::task;
use alloc::boxed::Box;

#[no_mangle]
pub extern "C" fn main() {
    task::Task::spawn(Box::pin(async move{
        let _r = fs::read_file("./test.txt").await;
        debug::println("ok");
    }));
}
