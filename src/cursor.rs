#[allow(dead_code)]

use std::ptr;

use crate::{c::{*, self}, error::Error, tx::Tx};

pub(crate) struct Cursor {
  error: Option<Error>,
  obx_cursor: *mut c::OBX_cursor
}

impl Drop for Cursor {
  fn drop(&mut self) {
    unsafe {
      if !self.obx_cursor.is_null() {
        self.error = c::call(c::obx_cursor_close(self.obx_cursor)).err();
        self.obx_cursor = std::ptr::null_mut();
      }

      if let Some(err) = &self.error {
        println!("Error: {}", err);
      }
    }
  }
}

impl Cursor {
  fn new(tx: Tx, entity_id: c::obx_schema_id) -> Self {
    match c::new_mut(unsafe { c::obx_cursor(tx.obx_txn, entity_id) }) {
      Ok(obx_cursor) => Cursor { obx_cursor, error: None },
      Err(e) => Cursor {
          obx_cursor: ptr::null_mut(),
          error: Some(e),
      },
    }
  }

  fn id_for_put(&self, id_or_zero: obx_id) -> obx_id {
      unsafe { obx_cursor_id_for_put(self.obx_cursor, id_or_zero) }
  }

  fn put(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
  ) {
      self.error = c::call(unsafe { c::obx_cursor_put(self.obx_cursor, id, data, size) }).err();
  }

  fn put4(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
      mode: OBXPutMode,
  ) {
    self.error = c::call(unsafe { obx_cursor_put4(self.obx_cursor, id, data, size, mode) }).err();
  }         

  fn put_new(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
  ) {
      self.error = c::call(unsafe { obx_cursor_put_new(self.obx_cursor, id, data, size) }).err()
  }

  fn insert(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
  ) {
      self.error = c::call(unsafe { obx_cursor_insert(self.obx_cursor, id, data, size) }).err()
  }

  fn update(
      &mut self,
      id: obx_id,
      data: *const ::std::os::raw::c_void,
      size: usize,
  ) {
      self.error = c::call(unsafe { obx_cursor_update(self.obx_cursor, id, data, size) }).err()
  }

  fn put_object(
      &self,
      data: *mut ::std::os::raw::c_void,
      size: usize,
  ) -> obx_id {
    unsafe { obx_cursor_put_object(self.obx_cursor, data, size) }
  }

  fn put_object4(
      &self,
      data: *mut ::std::os::raw::c_void,
      size: usize,
      mode: OBXPutMode,
  ) -> obx_id {
    unsafe { obx_cursor_put_object4(self.obx_cursor, data, size, mode) }
  }

  fn get(
      &mut self,
      id: obx_id,
      data: *mut *const ::std::os::raw::c_void,
      size: *mut usize,
  ) {
    self.error = c::call(unsafe { obx_cursor_get(self.obx_cursor, id, data, size) }).err()
  }

  fn get_all(&self) -> *mut OBX_bytes_array {
      unsafe { obx_cursor_get_all(self.obx_cursor) }
  }

  fn first(
      &mut self,
      data: *mut *const ::std::os::raw::c_void,
      size: *mut usize,
  ) {
      self.error = c::call(unsafe {obx_cursor_first(self.obx_cursor, data, size)}).err();
  }

  fn next(
      &mut self,
      data: *mut *const ::std::os::raw::c_void,
      size: *mut usize,
  ) {
      self.error = c::call(unsafe {obx_cursor_next(self.obx_cursor, data, size)}).err();
  }

  fn seek(&mut self, id: obx_id) {
      self.error = c::call(unsafe {obx_cursor_seek(self.obx_cursor, id)}).err();
  }

  fn current(
      &mut self,
      data: *mut *const ::std::os::raw::c_void,
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

  fn count(&mut self) -> u64 {
    let mut count: u64 = 0;
    self.error = c::call(unsafe {obx_cursor_count(self.obx_cursor, count as *mut u64)}).err();
    count
  }

  fn count_max(
      &mut self,
      max_count: u64,
  ) -> u64 {
    let mut out_count: u64 = 0;
    self.error = c::call(unsafe {obx_cursor_count_max(self.obx_cursor, max_count, out_count as *mut u64)}).err();
    out_count
  }

  // TODO fix
  // TODO test endianness
  // fn is_empty(&mut self) -> bool {
  //   let mut out_is_empty: bool = false;
  //   self.error = c::call(unsafe {obx_cursor_is_empty(self.obx_cursor, out_is_empty)}).err();
  //   out_is_empty
  // }

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
}


