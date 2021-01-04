#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::string;
use mw_std::debug;
use mw_std::fs;
use mw_std::sql;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::async_main]
async fn main() {
}
