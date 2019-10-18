use std::ffi::CString;
use libc::c_char;

#[no_mangle]
pub extern "C" fn say_hello() -> *mut c_char {
    let cstr = CString::new("hello").unwrap();
    cstr.into_raw()
}

#[no_mangle]
pub extern "C" fn hello_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
