#[allow(dead_code)]

use crate::c::{*, self};
use crate::error::Error;
use crate::store::Store;

/// _Not_ feasible initial idea: this is not known ahead of time by store and box
/// because it needs to be generated at the same crate and module,
/// before store and box are also compiled.
/// Also it's another abstraction layer.
// TODO reformat the following code block properly
/// impl<T> Entity {
///   fn to_FB(self, builder: &fb.Builder);
///   fn from_FB(store: &mut Store, byte_buffer: &ByteBuffer) -> T;
///   fn get_id(&self) -> u64;
///   fn set_id(&mut self, id: u64);
///   fn get_type(&self) -> std::any::TypeId;
///   fn to_one_relations(&self) -> ...
///   fn to_many_relations(&self) -> ...
/// }

// TODO
/// My gut feeling says use extension trait on the Entity directly
/// since the closure signatures all suggest that,
/// except objectFromOB from the dart impl, which could be an Entity trait factory
/// with signature: Entity::fromFB(store, fbData).
/// During compile time, the store or box only is concerned about
/// that the traits are implemented on the object being passed
/// in compile time, not runtime (unlike dart's impl)
/// In this case, we eliminate the need for ModelDefinition and EntityDefiniton
/// All we need are cross-concern cutting traits, and pass those instances around as mut refs.


// This Box type will confuse a lot of rust users of std::boxed::Box
pub struct Box<T> {
  _place_holder: Option<T>,
  error: Option<Error>,
  obx_box: *mut OBX_box,
  // obx_async: *mut OBX_async, // TODO
}

impl<T> Box<T> {
  fn new(store: Store, entity_id: c::obx_schema_id) -> Self {
    unsafe {
      let obx_box = c::obx_box(store.obx_store, entity_id);

      Box {
        _place_holder: None,
        error: None,
        obx_box,
      }
    }
  }

  fn get_store(&self) -> *mut OBX_store {
    unsafe {
        obx_box_store(self.obx_box)
    }
  }

  fn contains(&mut self, id: obx_id) -> bool {
    let mut contains = false;
    self.error = c::call(unsafe { obx_box_contains(self.obx_box, id, &mut contains) }).err();
    contains
  }

  fn contains_many(&mut self, ids: *const OBX_id_array) -> bool {
      let mut contains = false;
      self.error = c::call(unsafe { obx_box_contains_many(self.obx_box, ids, &mut contains) }).err();
      contains
  }

  fn get(
      &mut self,
      id: obx_id,
  ) -> (*mut *const ::std::os::raw::c_void, usize) {
      let mut data = std::ptr::null_mut();
      let mut size = 0;
      self.error = c::call(unsafe { obx_box_get(self.obx_box, id, data, &mut size) }).err();
      (data, size)
  }

  fn get_many(&self, ids: *const OBX_id_array) -> *mut OBX_bytes_array {
      unsafe { obx_box_get_many(self.obx_box, ids) }
  }

  fn get_all(&self) -> *mut OBX_bytes_array {
      unsafe { obx_box_get_all(self.obx_box) }
  }

  fn id_for_put(&self, id_or_zero: obx_id) -> obx_id {
      unsafe { obx_box_id_for_put(self.obx_box, id_or_zero) }
  }

  fn ids_for_put(&mut self, count: u64) -> obx_id {
      let mut first_id = 0;
      self.error = c::call(unsafe { obx_box_ids_for_put(self.obx_box, count, &mut first_id) }).err();
      first_id
  }

  fn put(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
  ) {
    self.error = c::call(unsafe { obx_box_put(self.obx_box, id, data, size) }).err();
  }

  fn insert(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
  ) {
    self.error = c::call(unsafe { obx_box_insert(self.obx_box, id, data, size) }).err();
  }

  fn update(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
  ) {
    self.error = c::call(unsafe { obx_box_update(self.obx_box, id, data, size) }).err();
  }

  fn put5(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
      mode: OBXPutMode,
  ) {
    self.error = c::call(unsafe { obx_box_put5(self.obx_box, id, data, size, mode) }).err();
  }

  fn put_object(
      &self,
      data: *mut ::std::os::raw::c_void,
      size: usize,
  ) -> obx_id {
      unsafe { obx_box_put_object(self.obx_box, data, size) }
  }

  fn put_object4(
      &self,
      data: *mut ::std::os::raw::c_void,
      size: usize,
      mode: OBXPutMode,
  ) -> obx_id {
      unsafe { obx_box_put_object4(self.obx_box, data, size, mode) }
  }

  fn put_many(
      &mut self,
      objects: *const OBX_bytes_array,
      ids: *const obx_id,
      mode: OBXPutMode,
  ) {
    self.error = c::call(unsafe { obx_box_put_many(self.obx_box, objects, ids, mode) }).err();
  }

  fn put_many5(&mut self, objects: *const OBX_bytes_array, ids: *const obx_id, mode: OBXPutMode, fail_on_id_failure: bool) {
    self.error = c::call(unsafe { obx_box_put_many5(self.obx_box, objects, ids, mode, fail_on_id_failure) }).err();
  }

  fn remove(&mut self, id: obx_id) {
    self.error = c::call(unsafe { obx_box_remove(self.obx_box, id) }).err();
  }

  fn remove_many(&mut self, ids: *const OBX_id_array) -> u64 {
    let mut out_count: u64 = 0;
    self.error = c::call(unsafe { obx_box_remove_many(self.obx_box, ids, out_count as *mut u64) }).err();
    out_count
  }

  fn remove_all(&mut self) -> u64 {
    let out_count: u64 = 0;
    self.error = c::call(unsafe { obx_box_remove_all(self.obx_box, out_count as *mut u64) }).err();
    out_count
  }

  // TODO fix boolean cast to *mut bool
  // fn is_empty(&self) -> bool {
  //   let mut out_is_empty = false;
  //   self.error = c::call(unsafe { obx_box_is_empty(self.obx_box, out_is_empty) }).err();
  //   out_is_empty
  // }

  fn count(&mut self, limit: u64) -> u64 {
    let out_count: u64 = 0;
    self.error = c::call(unsafe { obx_box_count(self.obx_box, limit, out_count as *mut u64) }).err();
    out_count
  }

  fn get_backlink_ids(&self, property_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
      unsafe { obx_box_get_backlink_ids(self.obx_box, property_id, id) }
  }

  fn rel_put(&self, relation_id: obx_schema_id, source_id: obx_id, target_id: obx_id) -> obx_err {
      unsafe { obx_box_rel_put(self.obx_box, relation_id, source_id, target_id) }
  }

  fn rel_remove(&self, relation_id: obx_schema_id, source_id: obx_id, target_id: obx_id) -> obx_err {
      unsafe { obx_box_rel_remove(self.obx_box, relation_id, source_id, target_id) }
  }

  fn rel_get_ids(&self, relation_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
      unsafe { obx_box_rel_get_ids(self.obx_box, relation_id, id) }
  }

  // TODO fix later
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

  fn visit_all(&mut self, visitor: obx_data_visitor, user_data: *mut ::std::os::raw::c_void) -> obx_err {
    unsafe {
        obx_box_visit_all(self.obx_box, visitor, user_data)
    }
  }

  // TODO fix later
  // fn async_remove(&mut self, id: obx_id) -> obx_err {
  //   unsafe {
  //       obx_async_remove(self.obx_async, id)
  //   }
  // }

  // fn async_create(&mut self, enqueue_timeout_millis: u64) -> *mut OBX_async {
  //   unsafe {
  //       obx_async_create(self.obx_async, enqueue_timeout_millis)
  //   }
  // }

  // fn async_close(&mut self) -> obx_err {
  //   unsafe {
  //       obx_async_close(self.obx_async)
  //   }
  // }

  fn rel_get_backlink_ids(&mut self, relation_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
    unsafe {
        obx_box_rel_get_backlink_ids(self.obx_box, relation_id, id)
    }
  }

  fn ts_min_max(&mut self, out_min_id: *mut obx_id, out_min_value: *mut i64, out_max_id: *mut obx_id, out_max_value: *mut i64) -> obx_err {
    unsafe {
        obx_box_ts_min_max(self.obx_box, out_min_id, out_min_value, out_max_id, out_max_value)
    }
  }

  fn ts_min_max_range(&mut self, range_begin: i64, range_end: i64, out_min_id: *mut obx_id, out_min_value: *mut i64, out_max_id: *mut obx_id, out_max_value: *mut i64) -> obx_err {
    unsafe {
        obx_box_ts_min_max_range(self.obx_box, range_begin, range_end, out_min_id, out_min_value, out_max_id, out_max_value)
    }
  }

  // TODO create async wrapper
  // reserved keyword
  // fn async_(&mut self) -> *mut OBX_async {
  //   unsafe {
  //       obx_async(self.obx_box)
  //   }
  // }
}



