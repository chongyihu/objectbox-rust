use crate::{c, error, util::ConstVoidPtr};

// TODO required for putAsync and putQueued
pub(crate) struct Async {
    pub(crate) obx_async: *mut c::OBX_async,
    pub(crate) error: Option<error::Error>,
    ptr_closed: bool,
}

impl Drop for Async {
    fn drop(&mut self) {
        if !self.ptr_closed && !self.obx_async.is_null() {
            self.close();
            self.obx_async = std::ptr::null_mut();
        }

        if let Some(err) = &self.error {
            eprintln!("Error: async: {err}");
        }
    }
}

impl Async {
    // TODO test
    pub fn from_box(obx_box: *mut c::OBX_box) -> error::Result<Self> {
        unsafe {
            let r = c::new_mut(c::obx_async(obx_box), "Async::from_box".to_string());

            match r {
                Ok(ptr) => Ok(Async {
                    obx_async: ptr,
                    error: None,
                    ptr_closed: false,
                }),
                Err(err) => Err(err.clone()),
            }
        }
    }

    // TODO test
    pub(crate) fn remove_with_id(&mut self, id: c::obx_id) -> error::Result<bool> {
        unsafe {
            let code = c::obx_async_remove(self.obx_async, id);
            self.error = c::call(code, "Async::error".to_string()).err();
            if let Some(err) = &self.error {
                Err(err.clone())
            } else {
                Ok(code == 0) // else: NOT_FOUND_404
            }
        }
    }

    // TODO test
    pub(crate) fn from_box_with_timeout(
        obx_box: *mut c::OBX_box,
        enqueue_timeout_millis: u64,
    ) -> error::Result<Self> {
        unsafe {
            let r = c::new_mut(
                c::obx_async_create(obx_box, enqueue_timeout_millis),
                "Async::from_box_with_timeout".to_string(),
            );

            match r {
                Ok(ptr) => Ok(Async {
                    obx_async: ptr,
                    error: None,
                    ptr_closed: false,
                }),
                Err(err) => Err(err.clone()),
            }
        }
    }

    // TODO test
    pub(crate) fn close(&mut self) {
        self.error = c::call(
            unsafe { c::obx_async_close(self.obx_async) },
            "Async::close".to_string(),
        )
        .err();
    }

    // TODO finish
    /*
    pub(crate) fn put5(&mut self, id: c::obx_id, data: ConstVoidPtr, size: usize, mode: c::OBXPutMode) -> error::Result<c::obx_id> {
      // TODO depending on the state of the object determine the mode (PUT, INSERT, UPDATE)
      // TODO if this is a fresh new object, get an id
      self.error = c::call(unsafe { c::obx_async_put5(self.obx_async, id, data, size, mode) }, "Async::put5".to_string()).err();
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
