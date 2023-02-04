#[allow(dead_code)]

use std::ffi::CString;
use std::path::Path;
use std::ptr;
use std::rc::Rc;

use anymap::AnyMap;

use crate::c::{*, self};
use crate::error::Error;

use crate::opt::Opt;
use crate::traits::{FactoryHelper, OBBlanket};
use crate::util::{ToCChar, ToCVoid};

// Caveat: copy and drop are mutually exclusive

pub struct Store {
  pub trait_map: Option<AnyMap>, // passed as a ref to a Box
  // TODO confirm: model and opt are cleaned up already and zero'ed, or else we'll have a double-free
  // Leaky? repeatedly allocate model and opts, and intentionally fail each time
  pub(crate) error: Option<Error>,
  pub(crate) obx_store: *mut OBX_store, // TODO confirm: model and opt are cleaned up already
}

impl Drop for Store {
  fn drop(&mut self) {
    if !self.obx_store.is_null() {
      self.prepare_to_close();
      self.close();
      self.obx_store = std::ptr::null_mut();
    }

    if let Some(err) = &self.error {
      eprintln!("Error: store: {err}");
    }
  }
}

// TODO Bonus: start admin http in debug from store?

impl Store {
  // TODO pub fn from_model_callback() ... generated from open_store()

  pub fn from_options(opt: &mut Opt) -> Self {
    if let Some(err) = &opt.error {
      panic!("Error: store: {err}");
    }
    match c::new_mut(unsafe { obx_store_open(opt.obx_opt) }, "store::from_options".to_string()) {
      Ok(obx_store) => {
        opt.ptr_consumed = !obx_store.is_null();
        Store {
          trait_map: None,
          error: None,
          obx_store,
        }
      },
      Err(e) => Store {
        trait_map: None,
        error: Some(e),
        obx_store: ptr::null_mut(),
      },
    }
  }

  pub fn get_box<T: 'static + OBBlanket>(&self) -> crate::r#box::Box::<T> {
    if let Some(err) = &self.error {
      panic!("Error: store: {err}");
    }
    let map = if let Some(m) = &self.trait_map {
      m
    }else {
      panic!("Error: unable to get box");
    };
    let helper = 
      if let Some(h) = 
        map.get::<Rc<dyn FactoryHelper<T>>>() {
      h
    }else {
      panic!("Error: unable to get entity helper");
    };
    crate::r#box::Box::<T>::new(self.obx_store, helper.clone())
  }

  pub fn is_open(path: &Path) -> bool {
    unsafe { obx_store_is_open(path.to_c_char()) }
  }

  pub fn from_path_attach(path: &Path) -> Self {
    Store {
      obx_store: unsafe { obx_store_attach(path.to_c_char()) },
      error: None,
      trait_map: None,
    }
  }

  pub fn from_store_id_attach(store_id: u64) -> Self {
    Store {
      obx_store: unsafe { obx_store_attach_id(store_id) },
      error: None,
      trait_map: None,
    }
  }

  pub fn attach_or_open(
    opt: *mut OBX_store_options,
    check_matching_options: bool,
    out_attached: *mut bool,
  ) -> Self {
    Store {
      obx_store: unsafe { obx_store_attach_or_open(opt, check_matching_options, out_attached) },
      error: None,
      trait_map: None,
    }
  }

  pub fn id(&self) -> u64 {
    unsafe { obx_store_id(self.obx_store) }
  }

  // TODO impl without Copy/Clone trait, because Drop, then use over channels
  // pub fn clone(&self) -> Self {
  //   Store {
  //     obx_store: unsafe { obx_store_clone(self.obx_store) },
  //     error: None,
  //   }
  // }
/*
  pub fn from_core_wrap(core_store: &mut Vec<u8>) -> Self {
    Store {
      obx_store: unsafe { obx_store_wrap(core_store) },
      error: None,
      trait_map: None,
    }
  }
*/

  pub fn entity_id(&self, entity_name: &str) -> obx_schema_id {
    unsafe {
      let c_str = if let Ok(r) = CString::new(entity_name) {
        r.as_ptr()
      }else {
        panic!("Error: unable to convert entity name");
      };
      obx_store_entity_id(self.obx_store, c_str)
    }
  }

  pub fn entity_property_id(&self,
    entity_id: obx_schema_id,
    property_name: &str
  ) -> obx_schema_id {
    unsafe {
      let c_str = if let Ok(r) = CString::new(property_name) {
        r.as_ptr()
      }else {
        panic!("Error: unable to convert property name");
      };
      obx_store_entity_property_id(self.obx_store, entity_id, c_str)
    }
  }

  pub fn await_async_completion(&self) -> bool {
    unsafe { obx_store_await_async_completion(self.obx_store) }
  }

  pub fn await_async_submitted(&self) -> bool {
    unsafe { obx_store_await_async_submitted(self.obx_store) }
  }

  pub fn debug_flags(&mut self, flags: OBXDebugFlags) {
    self.error = c::call(unsafe { obx_store_debug_flags(self.obx_store, flags) }, "store::debug_flags".to_string()).err();
  }

  pub fn opened_with_previous_commit(&self) -> bool {
    unsafe { obx_store_opened_with_previous_commit(self.obx_store) }
  }

  pub(crate) fn prepare_to_close(&mut self) {
    self.error = c::call(unsafe { obx_store_prepare_to_close(self.obx_store) }, "store::prepare_to_close".to_string()).err()
  }

  pub(crate) fn close(&mut self) {
    self.error = c::call(unsafe { obx_store_close(self.obx_store) }, "store::close".to_string()).err();
  }
}
