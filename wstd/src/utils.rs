use core::ffi::c_void;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn get_callback<F>(mut _closure: F) -> (unsafe extern "C" fn(*mut c_void), *mut c_void)
where
    F: FnMut(),
{
    unsafe extern "C" fn hook<F>(user_data: *mut c_void)
    where
        F: FnMut(),
    {
        (*(user_data as *mut F))()
    }
    (hook::<F>, &mut _closure as *mut _ as *mut c_void)
}
