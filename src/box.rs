use std::ptr;
use std::rc::Rc;
use std::slice::from_raw_parts;

#[allow(dead_code)]
use crate::c::{self, *};
use crate::error::{self, Error};

use crate::traits::{EntityFactoryExt, OBBlanket};
use crate::util::{MutConstVoidPtr, NOT_FOUND_404, SUCCESS_0};
use crate::{cursor::Cursor, txn::Tx};
use flatbuffers::FlatBufferBuilder;

// This Box type will confuse a lot of rust users of std::boxed::Box
pub struct Box<'a, T: OBBlanket> {
    pub(crate) helper: Rc<dyn EntityFactoryExt<T>>,
    pub(crate) error: Option<Error>,
    pub(crate) obx_box: *mut OBX_box,
    builder: FlatBufferBuilder<'a>,
    // pub(crate) async_: std::boxed::Box<Async>, // TODO
}

impl<T: OBBlanket> Box<'_, T> {
    pub(crate) fn new(store: *mut OBX_store, helper: Rc<dyn EntityFactoryExt<T>>) -> Self {
        unsafe {
            let obx_box = c::obx_box(store, helper.get_entity_id());

            Box {
                helper,
                error: None,
                obx_box,
                builder: FlatBufferBuilder::new(),
                // obx_store: store
            }
        }
    }

    // This should only be exposed between threads, channels, etc.
    pub(crate) fn get_store(&self) -> *mut OBX_store {
        unsafe { obx_box_store(self.obx_box) }
    }

    pub fn contains(&mut self, id: obx_id) -> error::Result<bool> {
        let mut contains = false;
        c::get_result(
            unsafe { obx_box_contains(self.obx_box, id, &mut contains) },
            contains,
        )
    }

    pub fn contains_many(&mut self, ids: &Vec<obx_id>) -> error::Result<Vec<bool>> {
        let mut r = Vec::<bool>::new();
        for id in ids {
            match self.contains(*id) {
                Ok(v) => r.push(v),
                Err(err) => err.as_result()?,
            }
        }
        Ok(r)
    }

    /*
      // TODO extension trait for Vec<u32?/OBX_id> -> OBX_id_array, see util.rs
      // TODO alternative: run contains one by one
      pub fn contains_many_id_array(&mut self, ids: *const OBX_id_array) -> bool {
          let mut contains = false;
          self.error = c::call(unsafe { obx_box_contains_many(self.obx_box, ids, &mut contains) }).err();
          contains
      }

      // TODO extension trait for mut_const_c_void -> slice -> Vec<u8> to be processed by flatbuffers
      pub fn get_data_ptr(
          &mut self,
          id: obx_id,
      ) -> (*mut *const ::std::os::raw::c_void, usize) {
          let data = std::ptr::null_mut(); // this is wrong, and will explode
          let mut size = 0;
          self.error = c::call(unsafe { obx_box_get(self.obx_box, id, data, &mut size) }).err();
          (data, size)
      }

      // TODO extension trait for Vec<u32?/OBX_id> -> &[Entity], see util.rs
      fn get_many_bytes_array(&self, ids: *const OBX_id_array) -> *mut OBX_bytes_array {
          unsafe { obx_box_get_many(self.obx_box, ids) }
      }

      // TODO convert OBX_bytes_array into &[Entity]
      fn get_all_bytes_array(&self) -> *mut OBX_bytes_array {
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

    // TODO size 16, align 8
    fn remove_many_id_array(&mut self, ids: *const OBX_id_array) -> u64 {
        let out_count: u64 = 0;
        self.error = c::call(
            unsafe { obx_box_remove_many(self.obx_box, ids, out_count as *mut u64) },
            Some("box::remove_many_id_array"),
        )
        .err();
        out_count
    }
    */

    pub fn remove_with_id(&mut self, id: obx_id) -> error::Result<bool> {
        unsafe {
            let code = obx_box_remove(self.obx_box, id);
            c::get_result(code, code == 0)
        }
    }

    pub fn remove_many(&mut self, ids: &Vec<c::obx_id>) -> error::Result<Vec<bool>> {
        let mut r = Vec::<bool>::new();
        for id in ids {
            match self.remove_with_id(*id) {
                Ok(v) => r.push(v),
                Err(err) => err.as_result()?,
            }
        }
        Ok(r)
    }

    // TODO check if this is ACID (or go with cursor instead)
    pub fn remove_all(&mut self) -> error::Result<u64> {
        unsafe {
            let out_count: *mut u64 = &mut 0;
            c::get_result(
                obx_box_remove_all(self.obx_box, out_count as *mut u64),
                *out_count,
            )
        }
    }

    pub fn is_empty(&mut self) -> error::Result<bool> {
        unsafe {
            let out_is_empty: *mut bool = &mut false; // coerce
            c::get_result(obx_box_is_empty(self.obx_box, out_is_empty), *out_is_empty)
        }
    }

    pub fn count(&mut self) -> error::Result<u64> {
        self.count_with_limit(0)
    }

    pub fn count_with_limit(&mut self, limit: u64) -> error::Result<u64> {
        unsafe {
            let out_count: *mut u64 = &mut 0;
            c::get_result(obx_box_count(self.obx_box, limit, out_count), *out_count)
        }
    }
    /*
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

        // TODO fix sooner than later
        fn visit_many(&mut self, ids: &[c::obx_id], visitor: obx_data_visitor, user_data: *mut ::std::os::raw::c_void) -> obx_err {
            unsafe {
                obx_box_visit_many(self.obx_box, ids.as_ptr(), visitor, user_data)
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
    */
    // TODO remove assertions when the code is more stable
    fn get_tx_cursor_mut(&self) -> (Tx, Cursor<T>) {
        let store = self.get_store();
        assert!(!store.is_null());

        let tx = Tx::new_mut(store);
        assert!(!tx.obx_txn.is_null());

        let cursor = Cursor::new(tx.obx_txn, self.helper.clone());
        assert!(!cursor.obx_cursor.is_null());

        (tx, cursor)
    }

    // TODO remove assertions when the code is more stable
    fn get_tx_cursor(&self) -> (Tx, Cursor<T>) {
        let store = self.get_store();
        assert!(!store.is_null());

        let tx = Tx::new(store);
        assert!(!tx.obx_txn.is_null());

        let cursor = Cursor::new(tx.obx_txn, self.helper.clone());
        assert!(!cursor.obx_cursor.is_null());

        (tx, cursor)
    }

    pub(crate) fn put_entity_in_ob(&mut self, cursor: &mut Cursor<T>, object: &mut T) -> c::obx_id {
        let old_id = object.get_id();
        let is_object_new = old_id == 0;
        let new_id = cursor.id_for_put(old_id);
        object.set_id(new_id);

        object.to_fb(&mut self.builder);
        let data = Vec::from(self.builder.finished_data());

        if is_object_new {
            cursor.put_new(new_id, &data);
        } else {
            cursor.put(new_id, &data);
        }

        new_id
    }

    pub fn put(&mut self, object: &mut T) -> error::Result<c::obx_id> {
        let (mut tx, mut cursor) = self.get_tx_cursor_mut();
        let new_id = self.put_entity_in_ob(&mut cursor, object);
        tx.success();

        if let Some(err) = &self.error {
            Err(err.clone())
        } else if let Some(err) = &tx.error {
            Err(err.clone())
        } else if let Some(err) = &cursor.error {
            Err(err.clone())
        } else {
            Ok(new_id)
        }
    }

    pub fn put_many(&mut self, objects: Vec<&mut T>) -> error::Result<Vec<c::obx_id>> {
        let (mut tx, mut cursor) = self.get_tx_cursor_mut();

        let mut vec_out = Vec::<c::obx_id>::new();
        for o in objects {
            vec_out.push(self.put_entity_in_ob(&mut cursor, o));
        }

        if let Some(err) = &self.error {
            Err(err.clone())
        } else if let Some(err) = &tx.error {
            Err(err.clone())
        } else if let Some(err) = &cursor.error {
            Err(err.clone())
        } else {
            tx.success();
            Ok(vec_out)
        }
    }

    /// For testing purposes
    pub fn count_with_cursor(&self) -> error::Result<u64> {
        let (tx, mut cursor) = self.get_tx_cursor();

        if let Some(err) = &tx.error {
            err.as_result()?;
        } else if let Some(err) = &cursor.error {
            err.as_result()?;
        }

        Ok(cursor.count())
    }

    unsafe fn from_raw_parts_to_object(
        &self,
        data_ptr_ptr: *mut *mut u8,
        size_ptr: *mut usize,
    ) -> T {
        let data_slice = from_raw_parts(*data_ptr_ptr, *size_ptr);
        let first_offset: usize = data_slice[0].into();

        assert!(
            first_offset > 0 && first_offset < *size_ptr,
            "Data from OB should be within bounds"
        );

        // TODO check speed improvement if table is recycled
        let mut table = flatbuffers::Table::new(data_slice, first_offset);
        self.helper.make(&mut table)
    }

    pub(crate) fn get_entity_from_cursor(
        &self,
        cursor: &mut Cursor<T>,
        id: c::obx_id,
    ) -> Option<T> {
        unsafe {
            let data_ptr_ptr: *mut *mut u8 = &mut ptr::null_mut();

            let size_ptr: *mut usize = &mut 0;
            let code = cursor.get(id, data_ptr_ptr as MutConstVoidPtr, size_ptr);

            // ensure first offset is within bounds

            if data_ptr_ptr.is_null() || code == NOT_FOUND_404 {
                None
            } else {
                Some(self.from_raw_parts_to_object(data_ptr_ptr, size_ptr))
            }
        }
    }

    pub fn get(&self, id: c::obx_id) -> error::Result<Option<T>> {
        let (tx, mut cursor) = self.get_tx_cursor();
        let r = self.get_entity_from_cursor(&mut cursor, id);

        if let Some(err) = &self.error {
            Err(err.clone())
        } else if let Some(err) = &tx.error {
            Err(err.clone())
        } else if let Some(err) = &cursor.error {
            Err(err.clone())
        } else {
            Ok(r)
        }
    }

    pub fn get_many(&self, ids: &[c::obx_id]) -> error::Result<Vec<Option<T>>> {
        let (tx, mut cursor) = self.get_tx_cursor();

        let mut r = Vec::<Option<T>>::new();

        for id in ids {
            r.push(self.get_entity_from_cursor(&mut cursor, *id));
        }

        if let Some(err) = &self.error {
            Err(err.clone())
        } else if let Some(err) = &tx.error {
            Err(err.clone())
        } else if let Some(err) = &cursor.error {
            Err(err.clone())
        } else {
            Ok(r)
        }
    }

    /// Returns all stored objects in this Box
    pub fn get_all(&self) -> error::Result<Vec<T>> {
        let (tx, mut cursor) = self.get_tx_cursor();

        let data_ptr_ptr: *mut *mut u8 = &mut ptr::null_mut();

        let size_ptr: *mut usize = &mut 0;

        let mut r: Vec<T> = Vec::new();

        let mut code = cursor.first(data_ptr_ptr as MutConstVoidPtr, size_ptr);

        // c::OBX_NOT_FOUND was a C #define that became a u32
        // which is incompatible with obx_err === i32
        while code != NOT_FOUND_404 {
            unsafe {
                r.push(self.from_raw_parts_to_object(data_ptr_ptr, size_ptr));
            }
            code = cursor.next(data_ptr_ptr as MutConstVoidPtr, size_ptr);

            if code != SUCCESS_0 /* c::OBX_SUCCESS */ && code != NOT_FOUND_404
            /* c::OBX_NOT_FOUND */
            {
                let err = c::call(code, Some("box::get_all")).err();
                if let Some(err) = &err {
                    cursor.error = Some(err.to_owned());
                }
                break;
            }
        }

        if let Some(err) = &self.error {
            Err(err.clone())
        } else if let Some(err) = &tx.error {
            Err(err.clone())
        } else if let Some(err) = &cursor.error {
            Err(err.clone())
        } else {
            Ok(r)
        }
    }
}
