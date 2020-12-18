#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use mw_std::debug;
use mw_std::fs;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::async_main]
async fn main() {
    let _r = fs::read_file("./test.txt").await;
    debug::println("ok");
}

// #[mw_rt::main]
// fn main() {
//     let _r = fs::read_file_callback("./test.txt", |_result| {
//         debug::println("ok");
//     });
// }
