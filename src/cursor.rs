
use std::rc::Rc;

use crate::{c::{*, self}, error::Error, traits::FactoryHelper, util::{ToCVoid, MutConstVoidPtr}};

// The best article ever on ffi
// https://blog.guillaume-gomez.fr/articles/2021-07-29+Interacting+with+data+from+FFI+in+Rust
pub(crate) struct Cursor<T> {
  helper: Rc<dyn FactoryHelper<T>>,
  pub(crate) error: Option<Error>,
  pub(crate) obx_cursor: *mut c::OBX_cursor
}

impl<T> Drop for Cursor<T> {
  fn drop(&mut self) {
    unsafe {
      if !self.obx_cursor.is_null() {
        self.error = c::call(c::obx_cursor_close(self.obx_cursor)).err();
        self.obx_cursor = std::ptr::null_mut();
      }

      if let Some(err) = &self.error {
        eprintln!("Error: cursor: {err}");
      }
    }
  }
}

impl<T> Cursor<T> {
  pub(crate) fn new(tx: *mut OBX_txn, helper: Rc<dyn FactoryHelper<T>>) -> Self {
    let entity_id = helper.get_entity_id();
    match c::new_mut(unsafe { c::obx_cursor(tx, entity_id) }) {
      Ok(obx_cursor) => Cursor {
        helper,
        obx_cursor,
        error: None
      },
      Err(err) => panic!("{err}"),
    }
  }

  pub(crate) fn id_for_put(&self, id_or_zero: obx_id) -> obx_id {
      unsafe { obx_cursor_id_for_put(self.obx_cursor, id_or_zero) }
  }

  pub(crate) fn put(
      &mut self,
      id: obx_id,
      data: &Vec<u8>,
  ) {
      self.error = c::call(unsafe { c::obx_cursor_put(self.obx_cursor, id, data.to_const_c_void(), data.len()) }).err();
  }
/*
  fn put4(
      &mut self,
      id: obx_id,
      data: &Vec<u8>, // bad idea
      mode: OBXPutMode,
  ) {
    self.error = c::call(unsafe { obx_cursor_put4(self.obx_cursor, id, data.to_const_c_void(), data.len(), mode) }).err();
  }         
*/
  pub(crate) fn put_new(
      &mut self,
      id: obx_id,
      data: &Vec<u8>,
  ) {
      self.error = c::call(unsafe { obx_cursor_put_new(self.obx_cursor, id, data.to_const_c_void(), data.len()) }).err()
  }
/*
  fn insert(
      &mut self,
      id: obx_id,
      data: &Vec<u8>, // bad idea
  ) {
      self.error = c::call(unsafe { obx_cursor_insert(self.obx_cursor, id, data.to_const_c_void(), data.len()) }).err()
  }

  fn update(
      &mut self,
      id: obx_id,
      data: &Vec<u8>, // bad idea
  ) {
      self.error = c::call(unsafe { obx_cursor_update(self.obx_cursor, id, data.to_const_c_void(), data.len()) }).err()
  }

  fn put_object(
      &self,
      data: *mut ::std::os::raw::c_void,
  ) -> obx_id {
    unsafe {
      obx_cursor_put_object(self.obx_cursor, data, len) // TODO fix if required
    }
  }

  fn put_object4(
      &self,
      data: *mut ::std::os::raw::c_void,
      mode: OBXPutMode,
  ) -> obx_id {
    unsafe { obx_cursor_put_object4(self.obx_cursor, data, data.len(), mode) }
  }
*/
  pub(crate) fn get(
      &mut self,
      id: obx_id,
      data : MutConstVoidPtr,
      size: *mut usize,
  ) {
    self.error = c::call(unsafe { obx_cursor_get(self.obx_cursor, id, data, size) }).err()
  }

  fn get_all(&self) -> *mut OBX_bytes_array {
      unsafe { obx_cursor_get_all(self.obx_cursor) }
  }

  pub(crate) fn first(
      &mut self,
      data: MutConstVoidPtr,
      size: *mut usize,
  ) -> c::obx_err {
    unsafe {
      obx_cursor_first(self.obx_cursor, data, size)
    }
  }

  pub(crate) fn next(
      &mut self,
      data: MutConstVoidPtr,
      size: *mut usize,
  ) -> c::obx_err {
      unsafe {
        obx_cursor_next(self.obx_cursor, data, size)
      }
  }

  fn seek(&mut self, id: obx_id) {
      self.error = c::call(unsafe {obx_cursor_seek(self.obx_cursor, id)}).err();
  }

  fn current(
      &mut self,
      data: MutConstVoidPtr,
      size: *mut usize,
  ) {
      self.error = c::call(unsafe {obx_cursor_current(self.obx_cursor, data, size)}).err();
  }

  fn remove(&mut self, id: obx_id) {
      self.error = c::call(unsafe {obx_cursor_remove(self.obx_cursor, id)}).err();
  }

  fn remove_all(&mut self) {
      self.error = c::call(unsafe {obx_cursor_remove_all(self.obx_cursor)}).err();
  }

  pub(crate)fn count(&mut self) -> u64 {
    unsafe {
      let count: *mut u64 = &mut 0;
      self.error = c::call(obx_cursor_count(self.obx_cursor, count as *mut u64)).err();
      *count  
    }
  }

  pub(crate) fn count_max(
      &mut self,
      max_count: u64,
  ) -> u64 {
    unsafe {
      let count: *mut u64 = &mut 0;
      self.error = c::call(obx_cursor_count_max(self.obx_cursor, max_count, count as *mut u64)).err();
      *count
    }
  }

  // TODO Determine: do we need a Tx for is_empty? Or just use the box
  // TODO test endianness
  fn is_empty(&mut self) -> bool {
    unsafe {
      let out_is_empty: *mut bool = &mut false; // coerce
      self.error = c::call(obx_cursor_is_empty(self.obx_cursor, out_is_empty as *mut bool)).err();
      *out_is_empty  
    }
  }

  fn backlinks(
      &self,
      entity_id: obx_schema_id,
      property_id: obx_schema_id,
      id: obx_id,
  ) -> *mut OBX_bytes_array {
      unsafe { obx_cursor_backlinks(self.obx_cursor, entity_id, property_id, id) }
  }

  fn backlink_ids(
      &self,
      entity_id: obx_schema_id,
      property_id: obx_schema_id,
      id: obx_id,
  ) -> *mut OBX_id_array {
    unsafe { obx_cursor_backlink_ids(self.obx_cursor, entity_id, property_id, id) }
  }

  fn rel_put(
      &mut self,
      relation_id: obx_schema_id,
      source_id: obx_id,
      target_id: obx_id,
  ) {
      self.error = c::call(unsafe {obx_cursor_rel_put(self.obx_cursor, relation_id, source_id, target_id)}).err();
  }

  fn rel_remove(&mut self, relation_id: obx_schema_id, source_id: obx_id, target_id: obx_id) {
    self.error = c::call(unsafe {obx_cursor_rel_remove(self.obx_cursor, relation_id, source_id, target_id)}).err();
  }

  fn rel_ids(&self, relation_id: obx_schema_id, source_id: obx_id) -> *mut OBX_id_array {
      unsafe { obx_cursor_rel_ids(self.obx_cursor, relation_id, source_id) }
  }

  /*
  fn ts_min_max(&mut self) -> (obx_id, i64, obx_id, i64) {
      let mut min_id: obx_id = 0;
      let mut min_value: i64 = 0;
      let mut max_id: obx_id = 0;
      let mut max_value: i64 = 0;
      self.error = c::call(unsafe {obx_cursor_ts_min_max(self.obx_cursor, &mut min_id, &mut min_value, &mut max_id, &mut max_value) }).err();
      (min_id, min_value, max_id, max_value)
  }

  fn ts_min_max_range(&mut self, range_begin: i64, range_end: i64) -> (obx_id, i64, obx_id, i64) {
      let mut min_id: obx_id = 0;
      let mut min_value: i64 = 0;
      let mut max_id: obx_id = 0;
      let mut max_value: i64 = 0;
      self.error = c::call(unsafe {obx_cursor_ts_min_max_range(self.obx_cursor, range_begin, range_end, &mut min_id, &mut min_value, &mut max_id, &mut max_value) }).err();
      (min_id, min_value, max_id, max_value)
  }
  */
}


