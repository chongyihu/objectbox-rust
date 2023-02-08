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
// For lack of variadic args on .query(), use query(vec!(condition...));
*/

// TODO write macro for boilerplate: fn obx_query_something(...) -> obx_err, rewrite to call,
// TODO also error check before chaining the next call
// TODO depending on property type, allow only certain calls at compile time?
// TODO compile time determined extension blanket traits?

use crate::c;
use crate::c::*;
use crate::error;
use crate::traits::EntityFactoryExt;
use crate::traits::OBBlanket;
use crate::util::test_fn_ptr_on_char_ptr;
use std::marker::PhantomData;
use std::ptr;
use std::rc::Rc;

// TODO pass generic type from box, via fn
impl<T: OBBlanket> Drop for Query<T> {
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

pub struct Query<T: OBBlanket> {
    error: Option<error::Error>,
    obx_query: *mut OBX_query,
    obx_store: *mut OBX_store,
    pub(crate) helper: Rc<dyn EntityFactoryExt<T>>,
    phantom_data: PhantomData<T>,
}

impl<T: OBBlanket> Query<T> {
    pub(crate) fn new(
        obx_store: *mut OBX_store,
        helper: Rc<dyn EntityFactoryExt<T>>,
        builder: *mut OBX_query_builder,
    ) -> error::Result<Self> {
        unsafe {
            let obx_query = obx_query(builder);
            if let Err(err) = c::new_mut(obx_query, "Query::new".to_string()) {
                err.as_result()?;
            }
            Ok(Query {
                error: None,
                obx_query,
                obx_store,
                helper: helper.clone(),
                phantom_data: PhantomData,
            })
        }
    }

    fn close(&mut self) -> obx_err {
        unsafe { obx_query_close(self.obx_query) }
    }

    // No Clone trait here, because that implies Copy,
    // which prevents using Drop
    pub fn clone(&self) -> error::Result<Self> {
        unsafe {
            let clone = obx_query_clone(self.obx_query);
            if let Err(err) = c::new_mut(clone, "Query::clone".to_string()) {
                err.as_result()?;
            }

            // if they are the same, a double free will occur
            // otherwise, the same drop semantics will apply
            assert_ne!(self.obx_query, clone);
            
            Ok(Query {
                error: None,
                obx_query: clone,
                obx_store: self.obx_store,
                helper: self.helper.clone(),
                phantom_data: PhantomData,
            })
        }
    }

    /// Paging related
    pub fn offset(&mut self, offset: usize) -> &Self {
        unsafe {
            let result = obx_query_offset(self.obx_query, offset);
            self.error = c::call(result, "Query::offset".to_string()).err();
        }
        self
    }

    /// Paging related
    pub fn offset_limit(&mut self, offset: usize, limit: usize) -> &Self {
        unsafe {
            let result = obx_query_offset_limit(self.obx_query, offset, limit);
            self.error = c::call(result, "Query::offset_limit".to_string()).err();
        }
        self
    }

    /// Paging related
    pub fn limit(&mut self, limit: usize) -> &Self {
        unsafe {
            let result = obx_query_limit(self.obx_query, limit);
            self.error = c::call(result, "Query::limit".to_string()).err();
        }
        self
    }

    // elect the cursor version
    // pub(crate) unsafe fn find(&mut self) -> *mut OBX_bytes_array {
    //     obx_query_find(self.obx_query)
    // }

    // WONTFIX: don't implement unless anyone asks for it
    // pub(crate) unsafe fn find_first(
    //     &mut self,
    //     data: MutConstVoidPtr,
    //     size: *mut usize,
    // ) -> obx_err {
    //     obx_query_find_first(self.obx_query, data, size)
    // }

    // WONTFIX: don't implement unless anyone asks for it
    // pub(crate) unsafe fn find_unique(
    //     &mut self,
    //     data: MutConstVoidPtr,
    //     size: *mut usize,
    // ) -> obx_err {
    //     obx_query_find_unique(self.obx_query, data, size)
    // }

    // elect the cursor version
    // pub(crate) unsafe fn visit(
    //     &mut self,
    //     visitor: obx_data_visitor,
    //     user_data: *mut ::std::os::raw::c_void,
    // ) -> obx_err {
    //     obx_query_visit(self.obx_query, visitor, user_data)
    // }

    // elect the cursor version
    // pub(crate) unsafe fn find_ids(&mut self) -> *mut OBX_id_array {
    //     obx_query_find_ids(self.obx_query)
    // }

    // elect the cursor version
    // pub(crate) unsafe fn count(&mut self, out_count: *mut u64) -> obx_err {
    //     obx_query_count(self.obx_query, out_count)
    // }

    // elect the cursor version
    // pub(crate) unsafe fn remove(&mut self, out_count: *mut u64) -> obx_err {
    //     obx_query_remove(self.obx_query, out_count)
    // }

    /// For testing and debugging
    /// A function pointer is passed here, to prevent dealing with lifetime issues.
    pub fn describe(&mut self, fn_ptr: fn(String) -> bool) -> bool {
        unsafe {
            let out_ptr = obx_query_describe(self.obx_query);
            test_fn_ptr_on_char_ptr(out_ptr, fn_ptr)
        }
    }

    /// For testing and debugging
    /// A function pointer is passed here, to prevent dealing with lifetime issues.
    pub fn describe_params(&mut self, fn_ptr: fn(String) -> bool) -> bool {
        unsafe {
            let out_ptr = obx_query_describe_params(self.obx_query);
            test_fn_ptr_on_char_ptr(out_ptr, fn_ptr)
        }
    }

    // TODO create tx and cursor boilerplate macro, which requires a ptr to store -> tx -> cursor -> close on read, success on write
    // TODO pass a closure fn to this in the pub fn impl
    unsafe fn cursor_visit(
        &mut self,
        cursor: &mut OBX_cursor,
        visitor: obx_data_visitor,
        user_data: *mut ::std::os::raw::c_void,
    ) -> obx_err {
        obx_query_cursor_visit(self.obx_query, cursor, visitor, user_data)
    }

    unsafe fn cursor_find(&mut self, cursor: &mut OBX_cursor) -> *mut OBX_bytes_array {
        obx_query_cursor_find(self.obx_query, cursor)
    }

    unsafe fn cursor_find_ids(&mut self, cursor: &mut OBX_cursor) -> *mut OBX_id_array {
        obx_query_cursor_find_ids(self.obx_query, cursor)
    }

    unsafe fn cursor_count(&mut self, cursor: &mut OBX_cursor, out_count: *mut u64) -> obx_err {
        obx_query_cursor_count(self.obx_query, cursor, out_count)
    }

    unsafe fn cursor_remove(&mut self, cursor: &mut OBX_cursor, out_count: *mut u64) -> obx_err {
        obx_query_cursor_remove(self.obx_query, cursor, out_count)
    }
    // end cursor

    // TODO implement later
    // start aliasing
    /*
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
    */
}
