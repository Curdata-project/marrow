#![no_std]

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo) -> ! {
    loop {}
}

// #[link(wasm_import_module = "maths")]
extern "C" {
    fn add() -> u32;
}

#[no_mangle]
pub fn entry() {
    mw_log::init().unwrap();
    log::info!("hello {}", "world");
}
