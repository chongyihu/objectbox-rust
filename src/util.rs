use std::ffi::CString;
use std::path::Path;
use std::os::raw::{c_char, c_void};

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
  fn to_mut_c_void(&mut self) -> *mut c_void;
  fn to_mut_const_c_void(&mut self) -> *mut *const c_void;
}

impl ToCVoid for Vec<u8> {
    fn to_const_c_void(&self) -> *const c_void {
      let sl = self.as_slice();
      sl.as_ptr() as *const c_void
    }

    fn to_mut_c_void(&mut self) -> *mut c_void {
      let sl = self.as_slice();
      sl.as_ptr() as *mut c_void
    }

    fn to_mut_const_c_void(&mut self) -> *mut *const c_void {
      let sl = self.as_slice();
      sl.as_ptr() as *mut *const c_void
    }
}

// TODO borrowed from StackOverflow on Unix (linux, mac, bsd, android etc.)
// #[cfg(unix)]
// fn path_to_bytes<P: AsRef<Path>>(path: P) -> Vec<u8> {
//   use std::os::unix::ffi::OsStrExt;
//   path.as_ref().as_os_str().as_bytes().to_vec()
// }

// // TODO borrowed from StackOverflow on windows
// #[cfg(not(unix))]
// fn path_to_bytes<P: AsRef<Path>>(path: P) -> Vec<i8> {
//     // On Windows, could use std::os::windows::ffi::OsStrExt to encode_wide(),
//     // but you end up with a Vec<u16> instead of a Vec<u8>, so that doesn't
//     // really help.
//     use std::os::windows::ffi::OsStrExt;
//     path.as_ref().to_string_lossy().to_string().into_bytes()
// }