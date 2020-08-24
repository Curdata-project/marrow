use cstr_core::{c_char, CStr};
use std::io::stderr;
use std::io::stdout;
use std::io::Write;

#[no_mangle]
pub extern "C" fn println_stdout(s: *const c_char) {
    let cstr = unsafe { CStr::from_ptr(s) };
    // TODO: Deal un utf-8
    let out_str = cstr.to_str().unwrap();
    let mut out = stdout();
    writeln!(&mut out, "{}", out_str).unwrap();
}

#[no_mangle]
pub extern "C" fn println_stderr(s: *const c_char) {
    let cstr = unsafe { CStr::from_ptr(s) };
    // TODO: Deal un utf-8
    let out_str = cstr.to_str().unwrap();
    let mut err = stderr();
    writeln!(&mut err, "{}", out_str).unwrap();
}
