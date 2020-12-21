#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use core::{alloc::Layout, mem};

use alloc::string;
use mw_std::debug;
use mw_std::fs;
use mw_std::sql;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::async_main]
async fn main() {
    let _r = fs::read_file("./test.txt").await;
    debug::println("ok");
}

#[no_mangle]
pub extern "C" fn _sql(ty: u8) {
    let runtime = mw_rt::runtime::Runtime::new();

    runtime.spawn(async move {
        // let result = sql::sql_run("select * from test_db").await;

        let create_str = r#"
        CREATE TABLE "test_db" (
            "account" VARCHAR(255) NOT NULL,
            "secret_type" VARCHAR(255) NOT NULL,
            PRIMARY KEY ("account")
          )
        "#;

        let op = match ty {
            0 => Some(sql::sql_execute(create_str, ty).await),
            1 => Some(sql::sql_execute("select * from test_db", ty).await),
            _ => None,
        };
        if op.is_none() {
            debug::println("type parsing failed");
            return;
        }
        let result = op.unwrap();
        let str = string::String::from_utf8(result).unwrap();
        debug::println(str.as_str());
    });
}

#[no_mangle]
pub extern "C" fn _wbindgen_malloc(size: usize) -> *mut u8 {
    let align = mem::align_of::<usize>();
    //获取布局，不能为0
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                //分配内存
                let ptr = alloc::alloc::alloc(layout);
                if !ptr.is_null() {
                    return ptr;
                }
            } else {
                return align as *mut u8;
            }
        }
    }
    loop {}
}

// #[mw_rt::main]
// fn main() {
//     let _r = fs::read_file_callback("./test.txt", |_result| {
//         debug::println("ok");
//     });
// }
