use crate::c::*;
use crate::error::Error;
pub struct Box<T> {
  _place_holder: Option<T>,
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