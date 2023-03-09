#![allow(dead_code)]
use crate::{c, error};

// TODO required for putAsync and putQueued
pub(crate) struct Async {
    pub(crate) obx_async: *mut c::OBX_async,
    ptr_closed: bool,
}

impl Drop for Async {
    fn drop(&mut self) {
        if !self.ptr_closed && !self.obx_async.is_null() {
            if let Err(e) = self.close() {
                eprint!("{e}");
            }
            self.obx_async = std::ptr::null_mut();
        }
    }
}

impl Async {
    // TODO test
    pub fn from_box(obx_box: *mut c::OBX_box) -> error::Result<Self> {
        unsafe {
            c::new_mut(c::obx_async(obx_box)).map(|ptr| Async {
                obx_async: ptr,
                ptr_closed: false,
            })
        }
    }

    // TODO test
    pub(crate) fn remove_with_id(&mut self, id: c::obx_id) -> error::Result<bool> {
        unsafe {
            let code = c::obx_async_remove(self.obx_async, id);
            c::call(code).map(|_| code == 0)
        }
    }

    // TODO test
    pub(crate) fn from_box_with_timeout(
        obx_box: *mut c::OBX_box,
        enqueue_timeout_millis: u64,
    ) -> error::Result<Self> {
        unsafe {
            c::new_mut(c::obx_async_create(obx_box, enqueue_timeout_millis)).map(|ptr| Async {
                obx_async: ptr,
                ptr_closed: false,
            })
        }
    }

    // TODO test
    pub(crate) fn close(&mut self) -> error::Result<()> {
        c::call(unsafe { c::obx_async_close(self.obx_async) })
    }

    // TODO finish
    /*
    pub(crate) fn put5(&mut self, id: c::obx_id, data: ConstVoidPtr, size: usize, mode: c::OBXPutMode) -> error::Result<c::obx_id> {
      // TODO depending on the state of the object determine the mode (PUT, INSERT, UPDATE)
      // TODO if this is a fresh new object, get an id
      c::call(unsafe { c::obx_async_put5(self.obx_async, id, data, size, mode) }).err();
      // TODO return this error::Result<id>
    }
    */

    /*
    // fn put(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
    //     unsafe { obx_async_put(async_, id, data, size) }
    // }

    // fn insert(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
    //     unsafe { obx_async_insert(async_, id, data, size) }
    // }

    // fn update(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
    //     unsafe { obx_async_update(async_, id, data, size) }
    // }

    // fn put_object(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize) -> obx_id {
    //     unsafe { obx_async_put_object(async_, data, size) }
    // }

    // fn put_object4(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize, mode: OBXPutMode) -> obx_id {
    //     unsafe { obx_async_put_object4(async_, data, size, mode) }
    // }

    // fn insert_object(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize) -> obx_id {
    //     unsafe { obx_async_insert_object(async_, data, size) }
    // }
    */
}
