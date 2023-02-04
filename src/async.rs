// TODO in lib pub mod r#async (because it's a reserved keyword)

/*
// TODO required for putAsync and putQueued
struct Async {
  obx_async: *mut OBX_async,
  ptr_closed: bool,
  error: Option<Error>,
}

impl Drop for Async {
  fn drop(&mut self) {
    unsafe {
      if !self.ptr_closed && !self.obx_async.is_null() {
        self.error = c::call(c::obx_async_close(self.obx_async)).err();
        self.obx_async = std::ptr::null_mut();
      }

      if let Some(err) = &self.error {
        eprintln!("Error: async: {err}");
      }
    }
  }
}

impl Async {
  // TODO create async wrapper
  // reserved keyword
  // pub fn async_(&mut self) -> *mut OBX_async {
  //   unsafe {
  //       obx_async(self.obx_box)
  //   }
  // }

  // TODO fix later
  // pub fn async_remove(&mut self, id: obx_id) -> obx_err {
  //   unsafe {
  //       obx_async_remove(self.obx_async, id)
  //   }
  // }

  // pub fn async_create(&mut self, enqueue_timeout_millis: u64) -> *mut OBX_async {
  //   unsafe {
  //       obx_async_create(self.obx_async, enqueue_timeout_millis)
  //   }
  // }

  // pub fn async_close(&mut self) -> obx_err {
  //   unsafe {
  //       obx_async_close(self.obx_async)
  //   }
  // }


  // TODO put in its own Type, with its own Drop
  // fn async_put(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
  //     unsafe { obx_async_put(async_, id, data, size) }
  // }

  // fn async_put5(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize, mode: OBXPutMode) -> obx_err {
  //     unsafe { obx_async_put5(async_, id, data, size, mode) }
  // }

  // fn async_insert(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
  //     unsafe { obx_async_insert(async_, id, data, size) }
  // }

  // fn async_update(&self, async_: *mut OBX_async, id: obx_id, data: *const ::std::os::raw::c_void, size: usize) -> obx_err {
  //     unsafe { obx_async_update(async_, id, data, size) }
  // }

  // fn async_put_object(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize) -> obx_id {
  //     unsafe { obx_async_put_object(async_, data, size) }
  // }

  // fn async_put_object4(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize, mode: OBXPutMode) -> obx_id {
  //     unsafe { obx_async_put_object4(async_, data, size, mode) }
  // }

  // fn async_insert_object(&self, async_: *mut OBX_async, data: *mut ::std::os::raw::c_void, size: usize) -> obx_id {
  //     unsafe { obx_async_insert_object(async_, data, size) }
  // }

  // TODO fix sooner than later
  // fn visit_many(&mut self, ids: &[c::obx_id], visitor: obx_data_visitor, user_data: *mut ::std::os::raw::c_void) -> obx_err {
  //   unsafe {
  //       obx_box_visit_many(self.obx_box, ids.as_ptr(), visitor, user_data)
  //   }
  // }

}
*/
