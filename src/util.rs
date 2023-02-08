use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

// not using bindgen's derived #define OBX_NOT_FOUND 404, because it's a u32
pub const NOT_FOUND_404: i32 = 404;
pub const SUCCESS_0: i32 = 0;

pub type MutConstVoidPtr = *mut *const c_void;
pub type ConstVoidPtr = *const c_void;
pub type PtrConstChar = *const ::std::os::raw::c_char;

// TODO verify correctness on all platforms
pub(crate) fn str_to_c_char(path: &str) -> *const c_char {
    let mut out_path = String::from(path);
    if !path.ends_with('\0') {
        out_path.push('\0');
    }
    let c_str = CString::new(out_path.as_str()).unwrap();
    c_str.as_ptr() as *const c_char
}

pub fn test_fn_ptr_on_char_ptr(c_ptr: PtrConstChar, fn_ptr: fn(String) -> bool) -> bool {
    // allow panic here, it's for testing anyway
    if c_ptr.is_null() {
        panic!("Encountered a null ptr");
    }

    let mut out_str = String::new();
    unsafe {
        let c_str = CStr::from_ptr(c_ptr); // assuming the ptr ends with '\0'
        match c_str.to_str() {
            // allow panic here, it's for testing anyway
            Err(err) => panic!("{err}"),
            Ok(s) => out_str.push_str(s),
        }
        fn_ptr(out_str)
    }
}

pub trait ToCChar {
    fn to_c_char(&self) -> *const c_char;
}

impl ToCChar for Path {
    fn to_c_char(&self) -> *const c_char {
        str_to_c_char(self.to_str().unwrap())
    }
}

pub(crate) trait ToCVoid {
    fn to_const_c_void(&self) -> *const c_void;
}

impl ToCVoid for Vec<u8> {
    fn to_const_c_void(&self) -> *const c_void {
        let sl = self.as_slice();
        sl.as_ptr() as *const c_void
    }
}
