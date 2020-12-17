#![no_std]

extern crate alloc;
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wstd::fs;
use wstd::debug;
use wstd::runtime;

#[no_mangle]
pub extern "C" fn main() {
    let runtime = runtime::Runtime::new();

    runtime.spawn(async move {
        let _r = fs::read_file("./test.txt").await;
        let _r = fs::read_file("./test2.txt").await;
        debug::println("ok");
    });
}
