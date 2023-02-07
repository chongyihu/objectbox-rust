// TODO write macro for boilerplate: fn obx_query_something(...) -> obx_err, rewrite to get_result,
// TODO use rusty result operators (or, or_else, ? etc.) to chain results
// TODO also error check before chaining the next call (obx_qb_cond)
// TODO depending on property type, allow only certain calls at compile time?
// TODO compile time determined extension blanket traits?
// TODO also use traits for operator overloading on specific functions (can't use blankets tho)
use crate::{
    c::*,
    error,
    store::Store,
    traits::FactoryHelper, util::PtrConstChar,
};

use std::rc::Rc;
use super::query::Query;

impl Drop for Builder {
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

pub struct Builder {
    error: Option<error::Error>,
    pub(crate) obx_query_builder: *mut OBX_query_builder,
    // TODO refresh memory: when transformed to query it's closed?
    // TODO so set flag when passing this object to a Query
    // builder.do_stuff().build() <- here to prevent drop (double free)
    has_built_query: bool,
}

impl Builder {
    pub fn from_store_and_entity_id<T>(store: &Store, factory: Rc<dyn FactoryHelper<T>>) -> Self {
        let entity_id = factory.get_entity_id(); // call factory
        let obx_query_builder = unsafe { obx_query_builder(store.obx_store, entity_id) };
        Builder {
            obx_query_builder,
            error: None,
            has_built_query: false,
        }
    }

    pub fn build(&mut self) -> error::Result<Query> {
        if let Some(err) = &self.error {
            Err(err.clone())
        } else {
            self.has_built_query = true;
            Ok(Query::from_query_builder(self.obx_query_builder))
        }
    }

    /// private, in case of double frees
    fn close(&mut self) -> obx_err {
        unsafe { obx_qb_close(self.obx_query_builder) }
    }

    pub(crate) fn type_id(&self) -> obx_schema_id {
        unsafe { obx_qb_type_id(self.obx_query_builder) }
    }

    fn error_code(&self) -> obx_err {
        unsafe { obx_qb_error_code(self.obx_query_builder) }
    }

    fn error_message(&self) -> PtrConstChar {
        unsafe { obx_qb_error_message(self.obx_query_builder) }
    }

    // TODO this should be implemented when Option<OB/FB Primitive> properties are supported
    pub(crate) unsafe fn is_null(&mut self, property_id: obx_schema_id) -> obx_qb_cond {
        obx_qb_null(self.obx_query_builder, property_id)
    }

    // TODO this should be implemented when Option<OB/FB Primitive> properties are supported
    pub(crate) unsafe fn not_null(&mut self, property_id: obx_schema_id) -> obx_qb_cond {
        obx_qb_not_null(self.obx_query_builder, property_id)
    }

    // TODO create macro for property_id boilerplate
    // TODO this belongs to trait PartialEq
    pub(crate) unsafe fn equals_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        obx_qb_equals_string(self.obx_query_builder, property_id, value, case_sensitive)
    }

    pub(crate) unsafe fn not_equals_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_not_equals_string(self.obx_query_builder, property_id, value, case_sensitive)
        }
    }

    pub(crate) unsafe fn contains_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_string(self.obx_query_builder, property_id, value, case_sensitive)
        }
    }

    pub(crate) unsafe fn contains_element_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_element_string(
                self.obx_query_builder,
                property_id,
                value,
                case_sensitive,
            )
        }
    }

    pub(crate) unsafe fn contains_key_value_string(
        &mut self,
        property_id: obx_schema_id,
        key: PtrConstChar,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_key_value_string(
                self.obx_query_builder,
                property_id,
                key,
                value,
                case_sensitive,
            )
        }
    }

    pub(crate) unsafe fn starts_with_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_starts_with_string(self.obx_query_builder, property_id, value, case_sensitive)
        }
    }

    pub(crate) unsafe fn ends_with_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_ends_with_string(self.obx_query_builder, property_id, value, case_sensitive)
        }
    }

    // TODO this belongs to PartialOrd trait, gt >
    pub(crate) unsafe fn greater_than_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_greater_than_string(self.obx_query_builder, property_id, value, case_sensitive)
        }
    }

    // TODO this belongs to trait PartialEq, ge >=
    pub(crate) unsafe fn greater_or_equal_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_greater_or_equal_string(
                self.obx_query_builder,
                property_id,
                value,
                case_sensitive,
            )
        }
    }

    // TODO this belongs to trait PartialEq, < lt
    pub(crate) unsafe fn less_than_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_less_than_string(self.obx_query_builder, property_id, value, case_sensitive)
        }
    }

    // TODO this belongs to trait PartialEq, <= le
    pub(crate) unsafe fn less_or_equal_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_less_or_equal_string(self.obx_query_builder, property_id, value, case_sensitive)
        }
    }

    pub(crate) unsafe fn in_strings(
        &mut self,
        property_id: obx_schema_id,
        values: *const PtrConstChar,
        count: usize,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_in_strings(
                self.obx_query_builder,
                property_id,
                values,
                count,
                case_sensitive,
            )
        }
    }

    pub(crate) unsafe fn any_equals_string(
        &mut self,
        property_id: obx_schema_id,
        value: PtrConstChar,
        case_sensitive: bool,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_any_equals_string(self.obx_query_builder, property_id, value, case_sensitive)
        }
    }

    // TODO PartialEq
    pub(crate) unsafe fn equals_int(
        &mut self,
        property_id: obx_schema_id,
        value: i64,
    ) -> obx_qb_cond {
        obx_qb_equals_int(self.obx_query_builder, property_id, value)
    }

    // TODO PartialEq
    pub(crate) unsafe fn not_equals_int(
        &mut self,
        property_id: obx_schema_id,
        value: i64,
    ) -> obx_qb_cond {
        obx_qb_not_equals_int(self.obx_query_builder, property_id, value)
    }

    // TODO PartialOrd
    pub(crate) unsafe fn greater_than_int(
        &mut self,
        property_id: obx_schema_id,
        value: i64,
    ) -> obx_qb_cond {
        obx_qb_greater_than_int(self.obx_query_builder, property_id, value)
    }

    pub(crate) unsafe fn greater_or_equal_int(
        &mut self,
        property_id: obx_schema_id,
        value: i64,
    ) -> obx_qb_cond {
        obx_qb_greater_or_equal_int(self.obx_query_builder, property_id, value)
    }

    pub(crate) unsafe fn less_than_int(
        &mut self,
        property_id: obx_schema_id,
        value: i64,
    ) -> obx_qb_cond {
        obx_qb_less_than_int(self.obx_query_builder, property_id, value)
    }

    pub(crate) unsafe fn less_or_equal_int(
        &mut self,
        property_id: obx_schema_id,
        value: i64,
    ) -> obx_qb_cond {
        obx_qb_less_or_equal_int(self.obx_query_builder, property_id, value)
    }

    pub(crate) unsafe fn between_2ints(
        &mut self,
        property_id: obx_schema_id,
        value_a: i64,
        value_b: i64,
    ) -> obx_qb_cond {
        obx_qb_between_2ints(self.obx_query_builder, property_id, value_a, value_b)
    }

    pub(crate) unsafe fn in_int64s(
        &mut self,
        property_id: obx_schema_id,
        values: *const i64,
        count: usize,
    ) -> obx_qb_cond {
        obx_qb_in_int64s(self.obx_query_builder, property_id, values, count)
    }

    pub(crate) unsafe fn not_in_int64s(
        &mut self,
        property_id: obx_schema_id,
        values: *const i64,
        count: usize,
    ) -> obx_qb_cond {
        obx_qb_not_in_int64s(self.obx_query_builder, property_id, values, count)
    }

    pub(crate) unsafe fn in_int32s(
        &mut self,
        property_id: obx_schema_id,
        values: *const i32,
        count: usize,
    ) -> obx_qb_cond {
        obx_qb_in_int32s(self.obx_query_builder, property_id, values, count)
    }

    pub(crate) unsafe fn not_in_int32s(
        &self,
        property_id: obx_schema_id,
        values: *const i32,
        count: usize,
    ) -> obx_qb_cond {
        obx_qb_not_in_int32s(self.obx_query_builder, property_id, values, count)
    }

    // TODO this belongs to trait PartialEq, gt >
    pub(crate) unsafe fn greater_than_double(
        &self,
        property_id: obx_schema_id,
        value: f64,
    ) -> obx_qb_cond {
        obx_qb_greater_than_double(self.obx_query_builder, property_id, value)
    }

    // TODO this belongs to trait PartialEq, gt > etc.
    pub(crate) unsafe fn greater_or_equal_double(
        &self,
        property_id: obx_schema_id,
        value: f64,
    ) -> obx_qb_cond {
        obx_qb_greater_or_equal_double(self.obx_query_builder, property_id, value)
    }

    pub(crate) unsafe fn less_than_double(
        &self,
        property_id: obx_schema_id,
        value: f64,
    ) -> obx_qb_cond {
        obx_qb_less_than_double(self.obx_query_builder, property_id, value)
    }

    pub(crate) unsafe fn less_or_equal_double(
        &self,
        property_id: obx_schema_id,
        value: f64,
    ) -> obx_qb_cond {
        obx_qb_less_or_equal_double(self.obx_query_builder, property_id, value)
    }

    pub(crate) unsafe fn between_2doubles(
        &self,
        property_id: obx_schema_id,
        value_a: f64,
        value_b: f64,
    ) -> obx_qb_cond {
        obx_qb_between_2doubles(self.obx_query_builder, property_id, value_a, value_b)
    }

    pub(crate) unsafe fn equals_bytes(
        &self,
        property_id: obx_schema_id,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_equals_bytes(self.obx_query_builder, property_id, value, size)
    }

    pub(crate) unsafe fn greater_than_bytes(
        &self,
        property_id: obx_schema_id,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_greater_than_bytes(self.obx_query_builder, property_id, value, size)
    }

    pub(crate) unsafe fn greater_or_equal_bytes(
        &self,
        property_id: obx_schema_id,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_greater_or_equal_bytes(self.obx_query_builder, property_id, value, size)
    }

    pub(crate) unsafe fn less_than_bytes(
        &self,
        property_id: obx_schema_id,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_less_than_bytes(self.obx_query_builder, property_id, value, size)
    }

    pub(crate) unsafe fn less_or_equal_bytes(
        &self,
        property_id: obx_schema_id,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_less_or_equal_bytes(self.obx_query_builder, property_id, value, size)
    }

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

    pub(crate) unsafe fn order(&self, property_id: obx_schema_id, flags: OBXOrderFlags) -> obx_err {
        obx_qb_order(self.obx_query_builder, property_id, flags)
    }

    pub(crate) unsafe fn link_property(
        &self,
        property_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        obx_qb_link_property(self.obx_query_builder, property_id)
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
                begin_property_id,
                end_property_id,
            )
        }
    }
}
