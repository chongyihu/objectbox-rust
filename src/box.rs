use crate::c::*;
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
  // TODO this shouldn't own a store, and it should have lifetime guarantees
  store: Option<Store>,
  obx_box: Option<*mut OBX_box>,
  obx_async: Option<*mut OBX_async>,
}



/*
#[repr(C)]
pub struct OBX_box {
  _unused: [u8; 0],
}
*/

// TODO Rewrite the following functions in rust as an impl of Box

/*
extern "C" {
    pub fn obx_box_store(box_: *mut OBX_box) -> *mut OBX_store;

    pub fn obx_box_contains(box_: *mut OBX_box, id: obx_id, out_contains: *mut bool) -> obx_err;

    pub fn obx_box_contains_many(
        box_: *mut OBX_box,
        ids: *const OBX_id_array,
        out_contains: *mut bool,
    ) -> obx_err;

    pub fn obx_box_get(
        box_: *mut OBX_box,
        id: obx_id,
        data: *mut *const ::std::os::raw::c_void,
        size: *mut usize,
    ) -> obx_err;

    pub fn obx_box_get_many(box_: *mut OBX_box, ids: *const OBX_id_array) -> *mut OBX_bytes_array;

    pub fn obx_box_get_all(box_: *mut OBX_box) -> *mut OBX_bytes_array;

    pub fn obx_box_visit_many(
        box_: *mut OBX_box,
        ids: *const OBX_id_array,
        visitor: obx_data_visitor,
        user_data: *mut ::std::os::raw::c_void,
    ) -> obx_err;

    pub fn obx_box_visit_all(
        box_: *mut OBX_box,
        visitor: obx_data_visitor,
        user_data: *mut ::std::os::raw::c_void,
    ) -> obx_err;

    pub fn obx_box_id_for_put(box_: *mut OBX_box, id_or_zero: obx_id) -> obx_id;

    pub fn obx_box_ids_for_put(
        box_: *mut OBX_box,
        count: u64,
        out_first_id: *mut obx_id,
    ) -> obx_err;

    pub fn obx_box_put(
        box_: *mut OBX_box,
        id: obx_id,
        data: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_err;

    pub fn obx_box_insert(
        box_: *mut OBX_box,
        id: obx_id,
        data: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_err;

    pub fn obx_box_update(
        box_: *mut OBX_box,
        id: obx_id,
        data: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_err;

    pub fn obx_box_put5(
        box_: *mut OBX_box,
        id: obx_id,
        data: *const ::std::os::raw::c_void,
        size: usize,
        mode: OBXPutMode,
    ) -> obx_err;

    pub fn obx_box_put_object(
        box_: *mut OBX_box,
        data: *mut ::std::os::raw::c_void,
        size: usize,
    ) -> obx_id;

    pub fn obx_box_put_object4(
        box_: *mut OBX_box,
        data: *mut ::std::os::raw::c_void,
        size: usize,
        mode: OBXPutMode,
    ) -> obx_id;

    pub fn obx_box_put_many(
        box_: *mut OBX_box,
        objects: *const OBX_bytes_array,
        ids: *const obx_id,
        mode: OBXPutMode,
    ) -> obx_err;

    pub fn obx_box_put_many5(
        box_: *mut OBX_box,
        objects: *const OBX_bytes_array,
        ids: *const obx_id,
        mode: OBXPutMode,
        fail_on_id_failure: bool,
    ) -> obx_err;

    pub fn obx_box_remove(box_: *mut OBX_box, id: obx_id) -> obx_err;

    pub fn obx_box_remove_many(
        box_: *mut OBX_box,
        ids: *const OBX_id_array,
        out_count: *mut u64,
    ) -> obx_err;

    pub fn obx_box_remove_all(box_: *mut OBX_box, out_count: *mut u64) -> obx_err;

    pub fn obx_box_is_empty(box_: *mut OBX_box, out_is_empty: *mut bool) -> obx_err;

    pub fn obx_box_count(box_: *mut OBX_box, limit: u64, out_count: *mut u64) -> obx_err;

    pub fn obx_box_get_backlink_ids(
        box_: *mut OBX_box,
        property_id: obx_schema_id,
        id: obx_id,
    ) -> *mut OBX_id_array;

    pub fn obx_box_rel_put(
        box_: *mut OBX_box,
        relation_id: obx_schema_id,
        source_id: obx_id,
        target_id: obx_id,
    ) -> obx_err;

    pub fn obx_box_rel_remove(
        box_: *mut OBX_box,
        relation_id: obx_schema_id,
        source_id: obx_id,
        target_id: obx_id,
    ) -> obx_err;

    pub fn obx_box_rel_get_ids(
        box_: *mut OBX_box,
        relation_id: obx_schema_id,
        id: obx_id,
    ) -> *mut OBX_id_array;

    pub fn obx_box_rel_get_backlink_ids(
        box_: *mut OBX_box,
        relation_id: obx_schema_id,
        id: obx_id,
    ) -> *mut OBX_id_array;

    pub fn obx_box_ts_min_max(
        box_: *mut OBX_box,
        out_min_id: *mut obx_id,
        out_min_value: *mut i64,
        out_max_id: *mut obx_id,
        out_max_value: *mut i64,
    ) -> obx_err;

    pub fn obx_box_ts_min_max_range(
        box_: *mut OBX_box,
        range_begin: i64,
        range_end: i64,
        out_min_id: *mut obx_id,
        out_min_value: *mut i64,
        out_max_id: *mut obx_id,
        out_max_value: *mut i64,
    ) -> obx_err;

    extern "C" {
    pub fn obx_async(box_: *mut OBX_box) -> *mut OBX_async;

    pub fn obx_async_put(
        async_: *mut OBX_async,
        id: obx_id,
        data: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_err;

    pub fn obx_async_put5(
        async_: *mut OBX_async,
        id: obx_id,
        data: *const ::std::os::raw::c_void,
        size: usize,
        mode: OBXPutMode,
    ) -> obx_err;

    pub fn obx_async_insert(
        async_: *mut OBX_async,
        id: obx_id,
        data: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_err;

    pub fn obx_async_update(
        async_: *mut OBX_async,
        id: obx_id,
        data: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_err;

    pub fn obx_async_put_object(
        async_: *mut OBX_async,
        data: *mut ::std::os::raw::c_void,
        size: usize,
    ) -> obx_id;

    pub fn obx_async_put_object4(
        async_: *mut OBX_async,
        data: *mut ::std::os::raw::c_void,
        size: usize,
        mode: OBXPutMode,
    ) -> obx_id;

    pub fn obx_async_insert_object(
        async_: *mut OBX_async,
        data: *mut ::std::os::raw::c_void,
        size: usize,
    ) -> obx_id;

    pub fn obx_async_remove(async_: *mut OBX_async, id: obx_id) -> obx_err;

    pub fn obx_async_create(box_: *mut OBX_box, enqueue_timeout_millis: u64) -> *mut OBX_async;

    pub fn obx_async_close(async_: *mut OBX_async) -> obx_err;
}
*/

