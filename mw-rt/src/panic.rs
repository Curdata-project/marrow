use alloc::format;
use mw_std::debug;

#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo) -> ! {
    debug::println(&format!("{}", _panic));
    loop {}
}
