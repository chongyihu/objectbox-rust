#![allow(dead_code)]
use std::path::Path;
use std::rc::Rc;

use anymap::AnyMap;

use crate::c::{self, *};
use crate::error::{self, Error};

use crate::opt::Opt;
use crate::traits::{EntityFactoryExt, OBBlanket};
use crate::util::ToCChar;

// Caveat: copy and drop are mutually exclusive

pub struct Store {
    pub trait_map: AnyMap, // passed as a ref to a Box
    // TODO confirm: model and opt are cleaned up already and zero'ed, or else we'll have a double-free
    // TODO get rid of all panics
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
    /// Assumes ownership of map, and Opt,
    pub fn new(mut opt: Opt, map: AnyMap) -> error::Result<Self> {
        unsafe {
            let obx_store = obx_store_open(opt.obx_opt);
            // This prevents a double free
            opt.ptr_consumed = !obx_store.is_null();
            let r = Store {
                trait_map: map,
                error: None,
                obx_store,
            };
            c::get_result_from_ptr(obx_store, r)
        }
    }

    pub fn get_box<T: 'static + OBBlanket>(&self) -> error::Result<crate::r#box::Box<T>> {
        let helper = if let Some(h) = self.trait_map.get::<Rc<dyn EntityFactoryExt<T>>>() {
            h
        } else {
            Error::new_local("Error: unable to get entity helper").as_result()?
        };
        Ok(crate::r#box::Box::<T>::new(self.obx_store, helper.clone()))
    }

    pub fn is_open(path: &Path) -> bool {
        unsafe { obx_store_is_open(path.as_c_char_ptr()) }
    }

    // TODO support later
    /*
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
            obx_store: unsafe {
                obx_store_attach_or_open(opt, check_matching_options, out_attached)
            },
            error: None,
            trait_map: None,
        }
    }
    */

    // TODO Determine if this is safe
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

    // TODO improve possible error handling, replace panic!
    /*
    fn set_entity_id(&self, entity_name: &str) -> obx_schema_id {
        unsafe {
            let c_str = if let Ok(r) = CString::new(entity_name) {
                r.as_ptr()
            } else {
                panic!("Error: unable to convert entity name");
            };
            obx_store_entity_id(self.obx_store, c_str)
        }
    }

    // TODO improve possible error handling, replace panic!
    fn entity_property_id(
        &self,
        entity_id: obx_schema_id,
        property_name: &str,
    ) -> obx_schema_id {
        unsafe {
            let c_str = if let Ok(r) = CString::new(property_name) {
                r.as_ptr()
            } else {
                panic!("Error: unable to convert property name");
            };
            obx_store_entity_property_id(self.obx_store, entity_id, c_str)
        }
    }
    */

    /*
    pub fn await_async_completion(&self) -> bool {
        unsafe { obx_store_await_async_completion(self.obx_store) }
    }

    pub fn await_async_submitted(&self) -> bool {
        unsafe { obx_store_await_async_submitted(self.obx_store) }
    }
    */

    pub fn debug_flags(&mut self, flags: OBXDebugFlags) -> &Self {
        self.error = c::call(
            unsafe { obx_store_debug_flags(self.obx_store, flags) },
            Some("store::debug_flags"),
        )
        .err();
        self
    }

    pub fn opened_with_previous_commit(&self) -> error::Result<bool> {
        let r = unsafe { obx_store_opened_with_previous_commit(self.obx_store) };
        if let Some(err) = &self.error {
            err.as_result()?;
        }
        Ok(r)
    }

    fn prepare_to_close(&mut self) -> &Self {
        self.error = c::call(
            unsafe { obx_store_prepare_to_close(self.obx_store) },
            Some("store::prepare_to_close"),
        )
        .err();
        self
    }

    fn close(&mut self) {
        self.error = c::call(
            unsafe { obx_store_close(self.obx_store) },
            Some("store::close"),
        )
        .err();
    }
}
