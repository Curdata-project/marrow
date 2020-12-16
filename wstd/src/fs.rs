use crate::utils;
use core::ffi::c_void;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

pub fn read_file_callback<F>(s: &str, f: F) -> usize
where
    F: FnMut(),
{
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _read_file_callback(
            cb: unsafe extern "C" fn(*mut c_void),
            user_data: *mut c_void,
            path: *const u8,
            path_len: usize,
        ) -> usize;
    }

    let (cb, user_data) = utils::get_callback(f);

    let bytes = s.as_bytes();

    unsafe { _read_file_callback(cb, user_data, bytes.as_ptr(), bytes.len()) }
}

pub struct ReadFile {
    result: Option<()>,
    task: Option<Waker>,
}

impl Default for ReadFile {
    fn default() -> Self {
        ReadFile {
            result: None,
            task: None,
        }
    }
}

impl Future for ReadFile {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // let mut inner = self.

        if let Some(_) = self.result {
            return Poll::Ready(())
        }

        // self.task = Some(cx.waker().clone());

        Poll::Pending
    }
}

pub fn read_file(s: &str) -> ReadFile {
    let mut fu = ReadFile::default();
    read_file_callback(s, || {
        fu.result = Some(())
    });
    fu
}

#[no_mangle]
pub extern "C" fn call_read_file_callback_fn(
    cb: unsafe extern "C" fn(*mut c_void),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data) }
}
