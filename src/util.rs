use std::ffi::{c_void, CString};
use std::os::raw::c_char;
use std::path::Path;

// not using bindgen's derived #define OBX_NOT_FOUND 404, because it's a u32
pub const NOT_FOUND_404: i32 = 404;
pub const SUCCESS_0: i32 = 0;

pub type MutConstVoidPtr = *mut *const c_void;

// TODO verify correctness on all platforms
pub(crate) fn str_to_c_char(path: &str) -> *const c_char {
    let mut out_path = String::from(path);
    if !path.ends_with('\0') {
        out_path.push('\0');
    }
    let c_str = CString::new(out_path.as_str()).unwrap();
    c_str.as_ptr() as *const c_char
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
