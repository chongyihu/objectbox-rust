use std::{marker::PhantomData, rc::Rc};

// TODO write macro for boilerplate: fn obx_query_something(...) -> obx_err, rewrite to get_result,
// TODO use rusty result operators (or, or_else, ? etc.) to chain results
// TODO also error check before chaining the next call (obx_qb_cond)
// TODO depending on property type, allow only certain calls at compile time?
// TODO compile time determined extension blanket traits?
use crate::{
    c::*,
    error,
    r#box::Box,
    traits::{EntityFactoryExt, OBBlanket},
    util::PtrConstChar,
};

use super::query::Query;

impl<T: OBBlanket> Drop for Builder<T> {
    fn drop(&mut self) {
        if !self.has_built_query && !self.obx_query_builder.is_null() {
            self.close();
            self.obx_query_builder = std::ptr::null_mut();
        }

        if let Some(err) = &self.error {
            eprintln!("Error: async: {err}");
        }
    }
}

pub struct Builder<T: OBBlanket> {
    error: Option<error::Error>,
    obx_store: *mut OBX_store,
    helper: Rc<dyn EntityFactoryExt<T>>,
    property_id: obx_schema_id,
    obx_query_builder: *mut OBX_query_builder,
    has_built_query: bool,
    phantom_data: PhantomData<T>,
}

impl<T: OBBlanket> Builder<T> {
    pub fn new(box_store: &Box<T>, property_id: obx_schema_id) -> Self {
        let entity_id = box_store.helper.get_entity_id(); // call factory
        let obx_store = box_store.get_store();
        assert!(!obx_store.is_null());
        let obx_query_builder = unsafe { obx_query_builder(obx_store, entity_id) };
        Builder {
            error: None,
            obx_store,
            helper: box_store.helper.clone(),
            property_id,
            obx_query_builder,
            has_built_query: false,
            phantom_data: PhantomData,
        }
    }

    pub fn build(&mut self) -> error::Result<Query<T>> {
        if let Some(err) = &self.error {
            Err(err.clone())
        } else {
            let r = Query::new(self.obx_store, self.helper.clone(), self.obx_query_builder)?;
            // iff a query is built properly, then do not drop, else drop
            let query = get_result(self.error_code(), r)?;
            self.has_built_query = true;
            Ok(query)
        }
    }

    /// private, in case of double frees
    fn close(&mut self) -> obx_err {
        unsafe { obx_qb_close(self.obx_query_builder) }
    }

    pub(crate) fn type_id(&self) -> obx_schema_id {
        unsafe { obx_qb_type_id(self.obx_query_builder) }
    }

    // TODO call this before finalizing build
    fn error_code(&self) -> obx_err {
        unsafe { obx_qb_error_code(self.obx_query_builder) }
    }

    // TODO call this before finalizing build
    fn error_message(&self) -> PtrConstChar {
        unsafe { obx_qb_error_message(self.obx_query_builder) }
    }

    // TODO this should be implemented when Option<OB/FB Primitive> properties are supported
    /*
    pub(crate) unsafe fn is_null(&mut self) -> obx_qb_cond {
        obx_qb_null(self.obx_query_builder, self.property_id)
    }

    // TODO this should be implemented when Option<OB/FB Primitive> properties are supported
    pub(crate) unsafe fn not_null(&mut self) -> obx_qb_cond {
        obx_qb_not_null(self.obx_query_builder, self.property_id)
    }
    */

    // eq_String
    pub(crate) unsafe fn equals_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        obx_qb_equals_string(
            self.obx_query_builder,
            self.property_id,
            value,
            case_sensitive,
        )
    }

    // ne_String
    pub(crate) unsafe fn not_equals_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_not_equals_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // contains_String
    pub(crate) unsafe fn contains_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // contains_element_String
    pub(crate) unsafe fn contains_element_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_element_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // contains_key_value_String
    pub(crate) unsafe fn contains_key_value_string(
        &mut self,
        key: PtrConstChar,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_key_value_string(
                self.obx_query_builder,
                self.property_id,
                key,
                value,
                case_sensitive,
            )
        }
    }

    // starts_with_String
    pub(crate) unsafe fn starts_with_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_starts_with_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // ends_with_String
    pub(crate) unsafe fn ends_with_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_ends_with_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // gt_String
    pub(crate) unsafe fn greater_than_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_greater_than_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // ge_String
    pub(crate) unsafe fn greater_or_equal_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_greater_or_equal_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // lt String
    pub(crate) unsafe fn less_than_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_less_than_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // le_String
    pub(crate) unsafe fn less_or_equal_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_less_or_equal_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // member_of_Strings / in_Strings
    pub(crate) unsafe fn in_strings(
        &mut self,
        values: *const PtrConstChar,
        count: usize,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_in_strings(
                self.obx_query_builder,
                self.property_id,
                values,
                count,
                case_sensitive,
            )
        }
    }

    // any_equals_String
    pub(crate) unsafe fn any_equals_string(
        &mut self,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_any_equals_string(
                self.obx_query_builder,
                self.property_id,
                value,
                case_sensitive,
            )
        }
    }

    // Eq (u8, i8, u16, i16, u32, i32, u64, i64)
    pub(crate) unsafe fn equals_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_equals_int(self.obx_query_builder, self.property_id, value)
    }

    // Ne (u8, i8, u16, i16, u32, i32, u64, i64)
    pub(crate) unsafe fn not_equals_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_not_equals_int(self.obx_query_builder, self.property_id, value)
    }

    // Gt (u8, i8, u16, i16, u32, i32, u64, i64)
    pub(crate) unsafe fn greater_than_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_greater_than_int(self.obx_query_builder, self.property_id, value)
    }

    // Ge (u8, i8, u16, i16, u32, i32, u64, i64)
    pub(crate) unsafe fn greater_or_equal_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_greater_or_equal_int(self.obx_query_builder, self.property_id, value)
    }

    // Lt (u8, i8, u16, i16, u32, i32, u64, i64)
    pub(crate) unsafe fn less_than_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_less_than_int(self.obx_query_builder, self.property_id, value)
    }

    // Le (u8, i8, u16, i16, u32, i32, u64, i64)
    pub(crate) unsafe fn less_or_equal_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_less_or_equal_int(self.obx_query_builder, self.property_id, value)
    }

    // between (u8, i8, u16, i16, u32, i32, u64, i64)
    pub(crate) unsafe fn between_2ints(&mut self, value_a: i64, value_b: i64) -> obx_qb_cond {
        obx_qb_between_2ints(self.obx_query_builder, self.property_id, value_a, value_b)
    }

    // in / member of (i64, u64?)
    pub(crate) unsafe fn in_int64s(&mut self, values: *const i64, count: usize) -> obx_qb_cond {
        obx_qb_in_int64s(self.obx_query_builder, self.property_id, values, count)
    }

    // not in / not member of (i64, u64?)
    pub(crate) unsafe fn not_in_int64s(&mut self, values: *const i64, count: usize) -> obx_qb_cond {
        obx_qb_not_in_int64s(self.obx_query_builder, self.property_id, values, count)
    }

    // in / member of (i32, u32?)
    pub(crate) unsafe fn in_int32s(&mut self, values: *const i32, count: usize) -> obx_qb_cond {
        obx_qb_in_int32s(self.obx_query_builder, self.property_id, values, count)
    }

    // not in / not member of (i32, u32?)
    pub(crate) unsafe fn not_in_int32s(&self, values: *const i32, count: usize) -> obx_qb_cond {
        obx_qb_not_in_int32s(self.obx_query_builder, self.property_id, values, count)
    }

    // gt f64
    pub(crate) unsafe fn greater_than_double(&self, value: f64) -> obx_qb_cond {
        obx_qb_greater_than_double(self.obx_query_builder, self.property_id, value)
    }

    // ge f64
    pub(crate) unsafe fn greater_or_equal_double(&self, value: f64) -> obx_qb_cond {
        obx_qb_greater_or_equal_double(self.obx_query_builder, self.property_id, value)
    }

    // lt f64
    pub(crate) unsafe fn less_than_double(&self, value: f64) -> obx_qb_cond {
        obx_qb_less_than_double(self.obx_query_builder, self.property_id, value)
    }

    // le f64
    pub(crate) unsafe fn less_or_equal_double(&self, value: f64) -> obx_qb_cond {
        obx_qb_less_or_equal_double(self.obx_query_builder, self.property_id, value)
    }

    // between f64
    pub(crate) unsafe fn between_2doubles(&self, value_a: f64, value_b: f64) -> obx_qb_cond {
        obx_qb_between_2doubles(self.obx_query_builder, self.property_id, value_a, value_b)
    }

    // eq Vec<u8>
    pub(crate) unsafe fn equals_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_equals_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // gt Vec<u8>
    pub(crate) unsafe fn greater_than_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_greater_than_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // ge Vec<u8>
    pub(crate) unsafe fn greater_or_equal_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_greater_or_equal_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // lt Vec<u8>
    pub(crate) unsafe fn less_than_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_less_than_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // le Vec<u8>
    pub(crate) unsafe fn less_or_equal_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_less_or_equal_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // TODO create all!() macro, substitute varargs
    pub(crate) unsafe fn all(&self, conditions: *const obx_qb_cond, count: usize) -> obx_qb_cond {
        obx_qb_all(self.obx_query_builder, conditions, count)
    }

    // TODO create any!() macro, substitute varargs
    pub(crate) unsafe fn any(&self, conditions: *const obx_qb_cond, count: usize) -> obx_qb_cond {
        obx_qb_any(self.obx_query_builder, conditions, count)
    }

    pub(crate) unsafe fn param_alias(&self, alias: PtrConstChar) -> obx_err {
        obx_qb_param_alias(self.obx_query_builder, alias)
    }

    pub(crate) unsafe fn order(&self, flags: OBXOrderFlags) -> obx_err {
        obx_qb_order(self.obx_query_builder, self.property_id, flags)
    }

    // TODO support later
    /*
    pub(crate) unsafe fn relation_count_property(
        &self,
        relation_entity_id: obx_schema_id,
        relation_property_id: obx_schema_id,
        relation_count: i32,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_relation_count_property(
                self.obx_query_builder,
                relation_entity_id,
                relation_property_id,
                relation_count,
            )
        }
    }
    pub(crate) unsafe fn link_property(
        &self,
    ) -> *mut OBX_query_builder {
        obx_qb_link_property(self.obx_query_builder, self.property_id)
    }

    pub(crate) unsafe fn backlink_property(
        &self,
        source_entity_id: obx_schema_id,
        source_property_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        unsafe {
            obx_qb_backlink_property(self.obx_query_builder, source_entity_id, source_property_id)
        }
    }

    pub(crate) unsafe fn link_standalone(
        &self,
        relation_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        obx_qb_link_standalone(self.obx_query_builder, relation_id)
    }

    pub(crate) unsafe fn backlink_standalone(
        &self,
        relation_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        obx_qb_backlink_standalone(self.obx_query_builder, relation_id)
    }

    pub(crate) unsafe fn link_time(
        &self,
        linked_entity_id: obx_schema_id,
        begin_property_id: obx_schema_id,
        end_property_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        unsafe {
            obx_qb_link_time(
                self.obx_query_builder,
                linked_entity_id,
                begin_self.property_id,
                end_self.property_id,
            )
        }
    }
    */
}
