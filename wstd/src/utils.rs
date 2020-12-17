#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo) -> ! {
    // println(format_args!("{:?}", _panic).as_str().unwrap());
    loop {}
}
