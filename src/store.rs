#[allow(dead_code)]

use anymap::AnyMap;

use crate::c::{*, self};
use crate::error::Error;

use crate::opt::Opt;

// Caveat: copy and drop are mutually exclusive

pub struct Store {
  pub(crate) model_callback: Option<Box<dyn Fn() -> crate::model::Model>>,
  pub(crate) trait_map: Option<AnyMap>, // passed as a ref to a Box
  // pub(crate) obx_model: *mut OBX_model, // TODO confirm: model and opt are cleaned up already,
  // Leaky? repeatedly allocate model and opts, and intentionally fail each time
  pub error: Option<Error>,
  pub(crate) obx_store: *mut OBX_store, // TODO confirm: model and opt are cleaned up already
}

impl Drop for Store {
  fn drop(&mut self) {
    if !self.obx_store.is_null() {
      self.close();
      self.obx_store = std::ptr::null_mut();
    }

    if let Some(err) = &self.error {
      println!("Error: {}", err);
    }
  }
}

// TODO Bonus: start admin http in debug from store?

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

impl Store {
  fn from_options(opt: &Opt) -> Self {
    Store {
      obx_store: unsafe { obx_store_open(opt.obx_opt) },
      error: None,
      model_callback: None,
      trait_map: None,
    }
  }

  // fn is_open(path: &Path) -> bool {
  //   let c_path = path_to_bytes(path).as_mut_ptr(); // TODO write test, assert ends with a null-terminator
  //   unsafe { obx_store_is_open(c_path) }
  // }

  // fn from_path_attach(path: &Path) -> Self {
  //   let c_path = path_to_bytes(path).as_ptr();
  //   Store {
  //     obx_store: unsafe { obx_store_attach(c_path) }, // TODO write test, assert ends with a null-terminator
  //     error: None,
  //     model_callback: None,
  //     trait_map: None,
  //   }
  // }

  fn from_store_id_attach(store_id: u64) -> Self {
    Store {
      obx_store: unsafe { obx_store_attach_id(store_id) },
      error: None,
      model_callback: None,
      trait_map: None,
    }
  }

  fn attach_or_open(
    opt: *mut OBX_store_options,
    check_matching_options: bool,
    out_attached: *mut bool,
  ) -> Self {
    Store {
      obx_store: unsafe { obx_store_attach_or_open(opt, check_matching_options, out_attached) },
      error: None,
      model_callback: None,
      trait_map: None,
    }
  }

  fn id(&self) -> u64 {
    unsafe { obx_store_id(self.obx_store) }
  }

  // TODO impl trait, then use over channels
  // fn clone(&self) -> Self {
  //   Store {
  //     obx_store: unsafe { obx_store_clone(self.obx_store) },
  //     error: None,
  //   }
  // }

  fn from_core_wrap(core_store: *mut ::std::os::raw::c_void) -> Self {
    Store {
      obx_store: unsafe { obx_store_wrap(core_store) },
      error: None,
      model_callback: None,
      trait_map: None,
    }
  }

  fn entity_id(&self, entity_name: *const ::std::os::raw::c_char) -> obx_schema_id {
    unsafe { obx_store_entity_id(self.obx_store, entity_name) }
  }

  fn entity_property_id(&self,
    entity_id: obx_schema_id,
    property_name: *const ::std::os::raw::c_char
  ) -> obx_schema_id {
    unsafe { obx_store_entity_property_id(self.obx_store, entity_id, property_name) }
  }

  fn await_async_completion(&self) -> bool {
    unsafe { obx_store_await_async_completion(self.obx_store) }
  }

  fn await_async_submitted(&self) -> bool {
    unsafe { obx_store_await_async_submitted(self.obx_store) }
  }

  fn debug_flags(&mut self, flags: OBXDebugFlags) {
    self.error = c::call(unsafe { obx_store_debug_flags(self.obx_store, flags) }).err();
  }

  fn opened_with_previous_commit(&self) -> bool {
    unsafe { obx_store_opened_with_previous_commit(self.obx_store) }
  }

  fn prepare_to_close(&mut self) {
    self.error = c::call(unsafe { obx_store_prepare_to_close(self.obx_store) }).err()
  }

  fn close(&mut self) {
    self.error = c::call(unsafe { obx_store_close(self.obx_store) }).err();
  }
}
