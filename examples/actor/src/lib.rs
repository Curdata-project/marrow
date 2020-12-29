#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

// use mw_std::debug;
use core::cell::RefCell;
use alloc::boxed::Box;
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

    async fn init(&mut self) {

    }
}

impl MyActor {
    pub fn return_int(&mut self) -> usize {
        1
    }

    pub fn return_bytes(&mut self) -> &[u8] {
        &[0u8; 10]
    }

    pub async fn async_return_int(&mut self) -> usize {
        1
    }
}
