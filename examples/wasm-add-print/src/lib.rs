#![no_std]

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn entry() {
    mw_log::init().unwrap();
    log::info!("{}", "hellp");
}
