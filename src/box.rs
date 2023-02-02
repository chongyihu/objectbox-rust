use std::rc::Rc;

#[allow(dead_code)]

use crate::c::{*, self};
use crate::error::Error;
use crate::store::Store;
use crate::traits::{FactoryHelper, Factory};
use crate::util::ToCVoid;

// This Box type will confuse a lot of rust users of std::boxed::Box
pub struct Box<T> {
  pub(crate) helper: Rc<dyn FactoryHelper<T>>,
  pub(crate) error: Option<Error>,
  pub(crate) obx_box: *mut OBX_box,
  // pub(crate) async_: std::boxed::Box<Async>, // TODO
}

impl<T> Box<T> {
  pub(crate) fn new(store: &Store, helper: Rc<dyn FactoryHelper<T>>) -> Self {
    unsafe {
      let obx_box = c::obx_box(store.obx_store, helper.get_entity_id());

      Box {
        helper,
        error: None,
        obx_box,
      }
    }
  }

  // This should only be exposed between threads, channels, etc.
  pub(crate) fn get_store(&self) -> *mut OBX_store {
    unsafe {
        obx_box_store(self.obx_box)
    }
  }

  pub fn contains(&mut self, id: obx_id) -> bool {
    let mut contains = false;
    self.error = c::call(unsafe { obx_box_contains(self.obx_box, id, &mut contains) }).err();
    contains
  }

  // TODO extension trait for Vec<u32?/OBX_id> -> OBX_id_array, see util.rs
  pub fn contains_many(&mut self, ids: *const OBX_id_array) -> bool {
      let mut contains = false;
      self.error = c::call(unsafe { obx_box_contains_many(self.obx_box, ids, &mut contains) }).err();
      contains
  }

  // TODO extension trait for mut_const_c_void -> slice -> Vec<u8> to be processed by flatbuffers
  pub fn get(
      &mut self,
      id: obx_id,
  ) -> (*mut *const ::std::os::raw::c_void, usize) {
      let data = std::ptr::null_mut();
      let mut size = 0;
      self.error = c::call(unsafe { obx_box_get(self.obx_box, id, data, &mut size) }).err();
      (data, size)
  }

  // TODO extension trait for Vec<u32?/OBX_id> -> &[Entity], see util.rs
  pub fn get_many(&self, ids: *const OBX_id_array) -> *mut OBX_bytes_array {
      unsafe { obx_box_get_many(self.obx_box, ids) }
  }

  // TODO convert OBX_bytes_array into &[Entity]
  pub fn get_all(&self) -> *mut OBX_bytes_array {
      unsafe { obx_box_get_all(self.obx_box) }
  }

  pub fn id_for_put(&self, id_or_zero: obx_id) -> obx_id {
      unsafe { obx_box_id_for_put(self.obx_box, id_or_zero) }
  }

  pub fn ids_for_put(&mut self, count: u64) -> obx_id {
      let mut first_id = 0;
      self.error = c::call(unsafe { obx_box_ids_for_put(self.obx_box, count, &mut first_id) }).err();
      first_id
  }

  pub fn put_vec_u8(
      &mut self,
      id: obx_id,
      data: &Vec<u8>,
  ) {
    self.error = c::call(unsafe { obx_box_put(self.obx_box, id, data.to_const_c_void(), data.len()) }).err();
  }

  pub fn insert_vec_u8(
      &mut self,
      id: obx_id,
      data: &Vec<u8>,
  ) {
    self.error = c::call(unsafe { obx_box_insert(self.obx_box, id, data.to_const_c_void(), data.len()) }).err();
  }

  pub fn update_vec_u8(
      &mut self,
      id: obx_id,
      data: &Vec<u8>,
  ) {
    self.error = c::call(unsafe { obx_box_update(self.obx_box, id, data.to_const_c_void(), data.len()) }).err();
  }

  pub fn put5_vec_u8(
      &mut self,
      id: obx_id,
      data: &Vec<u8>,
      mode: OBXPutMode,
  ) {
    self.error = c::call(unsafe { obx_box_put5(self.obx_box, id, data.to_const_c_void(), data.len(), mode) }).err();
  }

  pub fn put_object(
      &self,
      data: &mut Vec<u8>,
  ) -> obx_id {
      unsafe { obx_box_put_object(self.obx_box, data.to_mut_c_void(), data.len()) }
  }

  pub fn put_object4(
      &self,
      data: &mut Vec<u8>,
      mode: OBXPutMode,
  ) -> obx_id {
      unsafe { obx_box_put_object4(self.obx_box, data.to_mut_c_void(), data.len(), mode) }
  }

  pub fn put_many_bytes_array(
      &mut self,
      objects: *const OBX_bytes_array,
      ids: *const obx_id,
      mode: OBXPutMode,
  ) {
    self.error = c::call(unsafe { obx_box_put_many(self.obx_box, objects, ids, mode) }).err();
  }

  pub fn put_many5_bytes_array(&mut self, objects: *const OBX_bytes_array, ids: *const obx_id, mode: OBXPutMode, fail_on_id_failure: bool) {
    self.error = c::call(unsafe { obx_box_put_many5(self.obx_box, objects, ids, mode, fail_on_id_failure) }).err();
  }

  pub fn remove_with_id(&mut self, id: obx_id) {
    self.error = c::call(unsafe { obx_box_remove(self.obx_box, id) }).err();
  }

  pub fn remove_many_id_array(&mut self, ids: *const OBX_id_array) -> u64 {
    let out_count: u64 = 0;
    self.error = c::call(unsafe { obx_box_remove_many(self.obx_box, ids, out_count as *mut u64) }).err();
    out_count
  }

  pub fn remove_all(&mut self) -> u64 {
    let out_count: u64 = 0;
    self.error = c::call(unsafe { obx_box_remove_all(self.obx_box, out_count as *mut u64) }).err();
    out_count
  }

  pub fn is_empty(&mut self) -> bool {
    unsafe {
      let out_is_empty: *mut bool = &mut false; // coerce
      self.error = c::call(obx_box_is_empty(self.obx_box, out_is_empty)).err();
      *out_is_empty
    }
  }

  pub fn count_with_limit(&mut self, limit: u64) -> u64 {
    let out_count: u64 = 0;
    self.error = c::call(unsafe { obx_box_count(self.obx_box, limit, out_count as *mut u64) }).err();
    out_count
  }

  pub fn get_backlink_ids(&self, property_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
      unsafe { obx_box_get_backlink_ids(self.obx_box, property_id, id) }
  }

  pub fn rel_put(&self, relation_id: obx_schema_id, source_id: obx_id, target_id: obx_id) -> obx_err {
      unsafe { obx_box_rel_put(self.obx_box, relation_id, source_id, target_id) }
  }

  pub fn rel_remove(&self, relation_id: obx_schema_id, source_id: obx_id, target_id: obx_id) -> obx_err {
      unsafe { obx_box_rel_remove(self.obx_box, relation_id, source_id, target_id) }
  }

  pub fn rel_get_ids(&self, relation_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
      unsafe { obx_box_rel_get_ids(self.obx_box, relation_id, id) }
  }

  // TODO convert user_data to Vec<u8>
  pub fn visit_all(&mut self, visitor: obx_data_visitor, user_data: *mut ::std::os::raw::c_void) -> obx_err {
    unsafe {
        obx_box_visit_all(self.obx_box, visitor, user_data)
    }
  }

  pub fn rel_get_backlink_ids(&mut self, relation_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
    unsafe {
        obx_box_rel_get_backlink_ids(self.obx_box, relation_id, id)
    }
  }

  pub fn ts_min_max(&mut self, out_min_id: *mut obx_id, out_min_value: *mut i64, out_max_id: *mut obx_id, out_max_value: *mut i64) -> obx_err {
    unsafe {
        obx_box_ts_min_max(self.obx_box, out_min_id, out_min_value, out_max_id, out_max_value)
    }
  }

  pub fn ts_min_max_range(&mut self, range_begin: i64, range_end: i64, out_min_id: *mut obx_id, out_min_value: *mut i64, out_max_id: *mut obx_id, out_max_value: *mut i64) -> obx_err {
    unsafe {
        obx_box_ts_min_max_range(self.obx_box, range_begin, range_end, out_min_id, out_min_value, out_max_id, out_max_value)
    }
  }
}


struct Async {
  obx_async: *mut OBX_async
}

impl Async {
  // TODO create async wrapper
  // reserved keyword
  // pub fn async_(&mut self) -> *mut OBX_async {
  //   unsafe {
  //       obx_async(self.obx_box)
  //   }
  // }

  // TODO fix later
  // pub fn async_remove(&mut self, id: obx_id) -> obx_err {
  //   unsafe {
  //       obx_async_remove(self.obx_async, id)
  //   }
  // }

  // pub fn async_create(&mut self, enqueue_timeout_millis: u64) -> *mut OBX_async {
  //   unsafe {
  //       obx_async_create(self.obx_async, enqueue_timeout_millis)
  //   }
  // }

  // pub fn async_close(&mut self) -> obx_err {
  //   unsafe {
  //       obx_async_close(self.obx_async)
  //   }
  // }


  // TODO put in its own Type, with its own Drop
  // fn async_put(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
  //     unsafe { obx_async_put(async_, id, data, size) }
  // }

  // fn async_put5(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize, mode: OBXPutMode) -> obx_err {
  //     unsafe { obx_async_put5(async_, id, data, size, mode) }
  // }

  // fn async_insert(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
  //     unsafe { obx_async_insert(async_, id, data, size) }
  // }

  // fn async_update(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
  //     unsafe { obx_async_update(async_, id, data, size) }
  // }

  // fn async_put_object(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize) -> obx_id {
  //     unsafe { obx_async_put_object(async_, data, size) }
  // }

  // fn async_put_object4(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize, mode: OBXPutMode) -> obx_id {
  //     unsafe { obx_async_put_object4(async_, data, size, mode) }
  // }

  // fn async_insert_object(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize) -> obx_id {
  //     unsafe { obx_async_insert_object(async_, data, size) }
  // }

  // TODO fix sooner than later
  // fn visit_many(&mut self, ids: &[c::obx_id], visitor: obx_data_visitor, user_data: *mut ::std::os::raw::c_void) -> obx_err {
  //   unsafe {
  //       obx_box_visit_many(self.obx_box, ids.as_ptr(), visitor, user_data)
  //   }
  // }

}