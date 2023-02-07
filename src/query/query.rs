// TODO implement Drop on QueryProperty etc.
// TODO whichever holds a C ptr, that has a free/close C fn
//
// Reminder:
// Expression in dart: box.query(i.greaterThan(0)).build().property(pq);
// box.query -> QueryBuilder
// i -> QueryProperty (QP)
// i.greaterThan(0) -> Condition
// ..build() -> Query
// ..property(j) -> PropertyQuery (PQ) PQ vs QP are confusing as hell, I named it, mea culpa
// j -> QP (like i)
//
// Traits to reuse: https://doc.rust-lang.org/std/ops/index.html
// Ops: https://doc.rust-lang.org/book/appendix-02-operators.html
/*
enum _ConditionOp {
  isNull, // TODO only feasible when Option<OB_Rust_Primitive> is introduced
  notNull, // TODO only feasible when Option<OB_Rust_Primitive> is introduced
  eq, // std::cmp::PartialEq, eq
  notEq, // std::ops::PartialEq, ne
  contains,
  containsElement,
  startsWith,
  endsWith,
  gt, // std::cmp::PartialOrd, gt
  greaterOrEq, // std::cmp::PartialOrd, ge
  lt, // std::cmp::PartialOrd, lt
  lessOrEq, // std::cmp::PartialOrd, le
  oneOf,
  notOneOf,
  between,
}

// TODO even better, check predicates: https://docs.rs/predicates/2.1.5/predicates/index.html
// e.g. box.query(qp)

// For lack of variadic args on .query(), use query(vec!(condition...));
*/

// TODO write macro for boilerplate: fn obx_query_something(...) -> obx_err, rewrite to call,
// TODO also error check before chaining the next call
// TODO depending on property type, allow only certain calls at compile time?
// TODO compile time determined extension blanket traits?

use std::ffi::CStr;
use std::ptr;

use crate::c::*;
use crate::error;
use crate::util::ConstVoidPtr;
use crate::util::MutConstVoidPtr;
use crate::util::PtrConstChar;

impl Drop for Query {
    fn drop(&mut self) {
        if !self.obx_query.is_null() {
            // always close regardless, no flags to set, no potential double frees
            self.close();
            self.obx_query = ptr::null_mut();
        }

        if let Some(err) = &self.error {
            eprintln!("Error: async: {err}");
        }
    }
}

pub struct Query {
    error: Option<error::Error>,
    pub(crate) obx_query: *mut OBX_query,
}

impl Query {
    pub(crate) fn from_query_builder(builder: *mut OBX_query_builder) -> Self {
        unsafe {
            Self {
                obx_query: obx_query(builder),
                error: None,
            }
        }
    }

    fn close(&mut self) -> obx_err {
        unsafe { obx_query_close(self.obx_query) }
    }

    // TODO potential double frees here?
    // No Clone trait here, because that implies Copy,
    // which prevents using Drop
    fn clone(&self) -> Query {
        unsafe {
            Query {
                obx_query: obx_query_clone(self.obx_query),
                error: None,
            }
        }
    }

    /// Paging related
    pub(crate) unsafe fn offset(&mut self, offset: usize) -> obx_err {
        obx_query_offset(self.obx_query, offset)
    }

    /// Paging related
    pub(crate) unsafe fn offset_limit(&mut self, offset: usize, limit: usize) -> obx_err {
        obx_query_offset_limit(self.obx_query, offset, limit)
    }

    /// Paging related
    pub(crate) unsafe fn limit(&mut self, limit: usize) -> obx_err {
        obx_query_limit(self.obx_query, limit)
    }

    pub(crate) unsafe fn find(&mut self) -> *mut OBX_bytes_array {
        obx_query_find(self.obx_query)
    }

    pub(crate) unsafe fn find_first(
        &mut self,
        data: MutConstVoidPtr,
        size: *mut usize,
    ) -> obx_err {
        obx_query_find_first(self.obx_query, data, size)
    }

    pub(crate) unsafe fn find_unique(
        &mut self,
        data: MutConstVoidPtr,
        size: *mut usize,
    ) -> obx_err {
        obx_query_find_unique(self.obx_query, data, size)
    }

    // TODO pass a closure to this in the pub fn impl
    pub(crate) unsafe fn visit(
        &mut self,
        visitor: obx_data_visitor,
        user_data: *mut ::std::os::raw::c_void,
    ) -> obx_err {
        obx_query_visit(self.obx_query, visitor, user_data)
    }

    pub(crate) unsafe fn find_ids(&mut self) -> *mut OBX_id_array {
        obx_query_find_ids(self.obx_query)
    }

    pub(crate) unsafe fn count(&mut self, out_count: *mut u64) -> obx_err {
        obx_query_count(self.obx_query, out_count)
    }

    pub(crate) unsafe fn remove(&mut self, out_count: *mut u64) -> obx_err {
        obx_query_remove(self.obx_query, out_count)
    }

    /// For testing and debugging
    pub fn describe(&mut self) -> Result<&str, std::str::Utf8Error> {
        unsafe {
            let out_ptr = obx_query_describe(self.obx_query);
            if out_ptr.is_null() {
              // TODO map error to error::Result?
            }
            let c_str = CStr::from_ptr(out_ptr);
            c_str.to_str() // map error?
          }
      }
  

    /// For testing and debugging
    pub fn describe_params(&mut self) -> Result<&str, std::str::Utf8Error> {
        unsafe {
          let out_ptr = obx_query_describe_params(self.obx_query);
          if out_ptr.is_null() {
            // TODO fetch error
            // either: Err(Error()) or roundabout way by giving an error to ob first
          }
          let c_str = CStr::from_ptr(out_ptr);
          c_str.to_str()
        }
    }

    // TODO create tx and cursor boilerplate macro
    pub(crate) unsafe fn cursor_visit(
        &mut self,
        cursor: &mut OBX_cursor,
        visitor: obx_data_visitor,
        user_data: *mut ::std::os::raw::c_void,
    ) -> obx_err {
        obx_query_cursor_visit(self.obx_query, cursor, visitor, user_data)
    }

    pub(crate) unsafe fn cursor_find(&mut self, cursor: &mut OBX_cursor) -> *mut OBX_bytes_array {
        obx_query_cursor_find(self.obx_query, cursor)
    }

    pub(crate) unsafe fn cursor_find_ids(&mut self, cursor: &mut OBX_cursor) -> *mut OBX_id_array {
        obx_query_cursor_find_ids(self.obx_query, cursor)
    }

    pub(crate) unsafe fn cursor_count(
        &mut self,
        cursor: &mut OBX_cursor,
        out_count: *mut u64,
    ) -> obx_err {
        obx_query_cursor_count(self.obx_query, cursor, out_count)
    }

    pub(crate) unsafe fn cursor_remove(
        &mut self,
        cursor: &mut OBX_cursor,
        out_count: *mut u64,
    ) -> obx_err {
        obx_query_cursor_remove(self.obx_query, cursor, out_count)
    }
    // end cursor

    pub(crate) unsafe fn param_string(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: PtrConstChar,
    ) -> obx_err {
        obx_query_param_string(self.obx_query, entity_id, property_id, value)
    }

    // For strings
    pub(crate) unsafe fn param_2strings(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: PtrConstChar,
        value2: PtrConstChar,
    ) -> obx_err {
        obx_query_param_2strings(self.obx_query, entity_id, property_id, value, value2)
    }

    pub(crate) unsafe fn param_strings(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        values: *const PtrConstChar, // ptr ptr === array of CString
        count: usize,
    ) -> obx_err {
        obx_query_param_strings(self.obx_query, entity_id, property_id, values, count)
    }

    // For ints
    pub(crate) unsafe fn param_int(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: i64,
    ) -> obx_err {
        obx_query_param_int(self.obx_query, entity_id, property_id, value)
    }

    pub(crate) unsafe fn param_2ints(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value_a: i64,
        value_b: i64,
    ) -> obx_err {
        obx_query_param_2ints(
            self.obx_query,
            entity_id,
            property_id,
            value_a,
            value_b,
        )
    }

    pub(crate) unsafe fn param_int64s(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        values: *const i64,
        count: usize,
    ) -> obx_err {
        obx_query_param_int64s(self.obx_query, entity_id, property_id, values, count)
    }

    pub(crate) unsafe fn param_int32s(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        values: *const i32,
        count: usize,
    ) -> obx_err {
        obx_query_param_int32s(self.obx_query, entity_id, property_id, values, count)
    }

    // For doubles
    pub(crate) unsafe fn param_double(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: f64,
    ) -> obx_err {
        obx_query_param_double(self.obx_query, entity_id, property_id, value)
    }

    pub(crate) unsafe fn param_2doubles(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value_a: f64,
        value_b: f64,
    ) -> obx_err {
        obx_query_param_2doubles(
            self.obx_query,
            entity_id,
            property_id,
            value_a,
            value_b,
        )
    }

    // For bytes
    pub(crate) unsafe fn param_bytes(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: ConstVoidPtr,
        size: usize,
    ) -> obx_err {
        obx_query_param_bytes(self.obx_query, entity_id, property_id, value, size)
    }

    pub(crate) unsafe fn param_get_type_size(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
    ) -> usize {
        obx_query_param_get_type_size(self.obx_query, entity_id, property_id)
    }

    // For aliases
    pub(crate) unsafe fn param_alias_string(
        &mut self,
        alias: PtrConstChar,
        value: PtrConstChar,
    ) -> obx_err {
        obx_query_param_alias_string(self.obx_query, alias, value)
    }

    pub(crate) unsafe fn param_alias_strings(
        &mut self,
        alias: PtrConstChar,
        values: *const PtrConstChar,
        count: usize,
    ) -> obx_err {
        obx_query_param_alias_strings(self.obx_query, alias, values, count)
    }

    pub(crate) unsafe fn param_alias_int(
        &mut self,
        alias: PtrConstChar,
        value: i64,
    ) -> obx_err {
        obx_query_param_alias_int(self.obx_query, alias, value)
    }

    pub(crate) unsafe fn param_alias_2ints(
        &mut self,
        alias: PtrConstChar,
        value_a: i64,
        value_b: i64,
    ) -> obx_err {
        obx_query_param_alias_2ints(self.obx_query, alias, value_a, value_b)
    }

    pub(crate) unsafe fn param_alias_int64s(
        &mut self,
        alias: PtrConstChar,
        values: *const i64,
        count: usize,
    ) -> obx_err {
        obx_query_param_alias_int64s(self.obx_query, alias, values, count)
    }

    pub(crate) unsafe fn param_alias_int32s(
        &mut self,
        alias: PtrConstChar,
        values: *const i32,
        count: usize,
    ) -> obx_err {
        obx_query_param_alias_int32s(self.obx_query, alias, values, count)
    }

    pub(crate) unsafe fn param_alias_double(
        &mut self,
        alias: PtrConstChar,
        value: f64,
    ) -> obx_err {
        obx_query_param_alias_double(self.obx_query, alias, value)
    }

    pub(crate) unsafe fn param_alias_2doubles(
        &mut self,
        alias: PtrConstChar,
        value_a: f64,
        value_b: f64,
    ) -> obx_err {
        obx_query_param_alias_2doubles(self.obx_query, alias, value_a, value_b)
    }

    pub(crate) unsafe fn param_alias_bytes(
        &mut self,
        alias: PtrConstChar,
        value: ConstVoidPtr,
        size: usize,
    ) -> obx_err {
        obx_query_param_alias_bytes(self.obx_query, alias, value, size)
    }

    pub(crate) unsafe fn param_alias_get_type_size(
        &mut self,
        alias: PtrConstChar,
    ) -> usize {
        obx_query_param_alias_get_type_size(self.obx_query, alias)
    }
}
