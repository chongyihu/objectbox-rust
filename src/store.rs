use crate::c::*;
use crate::error::Error;

pub struct Store {
  pub(crate) model_callback: Option<Box<dyn Fn() -> crate::model::Model>>,
  pub(crate) obx_model: Option<*mut OBX_model>,
  pub(crate) obx_store: Option<*mut OBX_store>,
  pub(crate) obx_store_options: Option<*mut OBX_store_options>,
  pub error: Option<Error>,
}

// TODO impl Drop
// TODO Drop: obx_model_free -> OBX_model
// TODO Drop: obx_opt_free -> OBX_store_options
// TODO Drop: obx_store_close -> OBX_store

// TODO Bonus: start admin http in debug from store?

impl Store {
  pub fn open(opt: *mut OBX_store_options) -> *mut OBX_store {
      unsafe { obx_store_open(opt) }
  }

  pub fn is_open(path: *const ::std::os::raw::c_char) -> bool {
      unsafe { obx_store_is_open(path) }
  }

  pub fn attach(path: *const ::std::os::raw::c_char) -> *mut OBX_store {
      unsafe { obx_store_attach(path) }
  }

  pub fn attach_id(store_id: u64) -> *mut OBX_store {
      unsafe { obx_store_attach_id(store_id) }
  }

  pub fn attach_or_open(
      opt: *mut OBX_store_options,
      check_matching_options: bool,
      out_attached: *mut bool,
  ) -> *mut OBX_store {
      unsafe { obx_store_attach_or_open(opt, check_matching_options, out_attached) }
  }

  pub fn id(&self) -> u64 {
      if let Some(store) = self.obx_store {
        unsafe { 
          obx_store_id(store)
        }
      }else {
        0
      }
  }

  // TODO implement in Clone trait
  pub fn clone(&self) -> Self {
    if let Some(store) = self.obx_store {
      Store {
        obx_store: unsafe { Some(obx_store_clone(store)) },
        model_callback: None,
        obx_model: None,
        obx_store_options: None,
        error: None,
      }
    }else{
      println!("Unable to clone store");
      Store { model_callback: None, obx_model: None, obx_store: None, obx_store_options: None, error: None  }
    }
  }

  pub fn wrap(core_store: *mut ::std::os::raw::c_void) -> *mut OBX_store {
      unsafe { obx_store_wrap(core_store) }
  }

  pub fn entity_id(&self, entity_name: *const ::std::os::raw::c_char) -> obx_schema_id {
    if let Some(store) = self.obx_store {
      unsafe { 
        obx_store_entity_id(store, entity_name)
      }
    }else {
      0
    }
  }

  pub fn entity_property_id(
      &self,
      entity_id: obx_schema_id,
      property_name: *const ::std::os::raw::c_char,
  ) -> obx_schema_id {
    if let Some(store) = self.obx_store {
      unsafe { 
        obx_store_entity_property_id(store, entity_id, property_name)
      }
    }else {
      0
    }
  }

  pub fn await_async_completion(&self) -> bool {
    if let Some(store) = self.obx_store {
      unsafe { 
        obx_store_await_async_completion(store)
      }
    }else {
      false
    }
  }

  pub fn await_async_submitted(&self) -> bool {
    if let Some(store) = self.obx_store {
      unsafe { 
        obx_store_await_async_submitted(store)
      }
    }else {
      false
    }    
  }

  pub fn debug_flags(&mut self, flags: OBXDebugFlags) {
    if let Some(store) = self.obx_store {
      let result = unsafe { obx_store_debug_flags(store, flags) };
      self.error = call(result).err();
    }
  }

  pub fn opened_with_previous_commit(&self) -> bool {
    if let Some(store) = self.obx_store {
      unsafe { obx_store_opened_with_previous_commit(store) }
    }else { false }
  }

  pub fn prepare_to_close(&mut self) {
    if let Some(store) = self.obx_store {
      let result = unsafe { obx_store_prepare_to_close(store) };
      self.error = call(result).err();
    }
  }

  // TODO implement in drop, with the others
  pub fn close(&mut self) {
    if let Some(store) = self.obx_store {
      let result = unsafe { obx_store_close(store) };
      self.error = call(result).err();
    }
  }
}