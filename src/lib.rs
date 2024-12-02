use std::ffi::{CString, c_char};

#[no_mangle]
pub extern "C" fn hello_world() -> *mut c_char {
    let message = CString::new("Hello World from Rust!").unwrap();
    message.into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        drop(CString::from_raw(ptr));
    }
}
