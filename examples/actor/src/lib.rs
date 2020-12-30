#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

// use mw_std::debug;
use alloc::boxed::Box;
use core::cell::RefCell;
use mw_rt::actor::Actor;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::actor::actor]
pub struct MyActor {}

#[async_trait::async_trait]
impl Actor for MyActor {
    fn new() -> Self {
        MyActor {}
    }

    async fn init(&mut self) {}
}

#[mw_rt::actor::expose]
impl MyActor {
    pub fn return_int(&mut self) -> usize {
        1
    }

    pub fn return_bytes(&mut self) -> &[u8] {
        &[0u8; 10]
    }

    pub async fn async_return_bytes(&mut self, t: u8, bytes1: &[u8], bytes2: &[u8]) -> &[u8] {
        &[0u8; 10]
    }
}
